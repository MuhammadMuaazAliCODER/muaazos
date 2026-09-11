//! The Muaaz OS central stylesheet.
//!
//! The stylesheet is generated from the [`crate::theme`] palette so that
//! the named colors defined in Rust and the CSS `@define-color` variables
//! never drift apart. The prose of the stylesheet lives in
//! `src/style.css` and references those variables by name.

use crate::theme;

/// Builds the full Muaaz stylesheet as a string.
///
/// This is produced once at startup and handed to GTK. Keeping it as a
/// single block makes the theme auditable and easy to override later.
pub struct MuaazCss;

impl MuaazCss {
    /// Returns the complete stylesheet text: palette definitions followed
    /// by the widget stylesheet.
    pub fn as_owned_string() -> String {
        let mut css = palette_definitions();
        css.push_str(include_str!("style.css"));
        css
    }
}

/// Definitions of the `@define-color` variables used by the stylesheet.
///
/// Each palette entry is emitted as a GTK color variable. Widgets reference
/// these names (e.g. `@muaaz_accent`) instead of raw hex values.
pub fn palette_definitions() -> String {
    let mut out = String::from("/* Generated from crate::theme — keep colors in sync */\n");
    for (name, value) in theme::ALL {
        out.push_str(&format!("@define-color muaaz_{name} {value};\n"));
    }
    out
}

/// A small typed wrapper over a palette color so UI code can ask for a color
/// without hard-coding hex values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedColor(pub &'static str);

impl NamedColor {
    /// Primary brand accent.
    pub const ACCENT: Self = Self(theme::ACCENT);
    /// Primary text color.
    pub const TEXT_PRIMARY: Self = Self(theme::TEXT_PRIMARY);
    /// Secondary text color.
    pub const TEXT_SECONDARY: Self = Self(theme::TEXT_SECONDARY);
    /// Surface background.
    pub const SURFACE: Self = Self(theme::BG_SURFACE);

    /// Converts this color into a GTK `RGBA` value.
    pub fn to_rgba(self) -> gtk::gdk::RGBA {
        gtk::gdk::RGBA::parse(self.0).expect("palette colors are valid hex")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_colors_parse() {
        for color in [
            NamedColor::ACCENT,
            NamedColor::TEXT_PRIMARY,
            NamedColor::TEXT_SECONDARY,
            NamedColor::SURFACE,
        ] {
            let _ = color.to_rgba();
        }
    }

    #[test]
    fn palette_definitions_contains_all_entries() {
        let defs = palette_definitions();
        for (name, _) in theme::ALL {
            assert!(
                defs.contains(&format!("muaaz_{name}")),
                "missing definition for {name}"
            );
        }
    }
}