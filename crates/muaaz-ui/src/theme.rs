//! The Muaaz OS color palette.
//!
//! All colors are dark-first. UI code should reference these constants
//! rather than inventing values inline.

/// Base window / desktop background.
pub const BG_BASE: &str = "#0f1115";
/// Slightly raised background for headers and panels.
pub const BG_RAISED: &str = "#161a20";
/// Card / widget surface background.
pub const BG_SURFACE: &str = "#1d232b";
/// Surface in its hover state.
pub const BG_SURFACE_HOVER: &str = "#242c36";
/// Surface in its pressed/active state.
pub const BG_SURFACE_ACTIVE: &str = "#2c3644";
/// Emphasised highlight surface (selected items).
pub const BG_HIGHLIGHT: &str = "#232f3f";

/// Primary brand accent.
pub const ACCENT: &str = "#4da3ff";
/// Accent in its hover state.
pub const ACCENT_HOVER: &str = "#6fb6ff";
/// Accent in its pressed/active state.
pub const ACCENT_ACTIVE: &str = "#2f8bf9";
/// Subtle accent-tinted background for badges and highlights.
pub const ACCENT_SOFT: &str = "#10233d";

/// Primary text color.
pub const TEXT_PRIMARY: &str = "#e8edf2";
/// Secondary text color (descriptions, metadata).
pub const TEXT_SECONDARY: &str = "#9aa4b2";
/// Muted / tertiary text color (placeholders, hints).
pub const TEXT_MUTED: &str = "#5c6875";

/// Default widget border.
pub const BORDER: &str = "#2a323d";
/// Fainter border for subtle dividers.
pub const BORDER_SUBTLE: &str = "#222933";

/// Semantic status colors.
pub const STATUS_SUCCESS: &str = "#46c384";
pub const STATUS_WARNING: &str = "#e0b028";
pub const STATUS_ERROR: &str = "#f2645a";

/// Focus ring color.
pub const FOCUS: &str = "#4da3ff";

/// The complete set of named colors in the palette.
pub const ALL: &[(&str, &str)] = &[
    ("bg_base", BG_BASE),
    ("bg_raised", BG_RAISED),
    ("bg_surface", BG_SURFACE),
    ("bg_surface_hover", BG_SURFACE_HOVER),
    ("bg_surface_active", BG_SURFACE_ACTIVE),
    ("bg_highlight", BG_HIGHLIGHT),
    ("accent", ACCENT),
    ("accent_hover", ACCENT_HOVER),
    ("accent_active", ACCENT_ACTIVE),
    ("accent_soft", ACCENT_SOFT),
    ("text_primary", TEXT_PRIMARY),
    ("text_secondary", TEXT_SECONDARY),
    ("text_muted", TEXT_MUTED),
    ("border", BORDER),
    ("border_subtle", BORDER_SUBTLE),
    ("status_success", STATUS_SUCCESS),
    ("status_warning", STATUS_WARNING),
    ("status_error", STATUS_ERROR),
    ("focus", FOCUS),
];

/// The Muaaz OS brand identity.
pub mod brand {
    /// Human-readable product name.
    pub const NAME: &str = "Muaaz OS";
    /// Product tagline.
    pub const TAGLINE: &str = "Build. Create. Explore.";
    /// Reverse-domain application id.
    pub const APP_ID: &str = "org.muaaz.desktop";
    /// Early desktop version.
    pub const VERSION: &str = "0.1";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_categories_are_unique_names() {
        let mut names: Vec<&str> = ALL.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ALL.len(), "duplicate palette names");
    }

    #[test]
    fn palette_colors_are_valid_hex() {
        for (_, color) in ALL {
            assert_eq!(color.len(), 7, "color {color} should be #rrggbb");
            assert!(color.starts_with('#'));
        }
    }
}