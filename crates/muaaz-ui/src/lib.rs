//! Reusable Muaaz UI design system.
//!
//! This crate centralises the visual identity of Muaaz OS: colors,
//! spacing, typography, and the widget CSS classes used across the
//! desktop and (in the future) every Muaaz application.
//!
//! The rule is simple: UI code never hard-codes colors or spacing.
//! It references the palette and spacing constants defined here, and
//! widgets carry semantic CSS class names (`.muaaz-*`) that map to
//! this theme.

pub mod spacing;
pub mod style;
pub mod theme;
pub mod widgets;

pub use spacing::Spacing;
pub use style::MuaazCss;
pub use theme::*;
pub use widgets::*;

/// Loads the Muaaz stylesheet into a newly created `CssProvider` and
/// adds it to the default screen.
///
/// Returns the provider so the caller can keep it alive for the
/// lifetime of the application.
pub fn load_css() -> Result<gtk::CssProvider, gtk::glib::Error> {
    let provider = gtk::CssProvider::new();
    let css = MuaazCss::as_owned_string();
    provider.load_from_data(&css);
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("a display must exist"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    Ok(provider)
}