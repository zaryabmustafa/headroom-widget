---
name: Headroom
description: Claude Code and Codex usage limits, plus RAM, CPU and drives, in the Windows 11 taskbar. Settings window design system (Slint, Ember).
colors:
  amber: "#F59E0B"
  amber-ink: "#1C1917"
  ground: "#1C1917"
  deck: "#0F0D0C"
  raised: "#292524"
  raised-hover: "#332E2B"
  hair: "#3A3532"
  edge: "#57534E"
  text: "#FAFAF9"
  muted: "#A8A29E"
  taskbar-dark: "#202020"
  taskbar-light: "#EEEEEE"
typography:
  display:
    fontFamily: "Segoe UI Variable Display, Segoe UI, sans-serif"
    fontSize: "22px"
    fontWeight: 600
  wordmark:
    fontFamily: "Segoe UI Variable Text, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 600
  title:
    fontFamily: "Segoe UI Variable Text, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 600
  body:
    fontFamily: "Segoe UI Variable Text, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 400
  control:
    fontFamily: "Segoe UI Variable Text, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 400
  caption:
    fontFamily: "Segoe UI Variable Text, Segoe UI, sans-serif"
    fontSize: "11px"
    fontWeight: 400
rounded:
  tick: "4px"
  segment: "5px"
  frame: "6px"
  control: "7px"
  card: "8px"
  tile: "10px"
  pill: "11px"
spacing:
  hairline-gap: "2px"
  tight: "6px"
  row: "8px"
  tile-gap: "12px"
  card-pad: "16px"
  block: "18px"
  page-x: "28px"
components:
  rail-item:
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.control}"
    padding: "0 12px 0 10px"
    height: "38px"
  rail-item-selected:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text}"
    rounded: "{rounded.control}"
  field:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.card}"
    padding: "9px 12px 9px 16px"
    height: "54px"
  chip-toggle:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text}"
    typography: "{typography.title}"
    rounded: "{rounded.card}"
    padding: "0 16px 0 12px"
    height: "54px"
  chip-toggle-hover:
    backgroundColor: "{colors.raised-hover}"
  style-tile:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text}"
    typography: "{typography.title}"
    rounded: "{rounded.tile}"
    padding: "14px 12px 12px 12px"
  style-tile-selected:
    backgroundColor: "{colors.raised-hover}"
  segmented:
    backgroundColor: "{colors.deck}"
    textColor: "{colors.muted}"
    typography: "{typography.control}"
    rounded: "{rounded.control}"
    padding: "3px"
    height: "36px"
  segmented-option-picked:
    backgroundColor: "{colors.raised-hover}"
    textColor: "{colors.text}"
    rounded: "{rounded.segment}"
  toggle-off:
    backgroundColor: "{colors.hair}"
    rounded: "{rounded.pill}"
    width: "42px"
    height: "22px"
  toggle-on:
    backgroundColor: "{colors.amber}"
    rounded: "{rounded.pill}"
    width: "42px"
    height: "22px"
---

# Design System: Headroom

## Overview

**Creative North Star: "The Widget, Enlarged"**

The settings window is the taskbar widget blown up and laid open. A live replica of the user's own taskbar sits in a dark deck at the top; it is both the preview and the navigation. Every choice below it is shown as the render it produces, not described in a checkbox list. The world is Ember, the same charcoal-and-amber palette as the Headroom mark: warm near-black stone surfaces in three depths, one off-white for text, and amber reserved for "this one" and "on".

Density is moderate desktop-tool density: 13px body, 54px rows, 8px card corners, one font family. Depth comes from tonal steps of the stone ramp and 1px hairlines, never shadows. The mark's ceiling bar (a thin amber rule) is the system's single signature: it sits over whatever is selected, in the preview, on a style tile, on a segment.

The window refuses the category default of a grey sidebar over stacked checkbox pages, and it refuses the generic Windows dialog look: even the title bar is painted charcoal.

**Key Characteristics:**
- One live preview that doubles as navigation.
- Three stone depths (deck, ground, raised) separated by hairlines; flat, no shadows.
- Amber only for selection and on-states, shown as a ceiling bar, a filled tick or a filled switch.
- Choices are rendered, not described: style tiles carry the real section art.
- One family, Segoe UI Variable, Display cut for page titles only.

## Colors

Warm stone neutrals in three depths with a single amber accent lifted from the mark.

### Primary
- **Ember Amber** (`amber`): selection and on-state only. The ceiling bar over the edited section, the picked style tile and the picked segment; the filled tick of a chip; the "on" track of a switch; the keyboard focus ring; a selected rail item's mark and the page-title mark of a shown section.
- **Amber Ink** (`amber-ink`): anything drawn on top of amber (switch knob when on, the tick's check stroke). Same value as Charcoal, named separately so the pairing stays explicit.

### Neutral
- **Charcoal Ground** (`ground`): window background, the scrolling page area, and the title bar (painted via DWM caption colour).
- **Deck Black** (`deck`): the left rail, the preview deck and the inset well of segmented controls. Darker than the ground, so the rail and preview read as the frame and the page as the work area.
- **Raised Stone** (`raised`): cards (fields, chips, detail cards, style tiles) and the selected rail item.
- **Raised Stone, Lit** (`raised-hover`): hover on cards, the selected style tile, the picked segment.
- **Hairline** (`hair`): 1px borders on every card, dividers between rail, deck and page, the switch track when off.
- **Edge** (`edge`): the stronger 1px border on a card that is on or selected; switch track on hover; the Logo's outline.
- **Ceiling White** (`text`): all primary text, the switch knob when off, the mark's ceiling.
- **Ash** (`muted`): notes, values in the rail, captions, unselected segment labels, hidden-section marks, the unticked checkbox outline.
- **Taskbar Dark / Taskbar Light** (`taskbar-dark`, `taskbar-light`): not brand colours. The preview strip and the style-tile art frames use whichever matches the user's Windows theme, so the replica looks like their real taskbar.

### Named Rules
**The Ceiling Rule.** Amber marks "this one" or "on" and nothing else. A selection is shown by a short amber ceiling bar centred near the element's top (2-3px tall, inset 4-5px from the top edge, never touching corners or borders); an on-state by an amber fill. No amber text, no amber headings, no amber decoration.

**The Real Taskbar Rule.** The preview and every rendered choice sit on the user's actual taskbar colour, never on an Ember surface, because the point is to show what they will really see.

## Typography

**Display Font:** Segoe UI Variable Display (page titles only)
**Body Font:** Segoe UI Variable Text (window default)

**Character:** The native Windows 11 voice, used in one family. Hierarchy comes from size and weight steps (600 vs 400), never from a second face, colour or uppercase.

### Hierarchy
- **Display** (600, 22px): the section page title beside its 28px mark.
- **Wordmark** (600, 15px): "Headroom" beside the Logo in the rail. Used once.
- **Title** (600, 13px): group headings (Style, Details, Size), chip labels, style-tile labels; a selected rail item's name.
- **Body** (400, 13px): field and detail labels, rail names.
- **Control** (400, 12px; 600 when picked): segment labels, rail live values, page descriptions.
- **Caption** (400, 11px): notes under labels, chip details, preview hint, ghost-slot labels, the unofficial footer.

### Named Rules
**The One Family Rule.** Segoe UI Variable throughout; the Display optical cut appears only at 22px page titles.

## Layout

A fixed two-column window: 220px rail on the left, a 1px hairline, then the work column. Preferred size 980x820, minimum 900x600.

- **Rail** (deck): 14px padding (18px top), Logo and wordmark, five section rows and General separated by a hairline, a flexible spacer, then the unofficial footer pinned to the bottom.
- **Preview deck** (deck, fixed): the widget replica centred on a taskbar-coloured strip, height = preview + 96px, with a caption below. The replica draws at up to 1.5x and shrinks (floor 0.6x) so the strip, including 74px for each hidden-section slot, fits a 600px room.
- **Page** (ground, scrolls under the deck): 28px side padding, 20px top, 22px bottom; 18px between blocks. Order on a section page: header (mark, title, one-line description), full-width "Show on the taskbar" field, chips in one row, three style tiles in one row, detail cards side by side.
- **Rhythm:** 18px between page blocks, 6px between a group heading and its rows (heading carries 4px extra below), 8px between chips, cards and details, 12px between style tiles, 2px between rail rows.
- Rows always fill the width: chips, tiles and detail cards stretch equally rather than wrap.

## Elevation & Depth

Flat. No shadows anywhere. Depth is tonal: deck (darkest, the frame) under ground (the page) under raised (cards), with a lit raised step for hover and selection. Every card carries a 1px hairline border; that border steps up to Edge when the card is on or selected.

### Named Rules
**The Stone Steps Rule.** Elevation is a lighter stone step plus a hairline, never a shadow or glow.

**The Dim, Don't Hide Rule.** When a section is off, its settings stay visible and usable at 0.7 opacity; in the preview it waits at the strip's end as a muted "+ Name" slot. Nothing disappears because it is off.

## Shapes

Gently rounded rectangles on a tight radius ladder that grows with the element: 4px ticks, 5px segment options, 6px art frames and preview hit areas, 7px rail rows and segmented wells, 8px cards, 10px style tiles and the preview strip, fully rounded 11px switch pill. Nested corners shrink inward (well 7px, option 5px inside 3px padding). Selection bars are short, centred rounded rules (2-3px tall, radius half the height): 40% of a segment option's width (20-56px) at 4px from its top, 30% of a style tile's width (32-72px) at 5px from its top, echoing the mark's ceiling. The preview's bar is the exception: it spans the section, less 8px each side. The keyboard focus ring is a 2px amber outline drawn 3px outside the element, its radius = the element's radius + 3px.

## Components

### Buttons and pressables
There are no filled buttons. Every clickable thing (rail row, chip, tile, segment, preview section, ghost slot, switch) is a Pressable: pointer cursor, Space and Enter activate it, focus shows the 2px amber ring. Hover is a lighter stone step, or a 8% white wash (`#FFFFFF14`) on the taskbar-coloured preview.

### Switch (Toggle)
- **Shape:** 42x22 pill, 16px knob, slides 150 ms ease-out.
- **Off:** hairline track, ceiling-white knob; hover lifts the track to Edge.
- **On:** amber track, amber-ink knob.
- Used for binary settings in a Field row (show section, roll numbers, start with Windows).

### Segmented control
- **Style:** deck-coloured inset well, 36px tall, hairline border, 3px padding, options stretch equally (min 52px).
- **Picked:** lit stone background, ceiling-white 600 label, short centred amber ceiling bar (2px, 4px from the top) fading in over 150 ms; an optional 14px mark beside the word turns amber.
- **Unpicked:** transparent, Ash label; hover is raised stone.
- The control for every one-of-several choice; never radio buttons or dropdowns.

### Chip toggle
- **Style:** a checkbox that reads as a card: raised stone, 8px corners, 54px tall, min 132px, hairline border (Edge when on).
- **Tick:** 18px square, 4px corners; off is a 1.5px Ash outline, on is amber fill with an amber-ink check.
- **Label:** 13px 600, ceiling-white when on, Ash when off; optional 11px detail below.
- Used for what a section shows (limits, drives); fills one row.

### Field (setting row)
- Full-width raised card, 8px corners, min 54px, hairline border. Label (13px) and optional note (11px Ash) on the left; the control right-aligned. General settings and "Show on the taskbar" use it.

### Detail card
- Raised card, 8px corners, 12px padding: label and note above, a Segmented control below. Detail cards sit side by side and share the row.

### Navigation (rail)
- **Row:** 38px, 7px corners, 18px mark, 13px name, 12px Ash live value on the right ("Off" when hidden).
- **Selected:** raised stone, name 600, mark amber.
- **Hover:** 4% white wash (`#FFFFFF0A`).
- **Hidden section:** name and mark in Ash.

### Style tile (signature)
- Raised card, 10px corners, hairline border; a 76px art frame (6px corners, taskbar colour) holding the section really rendered in that style, then a 13px 600 label and an 11px note.
- **Selected:** lit stone, Edge border, a short centred 3px amber ceiling bar 5px from the top, fading in over 150 ms.
- Always three in a row: Numbers, Rings, Numbers in rings.

### Preview deck (signature)
- The live widget on a 10px-cornered strip in the user's taskbar colour. Each section is a hit area (6px corners, 8% white hover). The edited section wears a 3px amber ceiling bar that slides and resizes over 180 ms ease-out. Hidden sections wait at the end as 70px slots: a faint 15% white divider, the muted mark and "+ Name" in 11px; clicking one turns it on.

## Do's and Don'ts

### Do:
- **Do** show a choice as the render it produces (a style tile, the live preview), not as a sentence.
- **Do** mark selection with a short, centred amber ceiling bar just inside the element's top, and on-states with an amber fill.
- **Do** build depth from the stone steps (deck, ground, raised, raised-hover) plus 1px hairlines.
- **Do** use the Segmented control for every one-of-several choice and the Switch for every on/off row.
- **Do** dim off sections' settings to 0.7 opacity and keep them operable.
- **Do** keep every pressable keyboard-reachable with the 2px amber focus ring 3px outside the element.
- **Do** keep the "Unofficial. Not affiliated with Anthropic or OpenAI." line in the rail footer.

### Don't:
- **Don't** use amber for text, headings or decoration; it means selected or on.
- **Don't** use shadows or glows for elevation.
- **Don't** stack long checkbox lists; options that a section shows are chip toggles in one row.
- **Don't** add a second typeface or uppercase labels; hierarchy is size and weight within Segoe UI Variable.
- **Don't** put the Claude or OpenAI marks in Headroom's own branding (rail Logo, title bar icon); they identify sections only.
- **Don't** let the window fall back to system grey chrome; the title bar is charcoal.
