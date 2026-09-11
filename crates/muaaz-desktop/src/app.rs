use gtk::glib;
use gtk::prelude::*;
use muaaz_ui as ui;

/// The Muaaz Desktop application.
pub struct MuaazDesktop {
    application: gtk::Application,
}

impl MuaazDesktop {
    /// Creates a new, unstarted desktop application.
    pub fn new() -> Self {
        let application = gtk::Application::builder()
            .application_id(ui::theme::brand::APP_ID)
            .build();

        Self::build_actions(&application);
        application.connect_activate(Self::on_activate);

        Self { application }
    }

    fn build_actions(application: &gtk::Application) {
        // app.quit — Ctrl+Q
        let quit = gtk::gio::SimpleAction::new("quit", None);
        quit.connect_activate(glib::clone!(@weak application => move |_, _| {
            application.quit();
        }));
        application.add_action(&quit);
        application.set_accels_for_action("app.quit", &["<Control>q"]);

        // app.about — F1
        let about = gtk::gio::SimpleAction::new("about", None);
        about.connect_activate(glib::clone!(@weak application => move |_, _| {
            if let Some(window) = application.active_window() {
                crate::welcome::about_dialog(&window);
            }
        }));
        application.add_action(&about);
        application.set_accels_for_action("app.about", &["F1"]);
    }

    fn on_activate(application: &gtk::Application) {
        if let Err(err) = ui::load_css() {
            eprintln!("Failed to load the Muaaz stylesheet: {err}");
        }

        if let Some(settings) = gtk::Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        match application.active_window() {
            Some(window) => window.present(),
            None => {
                let desktop = crate::window::DesktopWindow::new(application);
                desktop.present();
            }
        }
    }

    /// Runs the application (blocks until it quits).
    pub fn run(&self) -> std::process::ExitCode {
        self.application.run().into()
    }
}

impl Default for MuaazDesktop {
    fn default() -> Self {
        Self::new()
    }
}