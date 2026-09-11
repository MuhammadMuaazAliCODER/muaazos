//! Welcome / hero section and app activation logic.
//!
//! The welcome section is the default body content of the desktop:
//! a hero area with Muaaz OS branding and a grid of application
//! shortcuts.
//!
//! Application activation (what happens when a shortcut or launcher row
//! is clicked) is handled here so the rest of the desktop shell does not
//! need to know about launch strategies.

pub mod welcome;

pub use welcome::{about_dialog, handle_activation, Welcome};