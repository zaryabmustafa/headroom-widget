# Headroom — brand

**Name:** Headroom. **Tagline:** *Know your limits before you hit them.*

**One-liner:** Claude Code and Codex usage limits, plus RAM, in the Windows 11 taskbar.

## Mark

![Headroom mark](../assets/icon.png)

Three meters (one per section: Claude, Codex, RAM) under a ceiling bar. The
gap between them is the headroom. Drawn on a 16-unit grid so it stays crisp
at 16 px.

| File | Use |
|---|---|
| `assets/logo.svg` | Source of truth |
| `assets/icon.ico` | Exe icon: 16/20/24/32/40/48/64/256, PNG entries |
| `assets/icon.png` | 256 px, README / store listings |

Regenerate the `.ico`/`.png` from the SVG's numbers if the mark changes
(WPF `DrawingVisual` → `RenderTargetBitmap`, PNG frames packed into an ICO).

## Colors

Palette "Ember" (picked 2026-11-13 over graphite+mint, traffic, aurora, ink, lime).

| Token | Hex | Use |
|---|---|---|
| Charcoal | `#1C1917` | Tile / background |
| Edge | `#57534E` | 0.5-unit inner outline, so the tile reads on dark backgrounds |
| Ceiling | `#FAFAF9` | Ceiling bar, text on dark |
| Amber | `#F59E0B` | Meters, accent |

In the widget itself, values use the taskbar's own palette. The brand colors
are for the icon, README and any store page.

## Rules

- Provider names appear only in descriptions ("for Claude Code and Codex"),
  never in the product name or the mark.
- The Claude and OpenAI marks appear in the widget to identify sections. They
  aren't part of Headroom's brand, so never use them on the icon, the README
  header or promo art.
- Unofficial: keep the "not affiliated with Anthropic or OpenAI" line wherever
  the product is described.
