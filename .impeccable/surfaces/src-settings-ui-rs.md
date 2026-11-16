---
version: 1
slug: "src-settings-ui-rs"
primary_target: "src/settings_ui.rs"
related_targets: []
---

# Settings window

Scope: `headroom.exe --settings`, the Slint settings window. Mode: Operate. Audience: Windows developers using Claude Code / Codex who tune the widget once after install and rarely return. Task: decide what each taskbar section shows and how, see the result before closing. Constraints: Slint 1.19 software renderer, native controls only, Ember brand, no long checkbox lists, preview tied to controls, not a generic dark dialog, not too sparse.

Chosen direction: a mix the user picked from the surface round: Guided setup's section rail and big visual style tiles + "The preview is the menu" (the live widget replica is navigation).

## Direction contract

THESIS: The settings window is the widget, enlarged: you edit a section by pointing at it in a live replica of your taskbar, and every choice is shown as the real render it produces. Refuses the category default of a grey sidebar over stacked checkbox pages.

OWN-WORLD: Ember. Charcoal #1C1917 ground, a darker #0F0D0C rail and preview deck, raised #292524 groups, Edge #57534E hairlines, Ceiling #FAFAF9 text, Amber #F59E0B for selection and on-states only. Segoe UI Variable, one family. Monochrome section marks colorized; the Headroom mark in the rail.

STORY: Open, see your taskbar as it will look; click a section (in the replica or the rail); pick its style from three rendered tiles, flip details with segmented switches; close. Hidden sections wait as "+ CPU" chips.

FIRST VIEWPORT: 980x820 preferred, 900x600 minimum; the page scrolls under a fixed preview deck. Left rail 220px: mark + Headroom, five sections each with mark, name and live value or "Off", then General; footer "Unofficial. Not affiliated with Anthropic or OpenAI." Right: preview deck with the widget at up to 1.5x (capped so the strip fits a 600px room) on a taskbar-coloured strip; hidden sections wait at the strip's end as dimmed slots ("+ CPU"); the selected section sits under an amber ceiling bar. Below: section header (mark, Display-size title, one-line description), a full-width "Show on the taskbar" row, chips filling one row, three rendered Style tiles, Details as side-by-side cards. Title bar is charcoal with the mark as window icon.

FORM: mix of dealt structures 1 (Guided setup) and 3 (preview as menu), user-composed; seed: seed:cb80835d47a050a9

SIGNATURE: the amber ceiling bar (from the mark) marks every selection: the section in the preview (slides 180 ms ease-out), the picked Style tile and the picked segment. Amber otherwise only in ticks and on switches.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
