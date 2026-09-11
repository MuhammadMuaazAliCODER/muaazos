//! The top system panel widget.
//!
//! ```text
//! [Muaaz OS]  [Launcher]    ...    [CPU ..%] [RAM ..GiB] [..%] | [14:03]
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use muaaz_system::MuaazSystem;
use muaaz_ui as ui;

use super::clock::Clock;
use super::status::Status;
use crate::config::DesktopConfig;

/// The fixed-height panel pinned to the top of the desktop window.
pub struct Panel {
    container: gtk::CenterBox,
    /// The menu button that opens the launcher popover.
    pub launcher_button: gtk::MenuButton,
    _clock: Clock,
    _status: Status,
}

impl Panel {
    /// Builds the panel.
    pub fn new(system: Rc<RefCell<MuaazSystem>>, config: &DesktopConfig) -> Self {
        let container = gtk::CenterBox::new();
        container.add_css_class("muaaz-panel");

        // --- Left: brand + launcher --------------------------------
        let start = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        start.set_halign(gtk::Align::Start);

        start.append(&ui::brand_label());

        let launcher_button = gtk::MenuButton::builder()
            .icon_name("view-app-grid-symbolic")
            .tooltip_text("Applications  (Ctrl+Space)")
            .css_classes(["muaaz-panel-button"])
            .build();
        start.append(&launcher_button);

        container.set_start_widget(Some(&start));

        // --- Right: status + clock ---------------------------------
        let end = gtk::Box::new(gtk::Orientation::Horizontal, 16);
        end.set_halign(gtk::Align::End);

        let status = Status::new(Rc::clone(&system), config.status_interval_secs);
        end.append(status.widget());

        let sep = gtk::Separator::new(gtk::Orientation::Vertical);
        sep.add_css_class("muaaz-panel-separator");
        end.append(&sep);

        let clock = Clock::new(config.clock_interval_secs);
        end.append(clock.widget());

        container.set_end_widget(Some(&end));

        Panel { container, launcher_button, _clock: clock, _status: status }
    }

    /// The panel widget to add to the window layout.
    pub fn widget(&self) -> &gtk::CenterBox {
        &self.container
    }
}