# Inanna branding and provenance

Status: initial documentation branding, 2026-10-30.

- Upstream: https://github.com/storytold/deckcraft
- Fork: https://github.com/tomblanchard312/Inanna
- Initial upstream commit: `3a4e30e10e83c5f988a074dba1a519984828b4fd`
- Identity: Inanna, an independent presentation editor based on DeckCraft by the ArtCraft team.
- Direction: dark surfaces, blue accents, readable controls, generous spacing and clear sidebar navigation. Retain Rust and egui/eframe; no Tk interface or rewrite.

This change replaces the repository landing page and removes the eight ArtCraft trademark graphics from the current source tree. LICENSE-MIT, LICENSE-APACHE, NOTICE, third-party notices and the brand terms are retained. Attribution rows for the removed graphics are removed; historical authorship stays in Git. No third-party artwork or new logo is added.

This is not a completed application rebrand. Existing app icons are openly licensed DeckCraft artwork and remain attributed. Runtime titles, About and community links, installers, desktop metadata, screenshots, update destinations and storage identifiers still require a coordinated review before any Inanna distribution. Do not publish inherited signed-build claims as Inanna claims. Retain copyright credits when changing About.

Keep package names, command IDs, file extensions and settings paths stable until a tested compatibility/migration plan exists. Do not redirect an Inanna updater to upstream binaries. Release branding must cover all desktop, CLI, web and installer surfaces together.

## Upstream synchronization

Keep `upstream` pointed at DeckCraft and `origin` at Inanna. Fetch upstream changes, integrate on a feature branch, rerun checks and review a PR against the fork default branch. Never rewrite inherited history or automatically merge. Do not restore restricted marks during synchronization. The original upstream roadmap and technical documentation remain as baseline material, not Inanna completion claims.
