# Headroom — dev plan

Windows 11 taskbar widget: Claude Code + Codex usage limits, plus RAM.
Fork of `AbdoSarmini/claude-usage-widget-for-windows-taskbar` v1.2.0 (MIT, Rust, Win32 only).
Public at <https://github.com/zaryabmustafa/headroom-widget>.

Current look (no dividers inside a section, one after each section):

```
✳ 5h 24%  W 82% |  ֎ 5h 12%  W 40% |  ▭ 61% |      (֎ = OpenAI mark, ▭ = RAM-stick icon)
```

---

## 1. Status (2026-11-13)

**Released: v0.1.1** (Latest). Phases A, B done; Phase E mostly done; C and D open.

| Item | State |
|---|---|
| Upstream imported | `8ce153f` |
| Codex provider, RAM, section layout, rename | `9647666` |
| Phase A: Windows build | Done. Rust 1.100 + VS 2022 Build Tools. Compiled first try with no code changes |
| Phase B: taskbar checks | Done, except two items not exercised (see Phase B table) |
| Fixes, features, brand, cleanup | `5fc26b9` (see §2) |
| v0.1.0 release | Built locally, published |
| Icon edge + v0.1.1 | `dea8257`, `3954603`; built locally, published |
| GitHub Actions release build | `82a5dd5`…`e693aa5`; dry run passes in ~1m15s, no warnings. Tag-triggered publish not yet exercised |
| Release docs | `d63b43e`: `docs/RELEASING.md` |
| Tests / lint | `cargo test` 7/7; clippy 0 errors, 5 style warnings |

---

## 2. Done on 2026-11-13

**Fixes**
- Explorer restart left **two widgets**, one stuck over the pinned icons. Cause: `T_LAYOUT` built a widget on the new taskbar before `TaskbarCreated` arrived, and that handler nulled `s.widget` without destroying it. Now it nulls `s.taskbar`, so `ensure_widget` destroys whatever is alive and recreates exactly one.

**Widget**
- Codex section shows the **OpenAI mark**, drawn at 80% of the icon box so it matches Claude's asterisk visually.
- RAM section shows a **RAM-stick icon** (Lucide `memory-stick`, ISC) in place of the "RAM" label. "Hide icon" brings the text back.
- Both icons come from one `render::draw_mask(mask, size, scale)`: 96×96 alpha masks in `assets/*.a8`, box-filtered to icon size.
- **No dividers inside a section** (`5h` and `W` are spaced, not divided); one divider ends each section.
- **Fable (`F`) hidden by default** (`HideFable` absent → hidden).
- **Menu regrouped:** Show ▸ (5-hour, Weekly, Fable · Codex, RAM) · Appearance ▸ · Accounts ▸ (incl. accounts per column) · Update every ▸ · Start with Windows · actions · Exit. Same IDs and handlers; tick = shown.

**Cleanup** (ponytail audit; source −592 / +95 lines)
- Deleted the dead upstream self-updater (`update.rs`, timer, messages, menu stubs, post-update mutex wait).
- Deleted the claude-swap (`cswap`) integration.
- Dropped `windows-sys` features `Win32_Security_Cryptography`, `Win32_System_Power`. (`Win32_UI_Shell_Common` is needed by the folder picker.)
- `codex.rs` uses `crate::now` instead of its own copy.

**Brand** (`docs/BRAND.md`)
- Mark: three meters under a ceiling bar; the gap is the headroom. 16-unit grid, `assets/logo.svg`.
- Palette "Ember": charcoal `#1C1917`, ceiling `#FAFAF9`, amber `#F59E0B`, edge `#57534E` (0.5-unit outline added in v0.1.1 so the tile reads on dark backgrounds).
- Tagline: *Know your limits before you hit them.*
- New `assets/icon.ico` (16–256, PNG entries) / `icon.png`; replaces upstream's orange icon.
- README rebuilt: centered logo, tagline, badges (Windows 11, Rust, no admin, MIT, downloads), taskbar screenshot (`assets/screenshot.png`, widget window only; no tooltip, no emails), sections for what it shows, install, menu, privacy, credits.

**Release**
- v0.1.0 and v0.1.1 published with `headroom.exe` + SHA-256 in the notes; downloaded copies verified against local hashes.
- `.github/workflows/release.yml`: `v*` tag → test, build, checksum, artifact, publish (creates the release or re-attaches the exe to an existing one). Manual run = dry run. `actions/checkout@v5`, `actions/upload-artifact@v6` (Node 24).
- `docs/RELEASING.md`: how to cut, dry-run, re-notes, and fix a release.

**Researched and parked:** CPU temperature (see §3).

---

## 3. Decisions (settled — don't relitigate)

| Decision | Why |
|---|---|
| Fork upstream, don't rewrite | Taskbar embedding, rendering, Claude OAuth refresh already solved |
| Name: **Headroom**; no "Claude"/"Codex" in product name | Trademarks; provider names only in descriptions |
| CPU temperature: **dropped** (re-checked 2026-11-13) | Needs a kernel driver (PawnIO) + admin service. On this Legion, non-admin: ACPI/thermal-zone counters empty; Lenovo `LENOVO_GAMEZONE_DATA.GetCPUTemp` → access denied. Only driverless route: read HWiNFO's Gadget values (`HKCU\Software\HWiNFO64\VSB`) while HWiNFO runs. Parked |
| Codex: **read-only**, no token refresh (v1) | Access token ~1h; refreshing could rotate the refresh token and sign Codex CLI out |
| Codex fallback: last `rate_limits` event in session logs, shown dimmed | Numbers only move while Codex is in use anyway |
| Label windows by **length**, not position | OpenAI has switched the 5h window off before |
| Self-update **removed** | Upstream updater would replace Headroom with the original exe; it was dead code |
| Codex mark = OpenAI logo (owner's call; was `>_`) | Recognizable at a glance. `assets/codex_mark.a8` from Simple Icons `openai.svg` (simple-icons@11.0.0) via WPF `Geometry.Parse`. **Now shipped publicly; trademark question still open** (see Phase C) |
| RAM mark = Lucide `memory-stick` (ISC) | `assets/ram_mark.a8`, same pipeline (stroke 2, round caps) |
| Fable limit hidden by default | Rarely relevant; one menu tick away |
| Brand palette "Ember" | Owner's pick over graphite+mint, traffic, aurora, ink, lime |
| Releases built by GitHub Actions from tags | Reproducible, not tied to one PC |
| Single exe, no admin, no service | Keep upstream's deployment model |

---

## 4. Architecture

```
main.rs    Win32 windows, timers, menu, tooltip, settings, accounts
  ├─ Provider { Claude, Codex }  → dispatches fetch() / label()
  ├─ Account { id, dir, kind, usage, … }   (one per login folder)
  ├─ sample_ram()  every 2 s (T_RAM)
  └─ ensure_widget() → builds one render::Model per section
                       → render::render() each → render::compose() → UpdateLayeredWindow
api.rs     Claude: .credentials.json, OAuth refresh (Claude Code's lock protocol), /api/oauth/usage
codex.rs   Codex: auth.json, JWT exp/email decode, wham/usage, session-log fallback
render.rs  Model { mark: Mark, labels, … }; render(); compose(); draw_mark() (Claude);
           draw_mask() for Codex / RAM from assets/*.a8
http.rs    WinHTTP client (UA "headroom/0.1")

assets/    icon.ico, icon.png, logo.svg (brand); codex_mark.a8, ram_mark.a8 (section marks);
           screenshot.png (README)
.github/workflows/release.yml   tag → build + publish
docs/      DEVPLAN.md (this), BRAND.md, RELEASING.md
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

## 5. Phases

Dev machine: Lenovo Legion Pro 5i (i9-14900HX, 32 GB), Windows 11 LTSC 26100. Claude Code and Codex both on **Windows** (not WSL).
Toolchain isn't on PATH in shells opened before the install: use `$env:USERPROFILE\.cargo\bin`.

### Phase A — get it compiling  ✓ done
Built clean on first try; no code changes were needed.

### Phase B — verify on the taskbar  ✓ done (2 items not exercised)
| Check | Result |
|---|---|
| Claude section | ✓ matches `/usage` |
| Codex section | ✓ `W 100%`, no `5h` (weekly-only `prolite` plan); live from `wham/usage`, no 403; matches session log |
| Codex stale path | **Not exercised** (token was live). Expire/rename the token or wait >1 h idle: section should dim, tooltip "From Codex's logs" |
| RAM | ✓ 85% vs 84.4% from the OS |
| Light + dark taskbar | ✓ |
| Width at 125% / 150% | ✓ no overlap |
| Hover pill | **Not explicitly checked** (tooltip shows; pill continuity across sections not confirmed) |
| Explorer restart (`taskkill /f /im explorer.exe; start explorer`) | ✓ after the duplicate-widget fix |
| Sleep / resume | ✓ refetches on wake |
| Menu toggles | ✓ Show → Codex / RAM resize immediately |

**If `wham/usage` returns 403 from Cloudflare:** the UA/header set is the suspect. Compare with Codex CLI's own request headers (`codex-rs`), not guesses.

### Phase C — polish  ← next
- **Trademark:** the OpenAI mark (and upstream's Claude asterisk) now ship in a public exe. Decide: keep, or switch Codex back to `>_` / a neutral mark.
- Finish the two Phase B items above.
- Clippy: 5 style warnings (3 auto-fixable with `cargo clippy --fix`).
- Tooltip: group by provider when there are multiple accounts per provider.
- Left-click: open the usage page for the section under the cursor (needs per-section x ranges from `compose`).
- Optional: compact mode (`24·82 12·40 61%`) if width is tight.
- Optional: CPU load % in tooltip only (`GetSystemTimes`, no admin).

### Phase D — Codex token refresh (optional, gated)
Only after reading how Codex CLI refreshes `auth.json` (endpoint, client id, whether refresh tokens rotate, any file lock). Must not log Codex CLI out. If rotation + no lock → keep read-only.

### Phase E — packaging  (mostly done)
- ✓ Own icon and brand (`docs/BRAND.md`).
- ✓ README with screenshot and badges.
- ✓ Release build in GitHub Actions (`docs/RELEASING.md`). First tag-triggered publish happens on the next release; watch it.
- Code signing: unsigned exe → SmartScreen warning on first run.
- Optional: a self-updater for Headroom releases (upstream's `update.rs` is in git history at `9647666`).

---

## 6. Constraints for whoever picks this up

- Don't refresh or rewrite Codex's `auth.json`.
- Don't bring back the upstream updater as-is.
- Don't add CPU temperature / kernel drivers.
- Keep upstream's MIT `LICENSE` and the attribution in README.
- No telemetry; only `api.anthropic.com`, `platform.claude.com`, `chatgpt.com` are contacted.
- Keep source files LF (`git ls-files --eol`); PowerShell's `WriteAllLines` writes CRLF.
