//! Muaaz Desktop — the desktop shell of Muaaz OS.
//!
//! This binary is the entry point of the desktop experience. The
//! implementation is split into focused modules so the shell can grow
//! (panel, launcher, notifications, workspace) without becoming a
//! monolithic file:
//!
//! ```text
//! main.rs      entry point + error handling
//! app.rs       the GtkApplication and its actions
//! config.rs    desktop configuration
//! window.rs    the main window and its layout
//! panel/       top system panel (launcher, clock, status)
//! launcher/    application launcher
//! welcome/     welcome / hero section
//! ```

mod app;
mod config;
mod launcher;
mod panel;
mod welcome;
mod window;

fn main() -> std::process::ExitCode {
    // A display server must be present. On a headless machine we print a
    // clear message instead of panicking.
    if gtk::gdk::Display::default().is_none() {
        eprintln!("Muaaz Desktop requires a display server (Wayland or X11).");
        return std::process::ExitCode::FAILURE;
    }

    let application = app::MuaazDesktop::new();
    application.run()
}