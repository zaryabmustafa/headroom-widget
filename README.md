<div align="center">

<img src="assets/icon.png" width="96" alt="Headroom logo">

# Headroom

**Know your limits before you hit them.**

A Windows 11 taskbar widget that shows your Claude Code and OpenAI Codex usage limits, plus RAM.

![Windows 11](https://img.shields.io/badge/Windows-11-F59E0B?style=flat-square&labelColor=1C1917)
![Rust](https://img.shields.io/badge/Rust-Win32-F59E0B?style=flat-square&labelColor=1C1917)
![No admin](https://img.shields.io/badge/admin-not%20needed-F59E0B?style=flat-square&labelColor=1C1917)
![License MIT](https://img.shields.io/badge/license-MIT-F59E0B?style=flat-square&labelColor=1C1917)
[![Downloads](https://img.shields.io/github/downloads/zaryabmustafa/headroom-widget/total?style=flat-square&labelColor=1C1917&color=F59E0B&label=downloads)](https://github.com/zaryabmustafa/headroom-widget/releases)

<img src="assets/screenshot.png" alt="Headroom in the taskbar: Claude 5h 29%, weekly 18%; Codex weekly 100%; RAM 74%">

</div>

---

Headroom is a free, open-source usage tracker for Claude Code and Codex on Windows.
It sits in the taskbar next to the clock and shows how much of your Claude 5-hour
limit, Claude weekly limit and Codex (ChatGPT) rate limits you've used, so you can
see when you're close to a limit before you hit it. It needs no API key; it reads
the logins Claude Code and the Codex CLI already saved. It's one small native exe
(Rust, Win32) using about 18 MB of RAM and almost no CPU.

## What it shows

| Section | What you see | Source |
|---|---|---|
| **Claude Code** | `5h` and `W` (weekly) used %, plus `F` (Fable) if you turn it on | `%USERPROFILE%\.claude\.credentials.json` → `api.anthropic.com/api/oauth/usage`. Refreshes the login the same way Claude Code does |
| **Codex** | `5h` and `W` used % (plans with only a weekly limit show just `W`) | `%USERPROFILE%\.codex\auth.json` (or `CODEX_HOME`) → `chatgpt.com/backend-api/wham/usage`. Read-only: when the token has expired, it falls back to the last `rate_limits` event in `.codex\sessions` and shows it dimmed |
| **RAM** | Memory in use, % | `GlobalMemoryStatusEx`, every 2 s |

Hover for details: who's signed in, reset times, and used / total GB of RAM.
Values turn orange at 80% and red at 95%. Each section fails on its own, so if one is
down it shows `–` and the others carry on.

> Neither usage endpoint is a public API; either can change without notice.

## Install

Download `headroom.exe` from the [latest release](https://github.com/zaryabmustafa/headroom-widget/releases/latest),
put it in a folder of your own (for example `%LOCALAPPDATA%\Programs\Headroom`), and run it.
It's a single exe with no installer, no service and no admin. Right-click the
widget and choose **Start with Windows** to launch it at sign-in.

The exe isn't code-signed yet, so Windows SmartScreen may warn on first run:
choose **More info → Run anyway**.

### Build from source

You need Rust (MSVC toolchain) and Visual Studio Build Tools with
**Desktop development with C++**.

```powershell
cargo build --release
.\target\release\headroom.exe
```

Maintainers: see [docs/RELEASING.md](docs/RELEASING.md) for how releases are cut.

## Menu

Right-click the widget:

| Item | What's inside |
|---|---|
| **Show** | 5-hour, Weekly and Fable limits · Codex · RAM |
| **Appearance** | Font size, Icon · remaining instead of used · labels · rolling numbers · usage rings · left side of the taskbar |
| **Accounts** | Add a Claude Code folder (`.claude`, `CLAUDE_CONFIG_DIR`) or a Codex folder (`.codex`, `CODEX_HOME`) · remove · accounts per column |
| **Update every** | Refresh interval |
| | Start with Windows · Refresh now · Open Claude / Codex usage page · Exit |

Settings live in `HKCU\Software\Headroom`.

## Privacy

Headroom only talks to `api.anthropic.com`, `platform.claude.com` and
`chatgpt.com`, using the logins that Claude Code and Codex already saved on
your PC. It has no telemetry and no self-update. It never writes to Codex's
`auth.json`.

## FAQ

**How do I check my Claude Code usage limit on Windows?**
Run Headroom. The Claude section shows the percentage of your 5-hour and weekly
limits used, and hovering shows when each one resets. Click it to open Claude's
usage page.

**Does it work with Claude Pro and Max plans?**
Yes. It works with any plan you sign in to Claude Code with; it shows the same
numbers as Claude Code's `/usage` command.

**How do I see my Codex rate limits?**
Sign in to the Codex CLI once. Headroom reads its saved login and shows the
5-hour and weekly Codex limits next to Claude's.

**Do I need an API key?**
No. Headroom uses the logins that Claude Code and Codex already saved on your PC.

**Will it get my account rate-limited?**
It checks at most once a minute (every 2 minutes by default), backs off when a
server says it's busy, and never retries a login the server refused.

**Does it work on Windows 10?**
It's built and tested on Windows 11. Windows 10 isn't tested.

## Credits

Fork of [claude-usage-widget-for-windows-taskbar](https://github.com/AbdoSarmini/claude-usage-widget-for-windows-taskbar)
by Abdo Sarmini, MIT-licensed; see [LICENSE](LICENSE). The RAM icon is
Lucide's `memory-stick` (ISC).

Unofficial: not affiliated with Anthropic or OpenAI. Claude and the OpenAI logo
are their owners' trademarks and are used only to label the sections.
Brand details are in [docs/BRAND.md](docs/BRAND.md).
