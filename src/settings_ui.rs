//! The settings window: `headroom.exe --settings`, a separate process so the widget's
//! own message loop stays as it is and nothing extra stays in memory once it's closed.
//! Settings live in the registry; after each change this window tells the widget
//! (WM_APP_RELOAD) and it re-reads them.
//!
//! Layout: a section rail on the left, and over the page a live replica of the widget
//! that is also the navigation (click a section in it). Style choices are shown as
//! real renders of that section. Everything is drawn with render.rs, so the window
//! shows exactly what the taskbar will.

use crate::*;
use slint::{ComponentHandle, Image, Model, ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};

slint::slint! {
    #[edition = "1.19"]
    import { ScrollView } from "std-widgets.slint";

    // Settings by registry key; a few keys are special (see ui_get / ui_set).
    export global Api {
        pure callback get(string) -> int;
        callback set(string, int);
    }

    // Ember, Headroom's palette (docs/BRAND.md), stretched to a working surface.
    global Theme {
        out property <color> ground: #1C1917;
        out property <color> deck: #0F0D0C;
        out property <color> raised: #292524;
        out property <color> raised-hover: #332E2B;
        out property <color> hair: #3A3532;
        out property <color> edge: rgb(87, 83, 78);
        out property <color> text: #FAFAF9;
        out property <color> muted: #A8A29E;
        out property <color> amber: #F59E0B;
        out property <color> amber-ink: #1C1917;
    }

    export struct SectionItem {
        name: string,
        value: string,
        shown: bool,
        icon: image,
    }

    // A clickable section in the preview, in logical pixels from the image's left edge.
    export struct Spot {
        x: length,
        w: length,
        index: int,
        name: string,
    }

    export struct Ghost {
        name: string,
        index: int,
        icon: image,
    }

    export struct Chip {
        label: string,
        detail: string,
        key: string,
        invert: bool,
    }

    export struct Art {
        img: image,
        w: length,
        h: length,
    }

    // What one section's page offers.
    export struct PageData {
        index: int,
        prefix: string,
        title: string,
        detail: string,
        hide-key: string,
        icon: image,
        chips-title: string,
        chips: [Chip],
        number-options: [string],
        number-note: string,
        icon-key: string,
        icon-options: [string],
        icon-images: [image],
        icon-note: string,
        sizes: bool,
        labels: bool,
    }

    // Anything you can click or reach with Tab; Space and Enter press it.
    component Pressable inherits Rectangle {
        callback clicked;
        in property <bool> enabled: true;
        out property <bool> hovered: ta.has-hover;
        accessible-role: button;
        accessible-action-default => { root.clicked(); }
        fs := FocusScope {
            enabled: root.enabled;
            key-pressed(e) => {
                if (e.text == " " || e.text == "\n") {
                    root.clicked();
                    return accept;
                }
                reject
            }
        }
        ta := TouchArea {
            enabled: root.enabled;
            mouse-cursor: root.enabled ? pointer : default;
            clicked => {
                root.clicked();
            }
        }
        Rectangle {
            x: -3px;
            y: -3px;
            width: parent.width + 6px;
            height: parent.height + 6px;
            border-radius: parent.border-radius + 3px;
            border-width: fs.has-focus ? 2px : 0px;
            border-color: Theme.amber;
        }
        @children
    }

    // Headroom's mark, drawn from logo.svg's 16-unit grid at 1.5x.
    component Logo inherits Rectangle {
        width: 24px;
        height: 24px;
        border-radius: 5px;
        background: #1C1917;
        border-width: 1px;
        border-color: Theme.edge;
        Rectangle { x: 4.5px; y: 4.5px; width: 15px; height: 1.5px; border-radius: 0.75px; background: Theme.text; }
        Rectangle { x: 4.5px; y: 13.5px; width: 3px; height: 6px; border-radius: 0.6px; background: Theme.amber; }
        Rectangle { x: 10.5px; y: 9px; width: 3px; height: 10.5px; border-radius: 0.6px; background: Theme.amber; }
        Rectangle { x: 16.5px; y: 12px; width: 3px; height: 7.5px; border-radius: 0.6px; background: Theme.amber; }
    }

    component Toggle inherits Pressable {
        in property <string> key;
        in property <bool> invert;
        in property <string> label;
        in-out property <bool> on: root.invert ? Api.get(root.key) == 0 : Api.get(root.key) == 1;
        callback toggled;
        accessible-role: switch;
        accessible-checkable: true;
        accessible-checked: root.on;
        accessible-label: root.label;
        width: 42px;
        height: 22px;
        border-radius: 11px;
        background: root.on ? Theme.amber : (root.hovered ? Theme.edge : Theme.hair);
        clicked => {
            root.on = !root.on;
            Api.set(root.key, (root.on != root.invert) ? 1 : 0);
            root.toggled();
        }
        knob := Rectangle {
            width: 16px;
            height: 16px;
            y: 3px;
            x: root.on ? parent.width - self.width - 3px : 3px;
            border-radius: 8px;
            background: root.on ? Theme.amber-ink : Theme.text;
            animate x { duration: 150ms; easing: ease-out; }
        }
    }

    // A row of mutually exclusive choices; the setting holds the picked index.
    component Segmented inherits Rectangle {
        in property <string> key;
        in property <[string]> options;
        // Drawn marks beside the words (the Claude icon choice); empty images draw nothing.
        in property <[image]> icons;
        in property <string> label;
        in-out property <int> current: Api.get(root.key);
        height: 36px;
        border-radius: 7px;
        background: Theme.deck;
        border-width: 1px;
        border-color: Theme.hair;
        HorizontalLayout {
            padding: 3px;
            spacing: 2px;
            for o[i] in root.options: Pressable {
                horizontal-stretch: 1;
                min-width: 52px;
                border-radius: 5px;
                accessible-role: radio-button;
                accessible-label: root.label + ": " + o;
                accessible-checkable: true;
                accessible-checked: root.current == i;
                background: root.current == i ? Theme.raised-hover : (self.hovered ? Theme.raised : transparent);
                clicked => {
                    root.current = i;
                    Api.set(root.key, i);
                }
                HorizontalLayout {
                    alignment: center;
                    spacing: 6px;
                    if i < root.icons.length && root.icons[i].width > 0: VerticalLayout {
                        alignment: center;
                        Image {
                            source: root.icons[i];
                            width: 14px;
                            height: 14px;
                            colorize: root.current == i ? Theme.amber : Theme.muted;
                        }
                    }
                    Text {
                        text: o;
                        vertical-alignment: center;
                        font-size: 12px;
                        font-weight: root.current == i ? 600 : 400;
                        color: root.current == i ? Theme.text : Theme.muted;
                    }
                }
                // The ceiling bar marks the choice, as it marks the section in the preview.
                Rectangle {
                    width: clamp(parent.width * 0.4, 20px, 56px);
                    x: (parent.width - self.width) / 2;
                    y: 4px;
                    height: 2px;
                    border-radius: 1px;
                    background: Theme.amber;
                    opacity: root.current == i ? 1 : 0;
                    animate opacity { duration: 150ms; easing: ease-out; }
                }
            }
        }
    }

    // A tick that reads as a card: what a section shows (a limit, a drive).
    component ChipToggle inherits Pressable {
        in property <Chip> data;
        in-out property <bool> on: root.data.invert ? Api.get(root.data.key) == 0 : Api.get(root.data.key) == 1;
        accessible-role: checkbox;
        accessible-checkable: true;
        accessible-checked: root.on;
        accessible-label: root.data.label;
        min-width: 132px;
        horizontal-stretch: 1;
        height: 54px;
        border-radius: 8px;
        background: root.hovered ? Theme.raised-hover : Theme.raised;
        border-width: 1px;
        border-color: root.on ? Theme.edge : Theme.hair;
        clicked => {
            root.on = !root.on;
            Api.set(root.data.key, (root.on != root.data.invert) ? 1 : 0);
        }
        HorizontalLayout {
            padding-left: 12px;
            padding-right: 16px;
            spacing: 10px;
            VerticalLayout {
                alignment: center;
                Rectangle {
                    width: 18px;
                    height: 18px;
                    border-radius: 4px;
                    background: root.on ? Theme.amber : transparent;
                    border-width: root.on ? 0px : 1.5px;
                    border-color: Theme.muted;
                    if root.on: Path {
                        x: 3px;
                        y: 3px;
                        width: 12px;
                        height: 12px;
                        viewbox-width: 12;
                        viewbox-height: 12;
                        commands: "M 2 6.5 L 5 9.5 L 10.5 3";
                        stroke: Theme.amber-ink;
                        stroke-width: 2px;
                    }
                }
            }
            VerticalLayout {
                alignment: center;
                spacing: 1px;
                Text {
                    text: root.data.label;
                    font-size: 13px;
                    font-weight: 600;
                    color: root.on ? Theme.text : Theme.muted;
                }
                if root.data.detail != "": Text {
                    text: root.data.detail;
                    font-size: 11px;
                    color: Theme.muted;
                }
            }
        }
    }

    // One style, shown as the section rendered that way.
    component StyleTile inherits Pressable {
        in property <string> label;
        in property <string> note;
        in property <Art> art;
        in property <bool> selected;
        in property <color> bar;
        accessible-checkable: true;
        accessible-checked: root.selected;
        accessible-label: root.label;
        horizontal-stretch: 1;
        border-radius: 10px;
        background: root.selected ? Theme.raised-hover : (root.hovered ? Theme.raised-hover : Theme.raised);
        border-width: 1px;
        border-color: root.selected ? Theme.edge : Theme.hair;
        Rectangle {
            width: clamp(parent.width * 0.3, 32px, 72px);
            x: (parent.width - self.width) / 2;
            y: 5px;
            height: 3px;
            border-radius: 1.5px;
            background: Theme.amber;
            opacity: root.selected ? 1 : 0;
            animate opacity { duration: 150ms; easing: ease-out; }
        }
        VerticalLayout {
            padding: 12px;
            padding-top: 14px;
            spacing: 10px;
            Rectangle {
                height: 76px;
                border-radius: 6px;
                background: root.bar;
                clip: true;
                Image {
                    source: root.art.img;
                    width: min(root.art.w, parent.width - 12px);
                    height: min(root.art.h, parent.height);
                    image-fit: contain;
                }
            }
            HorizontalLayout {
                spacing: 8px;
                VerticalLayout {
                    spacing: 2px;
                    Text {
                        text: root.label;
                        font-size: 13px;
                        font-weight: 600;
                        color: Theme.text;
                    }
                    Text {
                        text: root.note;
                        font-size: 11px;
                        color: Theme.muted;
                        wrap: word-wrap;
                    }
                }
            }
        }
    }

    component Group inherits VerticalLayout {
        in property <string> title;
        spacing: 6px;
        Text {
            text: root.title;
            font-size: 13px;
            font-weight: 600;
            color: Theme.text;
            // A heading sits closer to its rows than to the group above.
            height: self.preferred-height + 4px;
            vertical-alignment: top;
        }
    }

    // A setting row across the page: what it is on the left, its control on the right.
    component Field inherits Rectangle {
        in property <string> label;
        in property <string> note;
        min-height: 54px;
        border-radius: 8px;
        background: Theme.raised;
        border-width: 1px;
        border-color: Theme.hair;
        HorizontalLayout {
            padding-left: 16px;
            padding-right: 12px;
            padding-top: 9px;
            padding-bottom: 9px;
            spacing: 20px;
            VerticalLayout {
                alignment: center;
                spacing: 1px;
                horizontal-stretch: 1;
                Text {
                    text: root.label;
                    font-size: 13px;
                    color: Theme.text;
                }
                if root.note != "": Text {
                    text: root.note;
                    font-size: 11px;
                    color: Theme.muted;
                    wrap: word-wrap;
                }
            }
            VerticalLayout {
                alignment: center;
                @children
            }
        }
    }

    component RailItem inherits Pressable {
        in property <image> icon;
        in property <string> name;
        in property <string> value;
        in property <bool> shown: true;
        in property <bool> selected;
        accessible-label: root.name;
        accessible-checkable: true;
        accessible-checked: root.selected;
        height: 38px;
        border-radius: 7px;
        background: root.selected ? Theme.raised : (root.hovered ? #FFFFFF0A : transparent);
        HorizontalLayout {
            padding-left: 10px;
            padding-right: 12px;
            spacing: 11px;
            VerticalLayout {
                alignment: center;
                Image {
                    source: root.icon;
                    width: 18px;
                    height: 18px;
                    colorize: root.selected ? Theme.amber : (root.shown ? Theme.text : Theme.muted);
                }
            }
            Text {
                text: root.name;
                vertical-alignment: center;
                font-size: 13px;
                font-weight: root.selected ? 600 : 400;
                color: root.shown ? Theme.text : Theme.muted;
                horizontal-stretch: 1;
            }
            Text {
                text: root.value;
                vertical-alignment: center;
                font-size: 12px;
                color: Theme.muted;
            }
        }
    }

    // A detail of a section: label above its control, set side by side to fill the row.
    component Detail inherits Rectangle {
        in property <string> label;
        in property <string> note;
        horizontal-stretch: 1;
        border-radius: 8px;
        background: Theme.raised;
        border-width: 1px;
        border-color: Theme.hair;
        VerticalLayout {
            padding: 12px;
            spacing: 10px;
            VerticalLayout {
                spacing: 1px;
                Text {
                    text: root.label;
                    font-size: 13px;
                    color: Theme.text;
                }
                Text {
                    text: root.note;
                    font-size: 11px;
                    color: Theme.muted;
                }
            }
            @children
        }
    }

    component SectionPage inherits VerticalLayout {
        in property <PageData> data;
        in property <[Art]> tiles;
        in property <color> bar;
        in-out property <int> style: Api.get(root.data.prefix + "Style");
        in-out property <bool> shown: Api.get(root.data.hide-key) == 0;
        alignment: start;
        spacing: 18px;
        padding-left: 28px;
        padding-right: 28px;
        padding-top: 20px;
        padding-bottom: 22px;
        HorizontalLayout {
            spacing: 14px;
            VerticalLayout {
                alignment: center;
                Image {
                    source: root.data.icon;
                    width: 28px;
                    height: 28px;
                    colorize: root.shown ? Theme.amber : Theme.muted;
                }
            }
            VerticalLayout {
                alignment: center;
                spacing: 2px;
                horizontal-stretch: 1;
                Text {
                    text: root.data.title;
                    font-family: "Segoe UI Variable Display";
                    font-size: 22px;
                    font-weight: 600;
                    color: Theme.text;
                }
                Text {
                    text: root.data.detail;
                    font-size: 12px;
                    color: Theme.muted;
                }
            }
        }
        Field {
            label: "Show on the taskbar";
            note: root.shown ? "Shown in the widget." : "Hidden. Its settings are kept for when you turn it back on.";
            Toggle {
                key: root.data.hide-key;
                invert: true;
                label: "Show " + root.data.title + " on the taskbar";
                on: root.shown;
                toggled => {
                    root.shown = self.on;
                }
            }
        }
        if root.data.chips.length > 0: Rectangle {
            opacity: root.shown ? 1 : 0.7;
            Group {
                title: root.data.chips-title;
                HorizontalLayout {
                    spacing: 8px;
                    for c in root.data.chips: ChipToggle {
                        data: c;
                    }
                }
            }
        }
        Rectangle {
            opacity: root.shown ? 1 : 0.7;
            Group {
            title: "Style";
            HorizontalLayout {
                spacing: 12px;
                StyleTile {
                    label: "Numbers";
                    note: "Compact and easiest to read.";
                    art: root.tiles[0];
                    bar: root.bar;
                    selected: root.style == 0;
                    clicked => {
                        root.style = 0;
                        Api.set(root.data.prefix + "Style", 0);
                    }
                }
                StyleTile {
                    label: "Rings";
                    note: "A ring beside each number.";
                    art: root.tiles[1];
                    bar: root.bar;
                    selected: root.style == 1;
                    clicked => {
                        root.style = 1;
                        Api.set(root.data.prefix + "Style", 1);
                    }
                }
                StyleTile {
                    label: "Numbers in rings";
                    note: "Most visual; best on a tall taskbar.";
                    art: root.tiles[2];
                    bar: root.bar;
                    selected: root.style == 2;
                    clicked => {
                        root.style = 2;
                        Api.set(root.data.prefix + "Style", 2);
                    }
                }
            }
            }
        }
        Rectangle {
            opacity: root.shown ? 1 : 0.7;
            Group {
                title: "Details";
                HorizontalLayout {
                    spacing: 8px;
                    if root.data.number-options.length > 0: Detail {
                        label: "Number shows";
                        note: root.data.number-note;
                        Segmented {
                            key: root.data.prefix + "ShowRemaining";
                            label: "Number shows";
                            options: root.data.number-options;
                        }
                    }
                    Detail {
                        label: "Icon";
                        note: root.data.icon-note;
                        Segmented {
                            key: root.data.icon-key;
                            label: "Icon";
                            options: root.data.icon-options;
                            icons: root.data.icon-images;
                        }
                    }
                    if root.data.sizes: Detail {
                        label: "Shows as";
                        note: "Size uses Explorer's units.";
                        Segmented {
                            key: "DrivesSize";
                            label: "Shows as";
                            options: ["Percent", "Size"];
                        }
                    }
                    if root.data.labels: Detail {
                        label: "Labels";
                        note: "5h, W and F before the numbers.";
                        Segmented {
                            key: root.data.prefix + "HideLabels";
                            label: "Labels";
                            options: ["Shown", "Hidden"];
                        }
                    }
                }
            }
        }
    }

    component SwitchField inherits Field {
        in property <string> key;
        Toggle {
            key: root.key;
            label: root.label;
        }
    }

    component GeneralPage inherits VerticalLayout {
        in property <image> icon;
        in property <[string]> font-sizes;
        in property <[string]> icon-sizes;
        in property <[string]> intervals;
        alignment: start;
        spacing: 18px;
        padding-left: 28px;
        padding-right: 28px;
        padding-top: 20px;
        padding-bottom: 22px;
        HorizontalLayout {
            spacing: 14px;
            VerticalLayout {
                alignment: center;
                Image {
                    source: root.icon;
                    width: 28px;
                    height: 28px;
                    colorize: Theme.amber;
                }
            }
            VerticalLayout {
                alignment: center;
                spacing: 2px;
                Text {
                    text: "General";
                    font-family: "Segoe UI Variable Display";
                    font-size: 22px;
                    font-weight: 600;
                    color: Theme.text;
                }
                Text {
                    text: "Applies to every section.";
                    font-size: 12px;
                    color: Theme.muted;
                }
            }
        }
        Group {
            title: "Size";
            Field {
                label: "Text";
                Segmented {
                    key: "FontSize";
                    label: "Text size";
                    options: root.font-sizes;
                }
            }
            Field {
                label: "Icons";
                Segmented {
                    key: "IconSize";
                    label: "Icon size";
                    options: root.icon-sizes;
                }
            }
        }
        Group {
            title: "Layout";
            Field {
                label: "Position";
                note: "Left end means the top on a vertical taskbar.";
                Segmented {
                    key: "OnLeft";
                    label: "Position";
                    options: ["Next to the tray", "Left end"];
                }
            }
            Field {
                label: "Accounts per column";
                note: "With more than one account.";
                Segmented {
                    key: "AccountsPerColumn";
                    label: "Accounts per column";
                    options: ["1", "2", "3"];
                }
            }
        }
        Group {
            title: "Updates";
            Field {
                label: "Check usage every";
                Segmented {
                    key: "IntervalSeconds";
                    label: "Check usage every";
                    options: root.intervals;
                }
            }
            SwitchField {
                label: "Roll numbers when they change";
                key: "RollNumbers";
            }
            SwitchField {
                label: "Start with Windows";
                note: "Launch Headroom when you sign in.";
                key: "Autostart";
            }
        }
    }

    export component SettingsWindow inherits Window {
        title: "Headroom settings";
        default-font-family: "Segoe UI Variable Text";
        icon: root.app-icon;
        preferred-width: 980px;
        preferred-height: 820px;
        min-width: 900px;
        min-height: 600px;
        background: Theme.ground;
        in property <image> app-icon;
        in property <[SectionItem]> sections;
        in property <image> gear;
        in property <image> preview;
        in property <length> preview-width;
        in property <length> preview-height;
        in property <[Spot]> spots;
        in property <[Ghost]> ghosts;
        in property <length> ceiling-x;
        in property <length> ceiling-w;
        in property <color> bar;
        in property <[PageData]> page-model;
        in property <[Art]> tiles;
        in property <[string]> font-sizes;
        in property <[string]> icon-sizes;
        in property <[string]> intervals;
        in property <int> page: 0;
        callback select(int);

        HorizontalLayout {
            // Section rail.
            Rectangle {
                width: 220px;
                background: Theme.deck;
                VerticalLayout {
                    padding: 14px;
                    padding-top: 18px;
                    spacing: 2px;
                    HorizontalLayout {
                        padding-left: 8px;
                        padding-bottom: 18px;
                        spacing: 10px;
                        Logo { }
                        Text {
                            text: "Headroom";
                            vertical-alignment: center;
                            font-size: 15px;
                            font-weight: 600;
                            color: Theme.text;
                        }
                    }
                    for s[i] in root.sections: RailItem {
                        icon: s.icon;
                        name: s.name;
                        value: s.value;
                        shown: s.shown;
                        selected: root.page == i;
                        clicked => {
                            root.select(i);
                        }
                    }
                    Rectangle {
                        height: 17px;
                        Rectangle {
                            height: 1px;
                            background: Theme.hair;
                        }
                    }
                    RailItem {
                        icon: root.gear;
                        name: "General";
                        selected: root.page == 5;
                        clicked => {
                            root.select(5);
                        }
                    }
                    Rectangle {
                        vertical-stretch: 1;
                    }
                    Text {
                        text: "Unofficial. Not affiliated with Anthropic or OpenAI.";
                        wrap: word-wrap;
                        font-size: 11px;
                        color: Theme.muted;
                    }
                }
            }
            Rectangle {
                width: 1px;
                background: Theme.hair;
            }
            VerticalLayout {
                horizontal-stretch: 1;
                // The widget, enlarged: click a section to edit it.
                Rectangle {
                    height: root.preview-height + 96px;
                    background: Theme.deck;
                    VerticalLayout {
                        alignment: center;
                        spacing: 12px;
                        HorizontalLayout {
                            alignment: center;
                            spacing: 10px;
                            strip := Rectangle {
                                width: root.preview-width + 24px + root.ghosts.length * 74px;
                                height: root.preview-height + 20px;
                                border-radius: 10px;
                                background: root.bar;
                                Image {
                                    x: 12px;
                                    y: 10px;
                                    source: root.preview;
                                    width: root.preview-width;
                                    height: root.preview-height;
                                }
                                for s in root.spots: Pressable {
                                    x: 12px + s.x;
                                    y: 6px;
                                    width: s.w;
                                    height: root.preview-height + 8px;
                                    border-radius: 6px;
                                    accessible-label: "Edit " + s.name;
                                    background: self.hovered && s.index != root.page ? #FFFFFF14 : transparent;
                                    clicked => {
                                        root.select(s.index);
                                    }
                                }
                                // Sections that are off wait at the end, dimmed; click one to turn it on.
                                for g[k] in root.ghosts: Pressable {
                                    x: 12px + root.preview-width + k * 74px;
                                    y: 6px;
                                    width: 70px;
                                    height: root.preview-height + 8px;
                                    border-radius: 6px;
                                    accessible-label: "Turn on " + g.name;
                                    background: self.hovered ? #FFFFFF14 : transparent;
                                    clicked => {
                                        Api.set("Section:" + g.index, 1);
                                    }
                                    Rectangle {
                                        x: 0px;
                                        y: (parent.height - self.height) / 2;
                                        width: 1px;
                                        height: 20px;
                                        background: #FFFFFF26;
                                    }
                                    VerticalLayout {
                                        alignment: center;
                                        spacing: 4px;
                                        HorizontalLayout {
                                            alignment: center;
                                            Image {
                                                source: g.icon;
                                                width: 18px;
                                                height: 18px;
                                                colorize: Theme.muted;
                                            }
                                        }
                                        Text {
                                            text: "+ " + g.name;
                                            horizontal-alignment: center;
                                            font-size: 11px;
                                            color: Theme.muted;
                                        }
                                    }
                                }
                                // The ceiling bar from the mark marks what you are editing.
                                Rectangle {
                                    x: 12px + root.ceiling-x + 8px;
                                    y: 3px;
                                    width: max(0px, root.ceiling-w - 16px);
                                    height: 3px;
                                    border-radius: 1.5px;
                                    background: Theme.amber;
                                    opacity: root.ceiling-w > 0 ? 1 : 0;
                                    animate x, width, opacity { duration: 180ms; easing: ease-out; }
                                }
                            }
                        }
                        Text {
                            text: root.ghosts.length > 0 ? "Click a section to edit it, or a dimmed one to turn it on. Claude and Codex show sample numbers." : "Click a section to edit it. Claude and Codex show sample numbers.";
                            horizontal-alignment: center;
                            font-size: 11px;
                            color: Theme.muted;
                        }
                    }
                }
                Rectangle {
                    height: 1px;
                    background: Theme.hair;
                }
                ScrollView {
                    vertical-stretch: 1;
                    content-width: self.visible-width;
                    VerticalLayout {
                        alignment: start;
                        for p in root.page-model: SectionPage {
                            data: p;
                            tiles: root.tiles;
                            bar: root.bar;
                        }
                        if root.page == 5: GeneralPage {
                            icon: root.gear;
                            font-sizes: root.font-sizes;
                            icon-sizes: root.icon-sizes;
                            intervals: root.intervals;
                        }
                    }
                }
            }
        }
    }
}

const TITLE: *const u16 = w!("Headroom settings");
/// The preview's widget is drawn larger than on the taskbar, up to this zoom.
const PREVIEW_ZOOM: f32 = 1.5;
/// Room for the preview strip in the window, logical pixels.
const PREVIEW_ROOM: f32 = 600.0;

pub fn run() {
    unsafe {
        // One window: a second "Settings…" brings the open one forward.
        CreateMutexW(null(), 0, w!("Local\\Headroom.Settings"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let open = FindWindowW(null(), TITLE);
            if !open.is_null() {
                ShowWindow(open, SW_RESTORE);
                SetForegroundWindow(open);
            }
            return;
        }
    }
    with(|s| {
        s.dark = system_dark();
        load_settings(s);
        // The preview never fetches: accounts show sample numbers.
        let sample = |p: f64| Some(api::Limit { percent: p, resets_at: None });
        for (i, (dir, kind)) in load_account_dirs().into_iter().enumerate() {
            let mut a = Account::new(i as u32, dir, kind);
            let codex = kind == Provider::Codex;
            a.usage = Some(api::Usage {
                five_hour: sample(if codex { 12.0 } else { 42.0 }),
                has_five_hour: true,
                weekly: sample(if codex { 31.0 } else { 78.0 }),
                fable: sample(18.0).filter(|_| !codex),
                from_log: false,
            });
            s.accounts.push(a);
        }
    });
    sample_ram();
    sample_cpu();
    sample_drives();

    let ui = SettingsWindow::new().expect("settings window");
    let strings = |v: Vec<&str>| ModelRc::new(VecModel::from(v.into_iter().map(SharedString::from).collect::<Vec<_>>()));
    ui.set_bar(if with(|s| s.dark) { slint::Color::from_rgb_u8(0x20, 0x20, 0x20) } else { slint::Color::from_rgb_u8(0xEE, 0xEE, 0xEE) });
    ui.set_gear(icon(render::Mark::None));
    ui.set_app_icon(logo(64));
    ui.set_font_sizes(strings(FONT_SIZES.iter().map(|f| f.1).collect()));
    ui.set_icon_sizes(strings(ICON_SIZES.iter().map(|f| f.1).collect()));
    ui.set_intervals(strings(INTERVALS.iter().map(|f| f.1.trim_end_matches("utes").trim_end_matches("ute")).collect()));

    let api = ui.global::<Api>();
    api.on_get(|key| ui_get(&key) as i32);
    let weak = ui.as_weak();
    api.on_set(move |key, v| {
        ui_set(&key, v.max(0) as u32);
        with(load_settings);
        unsafe {
            let widget = FindWindowW(w!("HeadroomCtl"), null());
            if !widget.is_null() {
                PostMessageW(widget, WM_APP_RELOAD, 0, 0);
            }
        }
        if let Some(ui) = weak.upgrade() {
            refresh(&ui);
            // A section shown again from its chip: open it.
            if let Some(i) = key.strip_prefix("Section:").and_then(|i| i.parse::<i32>().ok()) {
                select(&ui, i);
            }
        }
    });
    let weak = ui.as_weak();
    ui.on_select(move |i| {
        if let Some(ui) = weak.upgrade() {
            select(&ui, i);
        }
    });
    select(&ui, 0);
    when_shown(ui.as_weak(), 100);
    // Keep RAM and CPU moving, and follow changes made from the widget's menu.
    let weak = ui.as_weak();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(RAM_EVERY_MS as u64), move || {
        sample_cpu();
        sample_ram();
        let before = with(|s| s.hidden);
        with(load_settings);
        if let Some(ui) = weak.upgrade() {
            if with(|s| s.hidden) != before {
                select(&ui, ui.get_page()); // a page built on the old values
            } else {
                refresh(&ui);
            }
        }
    });
    ui.run().expect("settings window");
}

thread_local! {
    /// Section marks for the rail and the preview's off slots, drawn once.
    static SECTION_ICONS: Vec<Image> = (0..SECTIONS.len()).map(|i| icon(section_mark(i))).collect();
}

/// Puts `rows` into a window list in place when its length is unchanged, so the
/// items (and keyboard focus, hover) survive the 2-second refresh.
fn sync<T: Clone + 'static>(model: ModelRc<T>, rows: Vec<T>, set: impl FnOnce(ModelRc<T>)) {
    match model.as_any().downcast_ref::<VecModel<T>>() {
        Some(list) if list.row_count() == rows.len() => {
            for (i, row) in rows.into_iter().enumerate() {
                list.set_row_data(i, row);
            }
        }
        _ => set(ModelRc::new(VecModel::from(rows))),
    }
}

/// Once the window exists: give its title bar the window's charcoal instead of the
/// system grey, and redraw the previews at its real scale. Tries every 50 ms.
fn when_shown(weak: slint::Weak<SettingsWindow>, tries: u32) {
    slint::Timer::single_shot(std::time::Duration::from_millis(50), move || unsafe {
        use windows_sys::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR, DWMWA_USE_IMMERSIVE_DARK_MODE};
        use windows_sys::Win32::System::Threading::GetCurrentProcessId;
        let hwnd = FindWindowW(null(), TITLE);
        let mut pid = 0;
        if !hwnd.is_null() {
            GetWindowThreadProcessId(hwnd, &mut pid);
        }
        if hwnd.is_null() || pid != GetCurrentProcessId() {
            if tries > 0 {
                when_shown(weak, tries - 1);
            }
            return;
        }
        // COLORREF is 0x00BBGGRR.
        for (attr, v) in [(DWMWA_USE_IMMERSIVE_DARK_MODE, 1u32), (DWMWA_CAPTION_COLOR, 0x0017_191C), (DWMWA_TEXT_COLOR, 0x00F9_FAFA)] {
            DwmSetWindowAttribute(hwnd, attr as u32, &v as *const u32 as *const _, 4);
        }
        if let Some(ui) = weak.upgrade() {
            refresh(&ui);
        }
    });
}

/// Opens a section's page (0 to 4), or General (5).
fn select(ui: &SettingsWindow, i: i32) {
    ui.set_page(i);
    let page: Vec<PageData> = page_data(i as usize).into_iter().collect();
    ui.set_page_model(ModelRc::new(VecModel::from(page)));
    refresh(ui);
}

/// Redraws everything that follows the settings: the preview, the rail, the style tiles.
fn refresh(ui: &SettingsWindow) {
    let scale = ui.window().scale_factor();
    let page = ui.get_page();

    // The preview, zoomed to fit its room.
    let (models, dark) = with(|s| (build_models(s), s.dark));
    let draw = |zoom: f32| {
        let h = (48.0 * zoom * scale).round() as i32; // Windows 11's taskbar height
        let frames: Vec<render::Frame> = models.iter().map(|(m, _)| render::render(m, h, scale * zoom)).collect();
        (render::compose(&frames, h, scale * zoom, dark, false, false), h)
    };
    // Hidden sections' slots (74 px each) share the room with the widget.
    let room = PREVIEW_ROOM - 74.0 * with(|s| (0..SECTIONS.len()).filter(|&i| s.hidden[i]).count()) as f32;
    let ((mut frame, mut ends), _) = draw(PREVIEW_ZOOM);
    if frame.w as f32 / scale > room {
        ((frame, ends), _) = draw((PREVIEW_ZOOM * room * scale / frame.w as f32).max(0.6));
    }
    let px = |v: i32| v as f32 / scale;
    let mut spots = Vec::new();
    let mut left = 0;
    for ((_, index), end) in models.iter().zip(&ends) {
        let index = *index;
        spots.push(Spot { x: px(left), w: px(end - left), index: index as i32, name: SECTION_NAMES[index].into() });
        left = *end;
    }
    let ceiling = spots.iter().find(|s| s.index == page);
    ui.set_ceiling_x(ceiling.map_or(0.0, |s| s.x));
    ui.set_ceiling_w(ceiling.map_or(0.0, |s| s.w));
    ui.set_preview(image(&frame));
    ui.set_preview_width(px(frame.w));
    ui.set_preview_height(px(frame.h));
    sync(ui.get_spots(), spots, |m| ui.set_spots(m));

    // The rail: each section's name, what it reads now, and whether it's shown.
    let items: Vec<SectionItem> = with(|s| {
        let claude = s.accounts.iter().filter(|a| a.kind == Provider::Claude).count();
        let codex = s.accounts.len() - claude;
        let accounts = |n: usize| match n {
            0 => "No login".to_string(),
            1 => "1 account".to_string(),
            n => format!("{n} accounts"),
        };
        let pct = |v: Option<f64>| v.map_or("–".to_string(), |p| format!("{}%", p.round() as i64));
        let drives = s.drives.len();
        let values = [
            accounts(claude),
            accounts(codex),
            pct(s.ram.map(|r| r.0)),
            pct(s.cpu),
            format!("{drives} drive{}", if drives == 1 { "" } else { "s" }),
        ];
        (0..SECTIONS.len())
            .map(|i| {
                let shown = !s.hidden[i];
                SectionItem {
                    name: SECTION_NAMES[i].into(),
                    value: if shown { values[i].clone().into() } else { "Off".into() },
                    shown,
                    icon: SECTION_ICONS.with(|icons| icons[i].clone()),
                }
            })
            .collect()
    });
    let ghosts: Vec<Ghost> = items
        .iter()
        .enumerate()
        .filter(|(_, it)| !it.shown)
        .map(|(i, it)| Ghost { name: it.name.clone(), index: i as i32, icon: it.icon.clone() })
        .collect();
    sync(ui.get_sections(), items, |m| ui.set_sections(m));
    sync(ui.get_ghosts(), ghosts, |m| ui.set_ghosts(m));

    // The three style tiles: this section drawn plain, with rings, with numbers in rings.
    if (0..5).contains(&page) {
        let tiles: Vec<Art> = style_frames(page as usize, scale).iter().map(|f| Art { img: image(f), w: px(f.w), h: px(f.h) }).collect();
        ui.set_tiles(ModelRc::new(VecModel::from(tiles)));
    }
}

fn section_mark(i: usize) -> render::Mark {
    [render::Mark::Claude, render::Mark::Codex, render::Mark::Ram, render::Mark::Cpu, render::Mark::Drive][i]
}

/// Section `i` rendered in each of the three styles, at taskbar size, shown or not.
fn style_frames(i: usize, scale: f32) -> Vec<render::Frame> {
    let h = (48.0 * scale).round() as i32;
    with(|s| {
        let (look, hidden) = (s.looks[i], s.hidden[i]);
        s.hidden[i] = false;
        let mut out = Vec::new();
        for (rings, ring_text) in [(false, false), (true, false), (true, true)] {
            s.looks[i].rings = rings;
            s.looks[i].ring_text = ring_text;
            let mut frame = None;
            for (mut m, section) in build_models(s) {
                if section == i {
                    m.last = true;
                    frame = Some(render::render(&m, h, scale));
                }
            }
            out.push(frame.unwrap_or(render::Frame { w: 1, h: 1, px: vec![[0, 0, 0, 0]] }));
        }
        s.looks[i] = look;
        s.hidden[i] = hidden;
        out
    })
}

/// The page for section `i`, or nothing for General.
fn page_data(i: usize) -> Option<PageData> {
    let strings = |v: &[&str]| ModelRc::new(VecModel::from(v.iter().map(|s| SharedString::from(*s)).collect::<Vec<_>>()));
    let chip = |label: &str, detail: String, key: &str, invert: bool| Chip { label: label.into(), detail: detail.into(), key: key.into(), invert };
    let chips: Vec<Chip> = match i {
        CLAUDE => vec![
            chip("5-hour", "Rolling session".into(), "HideFiveHour", true),
            chip("Weekly", "All models".into(), "HideWeekly", true),
            chip("Fable", "On plans that have it".into(), "HideFable", true),
        ],
        CODEX => vec![chip("5-hour", "Rolling session".into(), "CodexHideFiveHour", true), chip("Weekly", "All models".into(), "CodexHideWeekly", true)],
        DRIVES => with(|s| {
            s.drives
                .iter()
                .map(|&(n, used, total)| {
                    let l = DRIVE_LETTERS[n as usize];
                    let detail = format!("{} free of {}", size_text(total - used), size_text(total));
                    Chip { label: format!("{l}:").into(), detail: detail.into(), key: format!("Drive:{l}").into(), invert: false }
                })
                .collect()
        }),
        _ => Vec::new(),
    };
    let (title, detail, chips_title) = match i {
        CLAUDE => ("Claude Code", "Usage limits from your Claude Code login. Click opens Claude's usage page.", "Limits"),
        CODEX => ("Codex", "Usage limits from your Codex CLI login. Click opens Codex's usage page.", "Limits"),
        RAM => ("RAM", "Memory in use, every 2 seconds. Click opens Task Manager.", ""),
        CPU => ("CPU", "Load over the last 2 seconds, all cores. Click opens Task Manager.", ""),
        DRIVES => ("Drives", "Space used on internal drives, every minute. Click opens This PC.", "Drives (up to three fit)"),
        _ => return None,
    };
    let number: &[&str] = match i {
        CLAUDE | CODEX => &["Used", "Remaining"],
        DRIVES => &["Used", "Free"],
        _ => &[],
    };
    let (icon_key, icon_options): (String, &[&str]) =
        if i == CLAUDE { ("ClaudeIcon".into(), &["Asterisk", "Pixel", "None"]) } else { (format!("{}HideIcon", SECTIONS[i]), &["Shown", "Hidden"]) };
    // Each icon choice drawn as the mark it gives; "None" and "Hidden" draw nothing.
    let icon_images: Vec<Image> = if i == CLAUDE {
        vec![icon(render::Mark::Claude), icon(render::Mark::Pixel), Image::default()]
    } else {
        vec![icon(section_mark(i)), Image::default()]
    };
    let number_note = match i {
        DRIVES => "Free shows the space left.",
        _ => "Remaining counts down to the limit.",
    };
    let icon_note = match i {
        CLAUDE | CODEX | DRIVES => "Before the numbers.",
        _ => "Without it, a label names the number.",
    };
    Some(PageData {
        index: i as i32,
        prefix: SECTIONS[i].into(),
        title: title.into(),
        detail: detail.into(),
        hide_key: SECTION_HIDE_KEYS[i].into(),
        icon: icon(section_mark(i)),
        chips_title: chips_title.into(),
        chips: ModelRc::new(VecModel::from(chips)),
        number_options: strings(number),
        number_note: number_note.into(),
        icon_key: icon_key.into(),
        icon_options: strings(icon_options),
        icon_images: ModelRc::new(VecModel::from(icon_images)),
        icon_note: icon_note.into(),
        sizes: i == DRIVES,
        labels: matches!(i, CLAUDE | CODEX),
    })
}

/// A section mark as a white image the window tints.
fn icon(mark: render::Mark) -> Image {
    const SIZE: i32 = 56; // drawn at 28 or 18 logical px, crisp to 200% scaling
    let cov = render::mark_mask(mark, SIZE);
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(SIZE as u32, SIZE as u32);
    for (d, c) in buf.make_mut_slice().iter_mut().zip(cov) {
        let a = (c.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        *d = Rgba8Pixel { r: a, g: a, b: a, a };
    }
    Image::from_rgba8_premultiplied(buf)
}

/// Headroom's mark (assets/logo.svg: three meters under a ceiling), drawn at `size` pixels.
fn logo(size: u32) -> Image {
    // Rounded rects on the 16-unit grid: (x, y, w, h, radius, colour), back to front.
    const SHAPES: [(f32, f32, f32, f32, f32, [u8; 3]); 6] = [
        (0.0, 0.0, 16.0, 16.0, 3.5, [0x57, 0x53, 0x4E]), // edge
        (0.5, 0.5, 15.0, 15.0, 3.0, [0x1C, 0x19, 0x17]), // tile
        (3.0, 3.0, 10.0, 1.0, 0.5, [0xFA, 0xFA, 0xF9]),  // ceiling
        (3.0, 9.0, 2.0, 4.0, 0.4, [0xF5, 0x9E, 0x0B]),   // meters
        (7.0, 6.0, 2.0, 7.0, 0.4, [0xF5, 0x9E, 0x0B]),
        (11.0, 8.0, 2.0, 5.0, 0.4, [0xF5, 0x9E, 0x0B]),
    ];
    let k = size as f32 / 16.0;
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(size, size);
    for (n, d) in buf.make_mut_slice().iter_mut().enumerate() {
        let (x, y) = ((n as u32 % size) as f32 + 0.5, (n as u32 / size) as f32 + 0.5);
        let mut c = [0f32; 4]; // premultiplied
        for &(l, t, w, h, rad, rgb) in &SHAPES {
            let a = render::rounded_rect_cov(x, y, l * k, t * k, (l + w) * k, (t + h) * k, rad * k);
            for (ch, v) in c.iter_mut().zip([rgb[0] as f32, rgb[1] as f32, rgb[2] as f32, 255.0]) {
                *ch = v * a + *ch * (1.0 - a);
            }
        }
        let q = |v: f32| v.round().clamp(0.0, 255.0) as u8;
        *d = Rgba8Pixel { r: q(c[0]), g: q(c[1]), b: q(c[2]), a: q(c[3]) };
    }
    Image::from_rgba8_premultiplied(buf)
}

fn image(f: &render::Frame) -> Image {
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(f.w as u32, f.h as u32);
    for (d, p) in buf.make_mut_slice().iter_mut().zip(&f.px) {
        *d = Rgba8Pixel { r: p[2], g: p[1], b: p[0], a: p[3] };
    }
    Image::from_rgba8_premultiplied(buf)
}

/// A setting as the window shows it. Most are registry values as they are; segmented
/// controls hold an index, drives a "shown" tick, a style 0 to 2, the Claude icon a
/// choice of three.
fn ui_get(key: &str) -> u32 {
    let index = |v: u32, list: &[(u32, &str)]| list.iter().position(|(x, _)| *x == v).unwrap_or(0) as u32;
    if let Some(p) = key.strip_suffix("Style") {
        return match (setting(&format!("{p}Rings")), setting(&format!("{p}RingText"))) {
            (1, 1) => 2,
            (1, _) => 1,
            _ => 0,
        };
    }
    match key {
        "Autostart" => autostart_enabled() as u32,
        "ClaudeIcon" => if setting("ClaudeHideIcon") == 1 { 2 } else { setting("PixelIcon") },
        "FontSize" => index(with(|s| s.font_size), &FONT_SIZES),
        "IconSize" => index(with(|s| s.icon_size), &ICON_SIZES),
        "IntervalSeconds" => index(with(|s| s.interval), &INTERVALS),
        "AccountsPerColumn" => with(|s| s.rows_per_col) - 1,
        k => match drive_bit(k) {
            Some(bit) => (setting("HiddenDrives") & bit == 0) as u32,
            None => setting(k),
        },
    }
}

fn ui_set(key: &str, v: u32) {
    let at = |list: &[(u32, &str)]| list[(v as usize).min(list.len() - 1)].0;
    if let Some(p) = key.strip_suffix("Style") {
        write_setting(&format!("{p}Rings"), (v > 0) as u32);
        write_setting(&format!("{p}RingText"), (v == 2) as u32);
        return;
    }
    if let Some(i) = key.strip_prefix("Section:").and_then(|i| i.parse::<usize>().ok()) {
        // "+ CPU": show a hidden section again.
        write_setting(SECTION_HIDE_KEYS[i.min(SECTIONS.len() - 1)], 0);
        return;
    }
    match key {
        "Autostart" => set_autostart(v == 1),
        "ClaudeIcon" => {
            write_setting("ClaudeHideIcon", (v == 2) as u32);
            if v < 2 {
                write_setting("PixelIcon", v);
            }
        }
        "FontSize" => write_setting("FontSize", at(&FONT_SIZES)),
        "IconSize" => write_setting("IconSize", at(&ICON_SIZES)),
        "IntervalSeconds" => write_setting("IntervalSeconds", at(&INTERVALS)),
        "AccountsPerColumn" => write_setting("AccountsPerColumn", v + 1),
        k => match drive_bit(k) {
            Some(bit) => {
                let mask = setting("HiddenDrives");
                write_setting("HiddenDrives", if v == 1 { mask & !bit } else { mask | bit });
            }
            None => write_setting(k, v),
        },
    }
}

/// "Drive:D" → D's bit in HiddenDrives.
fn drive_bit(key: &str) -> Option<u32> {
    let letter = key.strip_prefix("Drive:")?.bytes().next()?;
    letter.is_ascii_uppercase().then(|| 1 << (letter - b'A'))
}
