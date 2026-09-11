use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use muaaz_system::{App, AppRegistry, MuaazSystem};

/// A popover listing all registered desktop applications.
///
/// Attach its [`popover`](Launcher::popover) to a menu button; GTK
/// manages show/hide automatically.
pub struct Launcher {
    popover: gtk::Popover,
}

impl Launcher {
    /// Builds the launcher.
    pub fn new(system: &Rc<RefCell<MuaazSystem>>, on_activate: Rc<dyn Fn(&App)>) -> Self {
        let apps = system.borrow().apps.list();

        let search = gtk::SearchEntry::builder()
            .placeholder_text("Search applications…")
            .hexpand(true)
            .css_classes(["muaaz-launcher-search"])
            .build();

        let list = gtk::ListBox::builder()
            .css_classes(["muaaz-launcher"])
            .selection_mode(gtk::SelectionMode::None)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        for app in &apps {
            list.append(&row(app));
        }

        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .has_frame(false)
            .min_content_height(120)
            .max_content_height(360)
            .child(&list)
            .build();

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.append(&search);
        container.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        container.append(&scrolled);

        let popover = gtk::Popover::builder()
            .css_classes(["muaaz-launcher"])
            .width_request(380)
            .position(gtk::PositionType::Bottom)
            .child(&container)
            .build();

        // Search filter — shows only rows whose name or summary match
        // the search text.
        {
            let list = list.clone();
            let apps = apps.clone();
            search.connect_search_changed(move |entry| {
                let needle = entry.text().to_string().to_lowercase();
                let mut index = 0usize;
                while let Some(child) = list.row_at_index(index as i32) {
                    let matches = needle.is_empty()
                        || apps.get(index).map_or(false, |app| {
                            app.name.to_lowercase().contains(&needle)
                                || app.summary.to_lowercase().contains(&needle)
                                || app.id.as_str().contains(&needle)
                        });
                    child.set_visible(matches);
                    index += 1;
                }
            });
        }

        // Row activation → launch callback → dismiss popover.
        {
            let apps = apps.clone();
            let on_activate = Rc::clone(&on_activate);
            let popover = popover.clone();
            list.connect_row_activated(move |_list, row| {
                let name = row.widget_name().to_string();
                if let Some(app) = apps.iter().find(|a| a.id.as_str() == name) {
                    on_activate(app);
                }
                popover.popdown();
            });
        }

        // Focus the search field when the popover is shown.
        {
            let search_ref = search.clone();
            popover.connect_map(move |_| {
                search_ref.grab_focus();
            });
        }

        // Pressing Escape (stop-search) dismisses the popover.
        {
            let popover = popover.clone();
            search.connect_stop_search(move |_| {
                popover.popdown();
            });
        }

        Self { popover }
    }

    /// The popover to attach to a menu button.
    pub fn popover(&self) -> &gtk::Popover {
        &self.popover
    }
}

fn row(app: &App) -> gtk::ListBoxRow {
    let icon = gtk::Image::builder()
        .icon_name(&app.icon_name)
        .pixel_size(28)
        .css_classes(["muaaz-shortcut-icon"])
        .build();

    let title = gtk::Label::builder()
        .label(&app.name)
        .css_classes(["muaaz-launcher-row-title"])
        .halign(gtk::Align::Start)
        .build();

    let desc = gtk::Label::builder()
        .label(&app.summary)
        .css_classes(["muaaz-launcher-row-desc"])
        .halign(gtk::Align::Start)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .max_width_chars(48)
        .build();

    let inner = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    inner.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 2);
    text.append(&title);
    text.append(&desc);
    inner.append(&text);

    let row = gtk::ListBoxRow::builder()
        .css_classes(["muaaz-launcher-row"])
        .child(&inner)
        .build();

    row.set_widget_name(app.id.as_str());
    row
}