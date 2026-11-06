# Inanna interface brief

Inanna is an EriduOS presentation workspace for people creating, editing and presenting slides. The slide canvas is the visual anchor. Slide thumbnails provide stable navigation on the left, the ribbon groups editing commands above the canvas, and the optional inspector stays beside the selection. File opens the backstage workspace for creating, opening and saving presentations.

On first run, the File workspace offers a blank presentation and recent files. In a populated workspace, the active slide, selected ribbon tab and Present action identify the current task. Open and save errors use the existing app feedback and recovery paths. Users can continue editing when Scribe or Voice cannot connect. Scribe uses local Enki and presents changes for review and explicit application. Its existing service errors identify the missing local runtime or model.

The GUI consumes semantic colors and base sizing from the pinned shared Eridu style library. Present uses its copper action tokens. The initial appearance follows the desktop, with light as the fallback. Users can select System, Light or Dark in File → Account. Preferences retains keyboard access, a visible interface size label and scaling through 200%. The ribbon keeps its scroll controls, shortcut names and accessible labels. The existing Inter and multilingual font fallback pipeline serves the interface. Slide content uses document fonts.

Linux is the primary validation platform. FreeBSD is a secondary target. Windows, macOS and the browser are existing build targets with separate runtime acceptance. Illumos remains planned.

[Eridu interface requirements](https://github.com/tomblanchard312/EriduOs/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/INTERFACE_RULES.md) · [Style guide](https://github.com/tomblanchard312/EriduOs/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/STYLE_GUIDE.md) · [Adoption checklist](https://github.com/tomblanchard312/EriduOs/blob/53133f86434676a5bdcb249556d3c11b50e600e4/design-system/ADOPTION.md)

## Review evidence

All nine local workspace gates passed. Automated checks cover the reported
desktop theme and manual overrides, Preferences cancellation, text and action
contrast, focus color contrast, and ribbon visibility at the tested widths.
The browser entry point compiled for WebAssembly. The light and dark editor
screenshots were inspected at 1440 × 900 using the headless test harness.

## Adoption still to verify

| Scope | Owner | Reason and follow-up |
|---|---|---|
| Phosphor and typography roles | Inanna maintainers | The existing icon set and compact ribbon metrics remain. The shared font installer bundles Source Serif, which Inanna's asset policy excludes. Selective icon and font installation, tabular figures, and the other type roles need a follow-up. [Eridu typography tracking](https://github.com/tomblanchard312/EriduOs/issues/27) |
| Native accessibility and workflow acceptance | Inanna maintainers | Test live KDE theme changes, screen readers, 200% scaling, reduced motion, localized text expansion, and FreeBSD installation. Track findings using the adoption checklist linked above. |
