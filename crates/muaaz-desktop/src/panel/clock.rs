//! The panel clock.
//!
//! Updates every second without ever blocking the main loop: glib's
//! timeout runs on the local (main) thread but never sleeps — it yields
//! control between frames, which keeps the rest of the UI responsive.

use gtk::glib;
use gtk::prelude::*;
use std::time::Duration;

/// A label showing the current local time.
pub struct Clock {
    label: gtk::Label,
    /// Keeps the periodic source alive for the lifetime of the clock.
    _source: glib::SourceId,
}

impl Clock {
    /// Creates a clock that refreshes every `interval_secs` seconds.
    pub fn new(interval_secs: u32) -> Self {
        let label = gtk::Label::new(None);
        label.add_css_class("muaaz-clock");
        update_label(&label);

        let label_for_timer = label.clone();
        let source = glib::timeout_add_local(Duration::from_secs(u64::from(interval_secs)), move || {
            update_label(&label_for_timer);
            glib::ControlFlow::Continue
        });

        Clock { label, _source: source }
    }

    /// The label widget to embed in the panel.
    pub fn widget(&self) -> &gtk::Label {
        &self.label
    }
}

/// Fills `label` with the current time, degrading to an empty string if the
/// system clock cannot be read.
fn update_label(label: &gtk::Label) {
    if let Ok(now) = glib::DateTime::now_local() {
        if let (Ok(text), Ok(tooltip)) = (now.format("%H:%M"), now.format("%A %e %B %Y, %H:%M:%S")) {
            label.set_label(&text);
            label.set_tooltip_text(Some(&tooltip));
        }
    }
}