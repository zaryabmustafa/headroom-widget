# Product

<!-- impeccable:product-schema 1 -->

## Platform

windows-desktop (native Win32 widget; the settings window is Slint 1.19, software renderer, winit backend). Not web, iOS or Android.

## Users

Developers on Windows 11 who use Claude Code and/or the Codex CLI and want their usage limits, plus RAM, CPU and disk, visible at a glance on the taskbar. They open the settings window after installing to tune what the widget shows, then rarely come back (confirmed 2026-11-16).

## Product Purpose

Headroom shows how close you are to your Claude Code and Codex usage limits (5-hour, weekly, Fable) and how loaded your PC is (RAM, CPU, internal drives), embedded in the Windows taskbar. Success: you notice you're near a limit before you hit it, without opening a browser. Tagline: *Know your limits before you hit them.*

## Positioning

A single native exe that lives inside the taskbar itself (not a tray icon, not a floating window), reads the logins Claude Code and Codex already saved, needs no API key, no admin rights and no service.

## Operating Context

- The widget sits in the taskbar beside the tray; hover shows a tooltip with reset times and details; click opens the provider's usage page, Task Manager or This PC.
- Right-click menu: Show (sections on/off), Accounts (add/remove Claude Code or Codex folders), Settings…, Refresh now, usage pages, Exit.
- Settings window (`headroom.exe --settings`), a separate process. Settings live in `HKCU\Software\Headroom`; each change is written there and the widget redraws at once.
- Sections, left to right: Claude, Codex, RAM, CPU, Drives. Each has its own look: remaining or used, labels, icon, usage rings, numbers inside the rings. Claude: 5-hour / Weekly / Fable limits and an icon choice (asterisk, pixel creature, none). Codex: 5-hour / Weekly. Drives: which internal drives (up to three fit on the taskbar). General: font size, icon size, update interval (1–30 min), accounts per column, rolling numbers, left side of the taskbar, Start with Windows.
- Taskbar light or dark follows Windows; vertical taskbars get a stacked layout.

## Capabilities and Constraints

- One exe, no runtime dependencies, MIT licence. Slint under its royalty-free licence (badge in README).
- Values turn orange above 75% and red above 95%, matching Claude Code.
- Usage endpoints are not public APIs; numbers can be missing (`–`) or stale (dimmed).
- Don't refresh Codex tokens, don't add a self-updater, don't add CPU temperature.
- The settings preview may only show sample numbers for Claude and Codex (it never fetches); RAM, CPU and drives are real.

## Brand Commitments

- Name: Headroom. Provider names only in descriptions ("for Claude Code and Codex"), never in the product name or mark.
- Mark: three meters under a ceiling bar (`assets/logo.svg`, `assets/icon.png`); the gap is the headroom.
- Palette "Ember": Charcoal `#1C1917`, Edge `#57534E`, Ceiling `#FAFAF9`, Amber `#F59E0B`.
- The settings window is a branded Headroom surface, not a generic Windows dialog (confirmed 2026-11-16).
- Claude and OpenAI marks identify sections in the widget only; never in Headroom's own branding.
- Unofficial: keep "not affiliated with Anthropic or OpenAI" wherever the product is described.

## Evidence on Hand

- `assets/logo.svg`, `assets/icon.png`, `assets/icon.ico`; section marks `assets/*_mark.a8` (Lucide memory-stick, cpu, hard-drive; OpenAI from Simple Icons).
- `docs/BRAND.md`, `README.md`, `docs/DEVPLAN.md`.
- No testimonials, user counts or press; don't invent them.

## Product Principles

1. Glanceable over complete: the taskbar shows the few numbers that matter; detail lives one hover away.
2. Native and light: one small exe, no services, nothing running that isn't needed.
3. Honest numbers: show stale or missing data as such; never guess.
4. Set and forget: settings are clear on first visit and stay out of the way after.

## Accessibility & Inclusion

Keep Slint's accessibility support on (screen readers). Controls must be keyboard reachable; warning colours are never the only signal (the number itself carries the value).
