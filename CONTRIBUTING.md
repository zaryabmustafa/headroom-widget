# Contributing to Headroom

Thanks for helping. Bug reports, fixes and small improvements are all welcome.

## Before you start

- **Bugs:** [open a bug report](https://github.com/zaryabmustafa/headroom-widget/issues/new?template=bug_report.yml).
  Include what the tooltip says and your Windows version.
- **Ideas:** [open a feature request](https://github.com/zaryabmustafa/headroom-widget/issues/new?template=feature_request.yml)
  before writing code for anything bigger than a small fix, so we can agree on the approach first.
- **Security problems:** don't open an issue; see [SECURITY.md](SECURITY.md).

Some things are deliberately left out. Please don't send pull requests that:

- refresh or write Codex tokens (Headroom only reads Codex's `auth.json`),
- add a self-updater or telemetry,
- add CPU temperature (it needs a kernel driver or admin rights).

## Build and test

You need Windows, Rust (MSVC toolchain) and Visual Studio Build Tools with
**Desktop development with C++**.

```powershell
cargo build --release
cargo test
.\target\release\headroom.exe
```

Exit any running copy of Headroom first, or the build can't replace the exe.

## Pull requests

- Keep each pull request to one change, and say what it fixes and how you tested it.
- Run `cargo test` and `cargo clippy` before you push.
- Keep the code small: Headroom has no runtime dependencies besides `windows-sys`,
  and new ones need a good reason.
- Use LF line endings.

Plans and past decisions are in [docs/DEVPLAN.md](docs/DEVPLAN.md). Maintainers cut
releases as described in [docs/RELEASING.md](docs/RELEASING.md).

By contributing, you agree that your work is released under the [MIT License](LICENSE).
