# Headroom

Claude Code and Codex usage limits, plus RAM, in the Windows 11 taskbar.

```
✳ 5h 24% | W 82% |  >_ 5h 12% | W 40% |  RAM 61% |
```

Fork of [claude-usage-widget-for-windows-taskbar](https://github.com/AbdoSarmini/claude-usage-widget-for-windows-taskbar)
by Abdo Sarmini (MIT). Unofficial; not affiliated with Anthropic or OpenAI.

## What it shows

| Section | Source | Notes |
|---|---|---|
| Claude Code | `%USERPROFILE%\.claude\.credentials.json` → `api.anthropic.com/api/oauth/usage` | Refreshes the login the same way Claude Code does (upstream behaviour) |
| Codex | `%USERPROFILE%\.codex\auth.json` (or `CODEX_HOME`) → `chatgpt.com/backend-api/wham/usage` | Read-only. When the token has expired, falls back to the last `rate_limits` event in `.codex\sessions\**\*.jsonl` and shows it dimmed |
| RAM | `GlobalMemoryStatusEx`, every 2 s | Tooltip shows used / total GB |

Windows are labelled by their length, not position: if OpenAI turns the
5-hour window off, the Codex `5h` group hides itself.

Neither usage endpoint is a public API; either can change without notice.

## Build

Needs Rust (MSVC toolchain) and Visual Studio Build Tools with "Desktop development with C++".

```powershell
cargo build --release
.\target\release\headroom.exe
```

## Menu (right-click)

- Customize → Show Codex / Show RAM, plus all upstream options (rings, icon, labels, font size…)
- Accounts → Add account… accepts a Claude Code folder (`.claude`, `CLAUDE_CONFIG_DIR`) or a Codex folder (`.codex`, `CODEX_HOME`)
- Open Claude usage page / Open Codex usage page

Settings live in `HKCU\Software\Headroom`. Self-update is disabled (upstream
releases would replace Headroom with the original widget).
