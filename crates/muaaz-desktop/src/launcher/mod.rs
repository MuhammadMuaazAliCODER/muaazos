//! The application launcher.
//!
//! Presents a searchable list of registered applications inside a
//! popover. Keyboard navigation (arrow keys, Enter, Escape) is handled
//! automatically by `gtk::ListBox` and `gtk::Popover`.

pub mod launcher;

pub use launcher::Launcher;