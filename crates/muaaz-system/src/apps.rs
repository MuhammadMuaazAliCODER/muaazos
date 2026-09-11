use muaaz_core::AppId;
use std::process::Command;

/// Which kind of backing an application uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppKind {
    /// A Muaaz-native application implemented in this workspace.
    Native,
    /// An external (host) program spawned as a subprocess.
    Host,
    /// Declared for the future but not implemented yet.
    Planned,
}

/// Category used for grouping applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppCategory {
    System,
    Development,
    Files,
    Utilities,
    Unknown,
}

/// A single application known to the desktop launcher.
#[derive(Debug, Clone)]
pub struct App {
    /// Stable identifier (`org.muaaz.files`).
    pub id: AppId,
    /// Display name shown in the launcher.
    pub name: String,
    /// One-line description.
    pub summary: String,
    /// Icon theme name (e.g. `utilities-terminal`).
    pub icon_name: String,
    /// Grouping category.
    pub category: AppCategory,
    /// How this app is launched.
    pub kind: AppKind,
    /// Command used when `kind == AppKind::Host`.
    pub command: Option<String>,
}

impl App {
    /// Creates a native (Muaaz-implemented) app entry.
    pub fn native(id: &str, name: &str, summary: &str, icon_name: &str, category: AppCategory) -> Self {
        Self {
            id: AppId::new(id),
            name: name.to_string(),
            summary: summary.to_string(),
            icon_name: icon_name.to_string(),
            category,
            kind: AppKind::Native,
            command: None,
        }
    }

    /// Creates a host-backed app entry that spawns an external command.
    pub fn host(
        id: &str,
        name: &str,
        summary: &str,
        icon_name: &str,
        category: AppCategory,
        command: &str,
    ) -> Self {
        let mut app = Self::native(id, name, summary, icon_name, category);
        app.kind = AppKind::Host;
        app.command = Some(command.to_string());
        app
    }

    /// Creates a "coming soon" entry for a future Muaaz application.
    pub fn planned(id: &str, name: &str, summary: &str, icon_name: &str, category: AppCategory) -> Self {
        let mut app = Self::native(id, name, summary, icon_name, category);
        app.kind = AppKind::Planned;
        app
    }
}

/// How launching an application can fail.
#[derive(Debug)]
pub enum LaunchError {
    /// App is declared but not yet implemented.
    NotImplemented(AppId),
    /// No launch command is defined for a host-backed app.
    MissingCommand(AppId),
    /// Could not spawn the external process.
    Spawn(AppId, std::io::Error),
}

impl std::fmt::Display for LaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented(id) => {
                write!(f, "{id} is not implemented yet")
            }
            Self::MissingCommand(id) => {
                write!(f, "{id} has no launch command configured")
            }
            Self::Spawn(id, e) => write!(f, "{id} failed to start: {e}"),
        }
    }
}

impl std::error::Error for LaunchError {}

/// A registry of applications that the desktop launcher can offer.
pub trait AppRegistry {
    /// Returns all registered applications.
    fn list(&self) -> Vec<App>;
    /// Launches the application with the given id.
    fn launch(&self, id: &AppId) -> Result<(), LaunchError>;
}

/// A simple in-memory registry.
///
/// The registry is intentionally a Vec so that future Muaaz applications
/// can register themselves at runtime. Persistence (and a real service
/// daemon) will replace this later without changing the trait.
#[derive(Debug, Default)]
pub struct LinuxAppRegistry {
    apps: Vec<App>,
}

impl LinuxAppRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an application, replacing any previous entry with the
    /// same id.
    pub fn register(&mut self, app: App) {
        self.apps.retain(|existing| existing.id != app.id);
        self.apps.push(app);
    }

    /// Returns the host command for an app if it can be found on `PATH`.
    ///
    /// Host apps degrade to `NotImplemented` when the command is missing
    /// (e.g. the host does not ship a terminal emulator).
    fn resolve_command(app: &App) -> Result<&str, LaunchError> {
        let cmd = app
            .command
            .as_deref()
            .ok_or_else(|| LaunchError::MissingCommand(app.id.clone()))?;
        let program = cmd.split_whitespace().next().unwrap_or(cmd);
        if which(program) {
            Ok(cmd)
        } else {
            Err(LaunchError::NotImplemented(app.id.clone()))
        }
    }
}

impl AppRegistry for LinuxAppRegistry {
    fn list(&self) -> Vec<App> {
        self.apps.clone()
    }

    fn launch(&self, id: &AppId) -> Result<(), LaunchError> {
        let app = self
            .apps
            .iter()
            .find(|candidate| &candidate.id == id)
            .ok_or_else(|| LaunchError::NotImplemented(id.clone()))?;

        match app.kind {
            AppKind::Planned => Err(LaunchError::NotImplemented(app.id.clone())),
            AppKind::Native => Err(LaunchError::NotImplemented(app.id.clone())),
            AppKind::Host => {
                let cmd = Self::resolve_command(app)?;
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn()
                    .map_err(|e| LaunchError::Spawn(app.id.clone(), e))?;
                Ok(())
            }
        }
    }
}

/// Returns `true` if `program` is executable and present on `PATH`.
fn which(program: &str) -> bool {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|dir| dir.join(program))
        .any(|candidate| candidate.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_registry() -> LinuxAppRegistry {
        let mut registry = LinuxAppRegistry::new();
        registry.register(App::native(
            "org.muaaz.about",
            "About",
            "About Muaaz OS",
            "help-about",
            AppCategory::System,
        ));
        registry.register(App::planned(
            "org.muaaz.software",
            "Software",
            "Install applications",
            "system-software-install",
            AppCategory::Utilities,
        ));
        registry
    }

    #[test]
    fn lists_all_registered() {
        let registry = sample_registry();
        assert_eq!(registry.list().len(), 2);
    }

    #[test]
    fn planned_app_cannot_launch() {
        let registry = sample_registry();
        let result = registry.launch(&AppId::new("org.muaaz.software"));
        assert!(matches!(result, Err(LaunchError::NotImplemented(_))));
    }

    #[test]
    fn unknown_app_cannot_launch() {
        let registry = sample_registry();
        let result = registry.launch(&AppId::new("org.muaaz.missing"));
        assert!(matches!(result, Err(LaunchError::NotImplemented(_))));
    }

    #[test]
    fn register_replaces_same_id() {
        let mut registry = sample_registry();
        registry.register(App::planned(
            "org.muaaz.about",
            "About (new)",
            "replacement",
            "help-about",
            AppCategory::System,
        ));
        let replaced = registry
            .list()
            .into_iter()
            .find(|a| a.id.as_str() == "org.muaaz.about")
            .unwrap();
        assert_eq!(replaced.summary, "replacement");
        assert_eq!(registry.list().len(), 2);
    }
}