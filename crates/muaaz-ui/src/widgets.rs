//! Reusable widget helpers built on the Muaaz design system.
//!
//! These functions create standard GTK widgets pre-wired with the Muaaz
//! CSS classes so callers stay consistent without duplicating styling.

use gtk::prelude::*;

/// Creates a brand-mark label (the "MUAaz OS" wordmark).
pub fn brand_label() -> gtk::Label {
    let label = gtk::Label::new(Some(crate::theme::brand::NAME));
    label.add_css_class("muaaz-brand-mark");
    label
}

/// Creates a section kicker label (the small uppercase-ish accent label
/// used above section titles).
pub fn section_kicker(text: &str) -> gtk::Label {
    let label = gtk::Label::new(Some(text));
    label.add_css_class("muaaz-section-kicker");
    label.set_opacity(0.0);
    label
}

/// Creates a primary (accent) action button.
pub fn accent_button(label: &str) -> gtk::Button {
    let button = gtk::Button::with_label(label);
    button.add_css_class("muaaz-accent-button");
    button
}

/// Creates a secondary (outline) action button.
pub fn outline_button(label: &str) -> gtk::Button {
    let button = gtk::Button::with_label(label);
    button.add_css_class("muaaz-outline-button");
    button
}

/// Creates a card container with the given content inside.
pub fn card(content: &impl IsA<gtk::Widget>) -> gtk::Box {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
    card.add_css_class("muaaz-card");
    card.append(content);
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widgets_carry_css_classes() {
        assert!(brand_label().has_css_class("muaaz-brand-mark"));
        assert!(accent_button("x").has_css_class("muaaz-accent-button"));
        assert!(outline_button("x").has_css_class("muaaz-outline-button"));
    }
}