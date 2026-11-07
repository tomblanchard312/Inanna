//! Inanna’s palette and measurements for the Eridu interface.

use egui::Color32;

pub(super) struct Palette {
    pub dark: bool,
    pub border: Color32,
    pub text: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub surface_subtle: Color32,
    pub text_muted: Color32,
    pub surface_hover: Color32,
    pub selected: Color32,
    pub accent: Color32,
    pub accent_text: Color32,
    pub primary: Color32,
    pub on_primary: Color32,
    pub focus: Color32,
    pub danger: Color32,
    pub warning: Color32,
}

const fn rgb(value: u32) -> Color32 {
    let [_, red, green, blue] = value.to_be_bytes();
    Color32::from_rgb(red, green, blue)
}

pub(super) const LIGHT: Palette = Palette {
    dark: false,
    border: rgb(0xC7CDD0),
    text: rgb(0x202C33),
    background: rgb(0xFAF9F6),
    surface: rgb(0xFFFFFF),
    surface_subtle: rgb(0xF1EEE8),
    text_muted: rgb(0x475962),
    surface_hover: rgb(0xEEF3F4),
    selected: rgb(0xE3EEF1),
    accent: rgb(0x246B83),
    accent_text: rgb(0xFFFFFF),
    primary: rgb(0x94592C),
    on_primary: rgb(0xFFFFFF),
    focus: rgb(0x246B83),
    danger: rgb(0xA32929),
    warning: rgb(0x8A5A00),
};

pub(super) const DARK: Palette = Palette {
    dark: true,
    border: rgb(0x71818A),
    text: rgb(0xF7F7F4),
    background: rgb(0x17232B),
    surface: rgb(0x202C33),
    surface_subtle: rgb(0x293A43),
    text_muted: rgb(0xD0D9DE),
    surface_hover: rgb(0x253740),
    selected: rgb(0x30505F),
    accent: rgb(0x83C9E0),
    accent_text: rgb(0x10232D),
    primary: rgb(0xE0A472),
    on_primary: rgb(0x10232D),
    focus: rgb(0x83C9E0),
    danger: rgb(0xFF9D99),
    warning: rgb(0xF2C46B),
};

pub(crate) mod space {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    #[cfg(not(target_arch = "wasm32"))]
    pub const MD: f32 = 12.0;
}

pub(crate) mod size {
    pub const CAPTION: f32 = 12.0;
    pub const BODY: f32 = 14.0;
    #[cfg(not(target_arch = "wasm32"))]
    pub const CARD_HEADING: f32 = 16.0;
    pub const SECTION_HEADING: f32 = 20.0;
}

pub(crate) mod radius {
    pub const SMALL: u8 = 4;
    pub const MEDIUM: u8 = 8;
    pub const LARGE: u8 = 12;
}

pub(super) fn prefers_dark(ctx: &egui::Context) -> bool {
    ctx.system_theme().map_or_else(|| portal_dark(ctx), |theme| theme == egui::Theme::Dark)
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn portal_dark(_: &egui::Context) -> bool {
    false
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn portal_dark(ctx: &egui::Context) -> bool {
    use std::sync::Once;
    use std::sync::atomic::{AtomicBool, Ordering};
    static START: Once = Once::new();
    static DARK: AtomicBool = AtomicBool::new(false);
    START.call_once(|| {
        let ctx = ctx.clone();
        let _ = std::thread::Builder::new().name("inanna-desktop-theme".into()).spawn(move || {
            let _ = watch_desktop_scheme(|dark| {
                if DARK.swap(dark, Ordering::Relaxed) != dark {
                    ctx.request_repaint();
                }
            });
        });
    });
    DARK.load(Ordering::Relaxed)
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn watch_desktop_scheme(update: impl Fn(bool)) -> zbus::Result<()> {
    use zbus::blocking::{Connection, Proxy};
    use zbus::zvariant::{OwnedValue, Value};

    const APPEARANCE: &str = "org.freedesktop.appearance";
    const SCHEME: &str = "color-scheme";
    fn dark(mut value: &Value<'_>) -> bool {
        while let Value::Value(inner) = value {
            value = inner;
        }
        matches!(value, Value::U32(1))
    }

    let bus = Connection::session()?;
    let desktop = Proxy::new(&bus, "org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop", "org.freedesktop.portal.Settings")?;
    let events = desktop.receive_signal("SettingChanged")?;
    let key = (APPEARANCE, SCHEME);
    let initial: zbus::Result<OwnedValue> = desktop.call("ReadOne", &key).or_else(|_| desktop.call("Read", &key));
    if let Ok(value) = initial {
        update(dark(&value));
    }
    for event in events {
        if let Ok((APPEARANCE, SCHEME, value)) = event.body().deserialize::<(&str, &str, Value<'_>)>() {
            update(dark(&value));
        }
    }
    Ok(())
}
