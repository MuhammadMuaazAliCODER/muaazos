use std::process::Command;
use std::rc::Rc;

use gtk::prelude::*;
use muaaz_system::{App, AppKind};
use muaaz_ui as ui;

/// Root widget for the welcome section.
pub struct Welcome {
    container: gtk::Box,
}

impl Welcome {
    /// Builds the welcome hero and shortcut grid.
    pub fn new(apps: Vec<App>, on_activate: Rc<dyn Fn(&App)>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 28);
        container.set_halign(gtk::Align::Center);
        container.set_valign(gtk::Align::Center);
        container.set_vexpand(true);
        container.set_margin_top(48);
        container.set_margin_bottom(32);
        container.set_margin_start(32);
        container.set_margin_end(32);
        container.add_css_class("muaaz-welcome");

        // --- Hero --------------------------------------------------------
        let title = gtk::Label::builder()
            .label(ui::theme::brand::NAME)
            .css_classes(["muaaz-welcome-title"])
            .halign(gtk::Align::Center)
            .build();
        container.append(&title);

        let subtitle = gtk::Label::builder()
            .label(ui::theme::brand::TAGLINE)
            .css_classes(["muaaz-welcome-subtitle"])
            .halign(gtk::Align::Center)
            .build();
        container.append(&subtitle);

        let version_chip = gtk::Label::builder()
            .label(format!("Desktop {}", crate::config::DesktopConfig::load().version))
            .css_classes(["muaaz-welcome-chip"])
            .halign(gtk::Align::Center)
            .build();
        container.append(&version_chip);

        // --- Shortcuts ---------------------------------------------------
        let kicker = gtk::Label::builder()
            .label("APPLICATIONS")
            .css_classes(["muaaz-section-kicker"])
            .halign(gtk::Align::Center)
            .build();
        container.append(&kicker);

        let flow = gtk::FlowBox::builder()
            .halign(gtk::Align::Center)
            .homogeneous(true)
            .min_children_per_line(2)
            .max_children_per_line(5)
            .column_spacing(12)
            .row_spacing(12)
            .selection_mode(gtk::SelectionMode::None)
            .css_classes(["muaaz-shortcuts"])
            .build();

        for app in &apps {
            let on_activate = Rc::clone(&on_activate);
            let app_clone = app.clone();
            let button = shortcut_button(app);
            button.connect_clicked(move |_| {
                on_activate(&app_clone);
            });
            flow.append(&button);
        }

        container.append(&flow);

        // --- Hint --------------------------------------------------------
        let hint = gtk::Label::builder()
            .label("Development build \u{2014} not a full operating system yet")
            .css_classes(["muaaz-welcome-hint"])
            .halign(gtk::Align::Center)
            .build();
        container.append(&hint);

        Self { container }
    }

    /// The root widget of the welcome section.
    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }
}

/// Decides what to do when an application is activated.
pub fn handle_activation(
    app: &App,
    window: &impl IsA<gtk::Window>,
    overlay: &libadwaita::ToastOverlay,
) {
    match app.kind {
        AppKind::Native if app.id.as_str() == "org.muaaz.about" => {
            about_dialog(window);
        }
        AppKind::Host => {
            let cmd = match app.command.as_deref() {
                Some(c) => c,
                None => {
                    let toast =
                        libadwaita::Toast::new(&format!("{} has no launch command", app.name));
                    overlay.add_toast(toast);
                    return;
                }
            };
            if let Err(e) = Command::new("sh").arg("-c").arg(cmd).spawn() {
                let toast =
                    libadwaita::Toast::new(&format!("Failed to start {}: {e}", app.name));
                overlay.add_toast(toast);
            }
        }
        _ => {
            let toast =
                libadwaita::Toast::new(&format!("{} is not available yet", app.name));
            overlay.add_toast(toast);
        }
    }
}

/// Shows the About Muaaz OS dialog.
pub fn about_dialog(window: &impl IsA<gtk::Window>) {
    let about = libadwaita::AboutWindow::builder()
        .transient_for(window)
        .application_name(ui::theme::brand::NAME)
        .version(ui::theme::brand::VERSION)
        .developers(["Muaaz OS Project"])
        .comments(ui::theme::brand::TAGLINE)
        .license_type(gtk::License::MitX11)
        .website("https://github.com/your-org/muaaz-os")
        .issue_url("https://github.com/your-org/muaaz-os/issues")
        .copyright("\u{00a9} 2026 Muaaz OS Project")
        .build();
    about.present();
}

/// Creates a single shortcut button for the FlowBox grid.
fn shortcut_button(app: &App) -> gtk::Button {
    let icon = gtk::Image::builder()
        .icon_name(&app.icon_name)
        .pixel_size(28)
        .css_classes(["muaaz-shortcut-icon"])
        .build();

    let title = gtk::Label::builder()
        .label(&app.name)
        .css_classes(["muaaz-shortcut-title"])
        .halign(gtk::Align::Start)
        .build();

    let desc = gtk::Label::builder()
        .label(&app.summary)
        .css_classes(["muaaz-shortcut-desc"])
        .halign(gtk::Align::Start)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .max_width_chars(28)
        .build();

    let status = gtk::Label::builder()
        .label(status_text(app))
        .css_classes(["muaaz-shortcut-status"])
        .halign(gtk::Align::Start)
        .build();

    let inner = gtk::Box::new(gtk::Orientation::Vertical, 8);
    inner.append(&icon);
    inner.append(&title);
    inner.append(&desc);
    inner.append(&status);

    let button = gtk::Button::builder()
        .css_classes(["muaaz-shortcut"])
        .child(&inner)
        .build();

    button.set_widget_name(app.id.as_str());
    button
}

/// Human-readable availability text for a shortcut button.
fn status_text(app: &App) -> &'static str {
    match app.kind {
        AppKind::Native => "Included",
        AppKind::Planned => "Coming soon",
        AppKind::Host => {
            if app.command.as_deref().is_some() {
                "Available"
            } else {
                "Not installed"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_text_matches_kind() {
        let app = App::planned(
            "t",
            "T",
            "D",
            "icon",
            muaaz_system::AppCategory::Utilities,
        );
        assert_eq!(status_text(&app), "Coming soon");
    }
}