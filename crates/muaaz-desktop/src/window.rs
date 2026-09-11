use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use muaaz_system::{AppCategory, AppRegistry, MuaazSystem};
use muaaz_ui as ui;

use crate::config::DesktopConfig;
use crate::launcher::Launcher;
use crate::panel::Panel;
use crate::welcome::{self, Welcome};

/// The main desktop window.
pub struct DesktopWindow {
    window: gtk::ApplicationWindow,
    _toast_overlay: libadwaita::ToastOverlay,
}

impl DesktopWindow {
    /// Creates and shows the desktop window.
    pub fn new(app: &gtk::Application) -> Self {
        let config = DesktopConfig::load();
        let mut system = MuaazSystem::new();
        register_system_apps(&mut system.apps);
        let system = Rc::new(RefCell::new(system));

        let toast_overlay = libadwaita::ToastOverlay::new();
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(ui::theme::brand::NAME)
            .default_width(config.window_width)
            .default_height(config.window_height)
            .build();
        window.set_size_request(config.min_window_width, config.min_window_height);
        window.add_css_class("muaaz-window");
        window.set_child(Some(&toast_overlay));

        // Activation callback shared by the launcher and the shortcuts.
        let overlay = toast_overlay.clone();
        let window_ref = window.clone();
        let on_activate: Rc<dyn Fn(&muaaz_system::App)> = Rc::new(move |app| {
            welcome::handle_activation(app, &window_ref, &overlay);
        });

        // Launcher (wrapped in a popover).
        let launcher = Launcher::new(&system, Rc::clone(&on_activate));

        // Top panel.
        let panel = Panel::new(Rc::clone(&system), &config);
        panel
            .launcher_button
            .set_popover(Some(launcher.popover()));

        // Welcome body.
        let apps = system.borrow().apps.list();
        let welcome_section = Welcome::new(apps, on_activate);

        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(welcome_section.widget())
            .build();

        let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
        root.add_css_class("muaaz-root");
        root.append(panel.widget());
        root.append(&scrolled);

        toast_overlay.set_child(Some(&root));

        // Keyboard action: toggle the launcher via Ctrl+Space.
        {
            let btn = panel.launcher_button.clone();
            let toggle = gtk::gio::SimpleAction::new("toggle-launcher", None);
            toggle.connect_activate(move |_, _| {
                btn.set_active(!btn.is_active());
            });
            window.add_action(&toggle);
        }

        Self {
            window,
            _toast_overlay: toast_overlay,
        }
    }

    /// Presents the window.
    pub fn present(&self) {
        self.window.present();
    }
}

/// Populates the system app registry with the default set of
/// applications.
fn register_system_apps(registry: &mut muaaz_system::LinuxAppRegistry) {
    registry.register(muaaz_system::App::native(
        "org.muaaz.about",
        "About",
        "About Muaaz OS",
        "help-about-symbolic",
        AppCategory::System,
    ));

    registry.register(muaaz_system::App::host(
        "org.muaaz.terminal",
        "Terminal",
        "System terminal",
        "utilities-terminal-symbolic",
        AppCategory::Development,
        "x-terminal-emulator",
    ));

    registry.register(muaaz_system::App::host(
        "org.muaaz.files",
        "Files",
        "Browse files",
        "system-file-manager-symbolic",
        AppCategory::Files,
        "nautilus",
    ));

    registry.register(muaaz_system::App::host(
        "org.muaaz.settings",
        "Settings",
        "System preferences",
        "preferences-system-symbolic",
        AppCategory::System,
        "gnome-control-center",
    ));

    registry.register(muaaz_system::App::planned(
        "org.muaaz.software",
        "Software",
        "Install applications",
        "system-software-install-symbolic",
        AppCategory::Utilities,
    ));
}