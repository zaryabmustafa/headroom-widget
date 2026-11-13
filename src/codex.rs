//! Codex CLI usage: reads the ChatGPT login Codex CLI saves in `auth.json`
//! and asks ChatGPT's backend for the 5-hour and weekly limits.
//!
//! Read-only on purpose. Codex's access token lasts about an hour and its
//! refresh token may rotate, so refreshing it here could sign Codex CLI out.
//! When the token has expired (Codex not used for a while), the numbers come
//! from the last `rate_limits` event Codex wrote to its session logs instead,
//! and are marked stale.

use crate::api::{parse_iso8601, FetchError, Limit, Usage};
use crate::now;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const HOST: &str = "chatgpt.com";
const USAGE_PATH: &str = "/backend-api/wham/usage";
const USER_AGENT: &str = "codex_cli_rs/0.0.0 (Windows; x86_64) headroom";
/// Windows up to this long count as the short ("5h") window; longer ones as weekly.
const SHORT_WINDOW_MAX_SECS: i64 = 24 * 3600;
/// Session-log day folders to look through, newest first.
const LOG_DAYS_TO_SCAN: usize = 7;

/// `CODEX_HOME`, or `%USERPROFILE%\.codex`.
pub fn default_dir() -> PathBuf {
    if let Ok(d) = std::env::var("CODEX_HOME") {
        if !d.is_empty() {
            return PathBuf::from(d);
        }
    }
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".codex")
}

fn auth_path(dir: &Path) -> PathBuf {
    dir.join("auth.json")
}

pub fn has_login(dir: &Path) -> bool {
    auth_path(dir).is_file()
}

fn load_auth(dir: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(auth_path(dir)).ok()?).ok()
}

/// The account's email from the ID token, or "Codex".
pub fn account_label(dir: &Path) -> String {
    load_auth(dir)
        .and_then(|a| a["tokens"]["id_token"].as_str().and_then(jwt_payload))
        .and_then(|p| p["email"].as_str().map(str::to_string))
        .filter(|e| !e.is_empty())
        .unwrap_or_else(|| "Codex".into())
}

pub fn fetch(dir: &Path) -> Result<Usage, FetchError> {
    let auth = load_auth(dir)
        .ok_or_else(|| FetchError::Other("No Codex login found. Run `codex` and sign in.".into()))?;
    let tokens = &auth["tokens"];
    let Some(access) = tokens["access_token"].as_str() else {
        return Err(FetchError::Other(
            "Codex is signed in with an API key, which has no plan limits.".into(),
        ));
    };
    let account_id = tokens["account_id"].as_str();
    let expired = jwt_payload(access)
        .and_then(|p| p["exp"].as_i64())
        .is_some_and(|exp| exp <= now() + 30);

    // Expired or already refused: no request until Codex writes a new token.
    let live_err = if expired || crate::api::is_dead(access) {
        FetchError::Other("Codex login expired. Showing the last numbers from Codex's logs; run `codex` to refresh.".into())
    } else {
        match get_usage(access, account_id) {
            Ok(u) => return Ok(u),
            // Pass throttling up instead of hiding it behind the logs, so the
            // caller backs off rather than asking again at the normal pace.
            Err(FetchError::RateLimited) => return Err(FetchError::RateLimited),
            Err(e) => e,
        }
    };
    // Fall back to what Codex itself last saw.
    match from_logs(dir) {
        Some(u) => Ok(u),
        None => Err(live_err),
    }
}

fn get_usage(token: &str, account_id: Option<&str>) -> Result<Usage, FetchError> {
    let mut headers = format!(
        "Authorization: Bearer {token}\r\nAccept: application/json\r\nUser-Agent: {USER_AGENT}\r\noriginator: codex_cli_rs"
    );
    if let Some(id) = account_id {
        headers += &format!("\r\nChatGPT-Account-Id: {id}");
    }
    let resp = crate::api::send("GET", HOST, USAGE_PATH, &headers, None)?;
    match resp.status {
        200 => {}
        401 => {
            crate::api::mark_dead(token);
            return Err(FetchError::Other("Codex login rejected. Run `codex` to sign in again.".into()));
        }
        // 403 here is usually Cloudflare refusing the request: back off like a 429.
        403 | 429 => return Err(FetchError::RateLimited),
        s => return Err(FetchError::Other(format!("Codex usage request failed (HTTP {s})"))),
    }
    let v: Value = serde_json::from_slice(&resp.body)
        .map_err(|_| FetchError::Other("Bad Codex usage response".into()))?;
    Ok(parse_usage(&v))
}

/// `{rate_limit: {primary_window: {used_percent, limit_window_seconds, reset_at}, secondary_window: …}}`
fn parse_usage(v: &Value) -> Usage {
    let rl = &v["rate_limit"];
    let mut u = Usage::default();
    for (i, key) in ["primary_window", "secondary_window"].iter().enumerate() {
        let w = &rl[*key];
        let Some(percent) = w["used_percent"].as_f64() else { continue };
        let window = w["limit_window_seconds"].as_i64();
        let resets_at = time_field(&w["reset_at"])
            .or_else(|| w["reset_after_seconds"].as_i64().map(|s| now() + s));
        place(&mut u, i == 0, window, Limit { percent, resets_at });
    }
    u
}

/// Files a limit as 5-hour or weekly by its window length (OpenAI has turned
/// the 5-hour window off before, so position alone isn't trusted).
fn place(u: &mut Usage, primary: bool, window_secs: Option<i64>, l: Limit) {
    let short = window_secs.map_or(primary, |s| s <= SHORT_WINDOW_MAX_SECS);
    if short {
        u.five_hour = Some(l);
        u.has_five_hour = true;
    } else {
        u.weekly = Some(l);
    }
}

/// Unix seconds as a number, or an ISO 8601 string.
fn time_field(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)).or_else(|| v.as_str().and_then(parse_iso8601))
}

// ---------- session-log fallback ----------

/// The newest `rate_limits` snapshot in `<dir>\sessions\YYYY\MM\DD\*.jsonl`.
fn from_logs(dir: &Path) -> Option<Usage> {
    for day in newest_day_dirs(&dir.join("sessions"), LOG_DAYS_TO_SCAN) {
        let mut files: Vec<(SystemTime, PathBuf)> = fs::read_dir(&day)
            .ok()?
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "jsonl"))
            .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
            .collect();
        files.sort_by_key(|f| std::cmp::Reverse(f.0));
        for (_, f) in files {
            if let Some(u) = fs::read_to_string(&f).ok().and_then(|t| last_snapshot(&t)) {
                return Some(u);
            }
        }
    }
    None
}

/// Day folders, newest first (names are zero-padded numbers, so they sort as text).
fn newest_day_dirs(sessions: &Path, max: usize) -> Vec<PathBuf> {
    let sorted_dirs = |p: &Path| -> Vec<PathBuf> {
        let mut v: Vec<PathBuf> = fs::read_dir(p)
            .map(|r| r.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect())
            .unwrap_or_default();
        v.sort();
        v.reverse();
        v
    };
    let mut out = Vec::new();
    for y in sorted_dirs(sessions) {
        for m in sorted_dirs(&y) {
            for d in sorted_dirs(&m) {
                out.push(d);
                if out.len() >= max {
                    return out;
                }
            }
        }
    }
    out
}

fn last_snapshot(text: &str) -> Option<Usage> {
    text.lines()
        .rev()
        .filter(|l| l.contains("\"rate_limits\""))
        .find_map(|l| snapshot_from_line(&serde_json::from_str(l).ok()?))
}

/// A log line holding `rate_limits: {primary: {used_percent, window_minutes,
/// resets_at | resets_in_seconds}, secondary: …}` somewhere inside it.
fn snapshot_from_line(v: &Value) -> Option<Usage> {
    let rl = find_key(v, "rate_limits")?;
    let logged_at = v["timestamp"].as_str().and_then(parse_iso8601).unwrap_or_else(now);
    let mut u = Usage { from_log: true, ..Usage::default() };
    let mut any = false;
    for (i, key) in ["primary", "secondary"].iter().enumerate() {
        let w = &rl[*key];
        let Some(mut percent) = w["used_percent"].as_f64() else { continue };
        let window = w["window_minutes"].as_i64().map(|m| m * 60);
        let resets_at = time_field(&w["resets_at"])
            .or_else(|| w["resets_in_seconds"].as_i64().map(|s| logged_at + s));
        // The window has rolled over since Codex logged it: nothing used yet.
        if resets_at.is_some_and(|t| t <= now()) {
            percent = 0.0;
        }
        place(&mut u, i == 0, window, Limit { percent, resets_at });
        any = true;
    }
    any.then_some(u)
}

fn find_key<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    match v {
        Value::Object(map) => map
            .get(key)
            .filter(|x| x.is_object())
            .or_else(|| map.values().find_map(|x| find_key(x, key))),
        Value::Array(a) => a.iter().find_map(|x| find_key(x, key)),
        _ => None,
    }
}

// ---------- JWT ----------

/// Decodes a JWT's payload (no signature check: only reading our own login).
fn jwt_payload(token: &str) -> Option<Value> {
    let part = token.split('.').nth(1)?;
    serde_json::from_slice(&base64url_decode(part)?).ok()
}

fn base64url_decode(s: &str) -> Option<Vec<u8>> {
    let val = |c: u8| -> Option<u32> {
        let v: u8 = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'-' | b'+' => 62,
            b'_' | b'/' => 63,
            _ => return None,
        };
        Some(v as u32)
    };
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let (mut acc, mut bits) = (0u32, 0u32);
    for &c in s.as_bytes().iter().filter(|&&c| c != b'=') {
        acc = (acc << 6) | val(c)?;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn live_response() {
        let u = super::parse_usage(&json!({"rate_limit": {
            "primary_window": {"used_percent": 27, "limit_window_seconds": 18000, "reset_at": 1794406200},
            "secondary_window": {"used_percent": 40.5, "limit_window_seconds": 604800, "reset_at": 1794906200}
        }}));
        assert!(u.has_five_hour);
        assert_eq!(u.five_hour.unwrap().percent, 27.0);
        assert_eq!(u.weekly.unwrap().resets_at, Some(1794906200));
        assert!(!u.from_log);
    }

    #[test]
    fn weekly_only() {
        // 5-hour window switched off: the only window is weekly, even as "primary".
        let u = super::parse_usage(&json!({"rate_limit": {
            "primary_window": {"used_percent": 12, "limit_window_seconds": 604800}
        }}));
        assert!(!u.has_five_hour && u.five_hour.is_none());
        assert_eq!(u.weekly.unwrap().percent, 12.0);
    }

    #[test]
    fn log_line() {
        let line = json!({"timestamp": "2099-01-01T00:00:00Z", "type": "event_msg", "payload": {
            "type": "token_count",
            "rate_limits": {
                "primary": {"used_percent": 33.0, "window_minutes": 300, "resets_in_seconds": 600},
                "secondary": {"used_percent": 8.0, "window_minutes": 10080, "resets_in_seconds": 86400}
            }
        }});
        let u = super::snapshot_from_line(&line).unwrap();
        assert!(u.from_log);
        assert_eq!(u.five_hour.unwrap().percent, 33.0);
        assert_eq!(u.weekly.unwrap().percent, 8.0);
    }

    #[test]
    fn jwt() {
        // {"email":"a@b.c","exp":1}
        let t = "x.eyJlbWFpbCI6ImFAYi5jIiwiZXhwIjoxfQ.y";
        let p = super::jwt_payload(t).unwrap();
        assert_eq!(p["email"], "a@b.c");
        assert_eq!(p["exp"], 1);
    }

    /// A refused, unexpired token must not be sent again (no request at all).
    #[test]
    fn dead_token_not_resent() {
        let dir = std::env::temp_dir().join(format!("headroom-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // {"exp":4102444800} = year 2100, so only the dead-token check can stop the request.
        let token = "x.eyJleHAiOjQxMDI0NDQ4MDB9.y";
        std::fs::write(dir.join("auth.json"), json!({"tokens": {"access_token": token}}).to_string()).unwrap();
        crate::api::mark_dead(token);
        let msg = match super::fetch(&dir) {
            Err(e) => e.message(),
            Ok(_) => String::new(),
        };
        let _ = std::fs::remove_dir_all(&dir);
        // "expired" = skipped locally; a request would have come back "rejected".
        assert!(msg.contains("expired"), "{msg}");
    }
}
