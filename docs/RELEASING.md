# Releasing Headroom

Releases are built by GitHub Actions ([`.github/workflows/release.yml`](../.github/workflows/release.yml)),
not on anyone's PC. Pushing a version tag is the whole process.

## Steps

1. **Bump the version** in `Cargo.toml` (`version = "0.1.2"`), then run `cargo build` once so
   `Cargo.lock` picks it up. Commit both. The workflow doesn't check that the tag matches
   `Cargo.toml`, so keep them in step yourself.
2. **Push `main`** and make sure it's what you want to ship.
3. **Tag and push the tag:**

   ```powershell
   git tag -a v0.1.2 -m "Headroom v0.1.2"
   git push origin v0.1.2
   ```

4. **Watch it run:** Actions tab → *Release*, or `gh run watch`. It takes about 1–2 minutes.

That's it. The release appears under *Releases*, marked Latest, with `headroom-setup.exe`
(installer) and `headroom.exe` (portable) attached.

## What the workflow does

On a `v*` tag, on `windows-latest`:

| Step | Notes |
|---|---|
| `cargo test --locked` | A failing test stops the release |
| `cargo build --release --locked` | Fails if `Cargo.lock` is out of date |
| Installer | Builds `headroom-setup.exe` from [`installer/headroom.iss`](../installer/headroom.iss) with Inno Setup (preinstalled on the runner; installed with Chocolatey if not). Its version comes from `Cargo.toml` |
| Checksum | SHA-256 of both files, shown in the run summary |
| Upload artifact | Both files are also kept on the run page, so you can download them from there |
| Publish | **No release for the tag yet:** creates one titled `Headroom vX.Y.Z` with GitHub's generated notes plus the SHA-256 lines. **Release already exists:** keeps its notes and (re)attaches both files |

### Building the installer locally

Install Inno Setup 6 (`winget install JRSoftware.InnoSetup`), build the release exe, then:

```powershell
& "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe" /DAppVersion=0.1.4 installer\headroom.iss
```

The installer lands in `target\installer\headroom-setup.exe`. Silent install for testing:
`headroom-setup.exe /VERYSILENT /TASKS=startup`; uninstall with `unins000.exe /VERYSILENT`
from the install folder. Uninstalling deletes `HKCU\Software\Headroom`, so export it first if
you want to keep your settings.

### Hand-written release notes

Let the workflow publish, then replace the generated notes:

```powershell
gh release edit v0.1.2 --notes-file notes.md
```

Copy the SHA-256 line from the generated notes (or the run summary) into `notes.md` first
if you want to keep it. Re-running the workflow later for the same tag keeps your notes; it
only re-attaches the exe.

## Dry run

Actions → *Release* → **Run workflow** (or `gh workflow run release.yml --ref main`).
It tests, builds and keeps the exe as a run artifact, and publishes nothing. Use it before
tagging when you've changed the build or the workflow.

## Fixing a bad release

- **Wrong exe, same version:** fix `main`, then move the tag and re-run:

  ```powershell
  git tag -f v0.1.2 && git push -f origin v0.1.2
  ```

  Moving a `v*` tag is blocked by the tag ruleset for everyone but admins.
  The workflow replaces both files (`--clobber`). Downloaders can't tell from the version that
  the exe changed, and the SHA-256 in the notes is now wrong. Prefer a new patch version.
- **Pull a release:** `gh release delete v0.1.2 --cleanup-tag`. The download badge drops its
  counts with it.

## Good to know

- The exe isn't code-signed, so SmartScreen warns on first run until it builds reputation.
  The README tells users how to get past the warning.
- v0.1.0 and v0.1.1 were built locally; releases from v0.1.2 on come from Actions.
- The workflow uses `actions/checkout@v5` and `actions/upload-artifact@v6` (Node 24). If
  GitHub starts warning about a Node version again, bump those.
