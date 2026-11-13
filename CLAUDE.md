# Headroom

Rust/Win32 taskbar widget (Claude Code + Codex usage, RAM). Fork of claude-usage-widget (MIT).

- Plan, decisions, status, next steps: `docs/DEVPLAN.md` — read it first.
- Build: `cargo build --release` → `target\release\headroom.exe`. Test: `cargo test`.
- Release: bump `Cargo.toml`, push a `v*` tag; Actions builds and publishes. See `docs/RELEASING.md`.
- Windows only (MSVC toolchain). No other runtime deps.
- Don't refresh Codex tokens, don't bring back the upstream self-updater, don't add CPU temp.
