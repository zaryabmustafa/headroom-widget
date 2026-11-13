# Headroom — dev plan

Windows 11 taskbar widget: Claude Code + Codex usage limits, plus RAM.
Fork of `AbdoSarmini/claude-usage-widget-for-windows-taskbar` v1.2.0 (MIT, Rust, Win32 only).

Target look:

```
✳ 5h 24% | W 82% |  ֎ 5h 12% | W 40% |  ▭ 61% |      (֎ = OpenAI mark, ▭ = RAM-stick icon)
```

---

## 1. Status (2026-11-13)

| Item | State |
|---|---|
| Upstream imported | Done — commit `8ce153f` |
| Codex provider, RAM, section layout, rename, self-update off | Written — commit `9647666` |
| Phase A (Windows build) | **Done 2026-11-13** — rustc 1.100 + VS 2022 Build Tools; release build clean with no code changes, `cargo test` 9/9, clippy 0 errors / 5 style warnings |
| Phase B (taskbar verification) | **Done 2026-11-13** — all checks pass: light/dark, 125%/150% width, sleep/resume refetch, Show → Codex/RAM toggles; plus renders on taskbar; RAM 85% vs 84.4% OS ✓; Codex `W 100%`, no 5h, matches session log (weekly-only `prolite`) ✓, served live from `wham/usage` (no 403) ✓; Claude matches `/usage` ✓; Explorer restart ✓ (fixed duplicate widget — TaskbarCreated handler orphaned the one T_LAYOUT had just built). |

---

## 2. Decisions (settled — don't relitigate)

| Decision | Why |
|---|---|
| Fork upstream, don't rewrite | Taskbar embedding, rendering, Claude OAuth refresh already solved |
| Name: **Headroom**; no "Claude"/"Codex" in product name | Trademarks; provider names only in descriptions |
| CPU temperature: **dropped** (re-checked 2026-11-13) | Needs a kernel driver (PawnIO) + admin service. On this Legion, non-admin: ACPI/thermal-zone counters empty; Lenovo `LENOVO_GAMEZONE_DATA.GetCPUTemp` → access denied. Only driverless route: read HWiNFO's Gadget values (`HKCU\Software\HWiNFO64\VSB`) while HWiNFO runs — parked |
| Codex: **read-only**, no token refresh (v1) | Access token ~1h; refreshing could rotate the refresh token and sign Codex CLI out |
| Codex fallback: last `rate_limits` event in session logs, shown dimmed | Numbers only move while Codex is in use anyway |
| Label windows by **length**, not position | OpenAI has switched the 5h window off before |
| Self-update **removed** (2026-11-13) | Upstream updater would replace Headroom with the original exe; it was dead code, so `update.rs` is deleted |
| Codex mark = OpenAI logo (changed 2026-11-13, owner's call; was `>_`) | Recognizable at a glance. Mask `assets/codex_mark.a8` = 96×96 alpha rasterized from Simple Icons `openai.svg` (simple-icons@11.0.0) via WPF `Geometry.Parse`. Trademark: revisit before any public release |
| RAM mark = Lucide `memory-stick` icon (ISC), replaces the "RAM" label | Mask `assets/ram_mark.a8`, same pipeline (stroke 2, round caps). "Hide icon" brings the text label back |
| Single exe, no admin, no service | Keep upstream's deployment model |

---

## 3. Architecture

```
main.rs    Win32 windows, timers, menu, tooltip, settings, accounts
  ├─ Provider { Claude, Codex }  → dispatches fetch() / label()
  ├─ Account { id, dir, kind, usage, … }   (one per login folder)
  ├─ sample_ram()  every 2 s (T_RAM)
  └─ ensure_widget() → builds one render::Model per section
                       → render::render() each → render::compose() → UpdateLayeredWindow
api.rs     Claude: .credentials.json, OAuth refresh (Claude Code's lock protocol), /api/oauth/usage
codex.rs   Codex: auth.json, JWT exp/email decode, wham/usage, session-log fallback
render.rs  Model { mark: Mark, labels: [&str;3], fixed, … }; render(); compose(); draw_codex_mark()
http.rs    WinHTTP client (UA "headroom/0.1")
```

Shared data type: `api::Usage { five_hour, has_five_hour, weekly, fable, from_log }`.

### Endpoints

| Provider | Credentials | Request | Parsed |
|---|---|---|---|
| Claude | `%USERPROFILE%\.claude\.credentials.json` (`CLAUDE_CONFIG_DIR`) → `claudeAiOauth.accessToken` | `GET https://api.anthropic.com/api/oauth/usage` + `anthropic-beta: oauth-2025-04-20` | `five_hour`, `seven_day` `.utilization` / `.resets_at` |
| Codex | `%USERPROFILE%\.codex\auth.json` (`CODEX_HOME`) → `tokens.access_token`, `tokens.account_id` | `GET https://chatgpt.com/backend-api/wham/usage` + `ChatGPT-Account-Id`, `originator: codex_cli_rs`, codex-style UA | `rate_limit.{primary,secondary}_window` `.used_percent` / `.limit_window_seconds` / `.reset_at` |
| Codex fallback | `.codex\sessions\YYYY\MM\DD\*.jsonl` (newest 7 days) | none | last line containing `rate_limits`: `{primary,secondary}.used_percent`, `window_minutes`, `resets_at` or `resets_in_seconds` |

Both usage endpoints are undocumented. Each provider fails independently (`–` for that section only).

### Settings (`HKCU\Software\Headroom`)

Upstream DWORDs (IntervalSeconds, FontSize, IconSize, HideIcon, PixelIcon, Rings, RingText, RollNumbers, HideFiveHour, HideWeekly, HideFable, HideLabels, OnLeft, ShowRemaining, AccountsPerColumn) plus:

| Name | Type | Meaning |
|---|---|---|
| `Accounts` | REG_SZ, `\|`-separated | Claude config dirs (empty → default dir) |
| `CodexAccounts` | REG_SZ, `\|`-separated | Codex dirs. **Absent** → auto-add default if `auth.json` exists. **Present but empty** → none |
| `HideCodex` | DWORD | Hide Codex section |
| `HideRam` | DWORD | Hide RAM section |
| `HideFable` | DWORD | **Absent → hidden** (Headroom default; upstream showed it). `0` shows it |

Autostart: `HKCU\…\Run\Headroom`. Single instance: mutex `Local\Headroom.Singleton`.

---

## 4. Next steps

Machine: Lenovo Legion Pro 5i (i9-14900HX, 32 GB), Windows 11 LTSC 26100. Claude Code and Codex are both installed on **Windows** (not WSL).

### Phase A — get it compiling  ✓ done
1. `cargo build --release` in this folder.
2. Fix errors. Likely spots: closure borrows of `s` inside `ensure_widget`'s `with(|s| …)` block; `render::Model` field list vs construction in `main.rs`; `MEMORYSTATUSEX` import path.
3. `cargo test` — `api.rs` and `codex.rs` tests must pass.
4. `cargo clippy` clean of errors (warnings OK for now; `update.rs` may warn as dead code).

**Done when:** `target\release\headroom.exe` builds and tests pass.

### Phase B — verify on the taskbar  ✓ done
| Check | Pass condition |
|---|---|
| Claude section | Matches `/usage` in Claude Code and claude.ai/settings/usage |
| Codex section | Matches `/status` in Codex. Weekly-only plans show no `5h` |
| Codex stale path | Rename/expire token (or wait >1h idle): section dims, tooltip says "From Codex's logs" |
| RAM | Within ±1% of Task Manager → Memory |
| Light + dark taskbar | Both readable |
| Width | Doesn't overlap pinned icons or the tray chevron at 100% / 125% / 150% scaling |
| Hover pill | One continuous pill across all sections |
| Explorer restart | `taskkill /f /im explorer.exe; start explorer` (PowerShell) → widget comes back |
| Sleep / resume | Refetches within ~5 s of wake |
| Menu toggles | Show Codex / Show RAM resize the widget immediately |

**If `wham/usage` returns 403 from Cloudflare:** the UA/header set is the suspect. Compare with Codex CLI's own request headers (Codex CLI source: `codex-rs`), not guesses.

### Phase C — polish
- Codex mark: review at 16/18/22/26 px (box-filtered from the 96 px mask).
- Tooltip: group by provider when there are multiple accounts per provider.
- Left-click: open the usage page for the section under the cursor (needs per-section x ranges from `compose`).
- Optional: compact mode (`24·82 12·40 61%`) if width is tight.
- Optional: CPU load % in tooltip only (`GetSystemTimes`, no admin).

### Phase D — Codex token refresh (optional, gated)
Only after reading how Codex CLI refreshes `auth.json` (endpoint, client id, whether refresh tokens rotate, any file lock). Must not log Codex CLI out. If rotation + no lock → keep read-only.

### Phase E — packaging
- ~~Own icon~~ Done 2026-11-13: Headroom mark, see `docs/BRAND.md`.
- Release build in GitHub Actions (windows-latest, `cargo build --release`), attach `headroom.exe`.
- Optional: a self-updater for Headroom releases (upstream's `update.rs` was deleted; recover it from git history `9647666` if useful).
- README screenshots.

---

## 5. Constraints for whoever picks this up

- Don't refresh or rewrite Codex's `auth.json`.
- Don't bring back the upstream updater as-is.
- Don't add CPU temperature / kernel drivers.
- Keep upstream's MIT `LICENSE` and the attribution in README.
- No telemetry; only `api.anthropic.com`, `platform.claude.com`, `chatgpt.com` are contacted.
