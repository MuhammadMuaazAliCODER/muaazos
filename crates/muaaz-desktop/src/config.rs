//! Desktop-level configuration.
//!
//! Configuration lives in one place so the desktop behaves the same on
//! every machine. Future releases can read from `$XDG_CONFIG_HOME` or a
//! Muaaz configuration service without changing widget code.

/// Desktop configuration for Muaaz Desktop.
#[derive(Debug, Clone)]
pub struct DesktopConfig {
    /// Initial window width in pixels.
    pub window_width: i32,
    /// Initial window height in pixels.
    pub window_height: i32,
    /// Minimum window width in pixels.
    pub min_window_width: i32,
    /// Minimum window height in pixels.
    pub min_window_height: i32,
    /// How often the clock refreshes, in seconds.
    pub clock_interval_secs: u32,
    /// How often system status refreshes, in seconds.
    pub status_interval_secs: u32,
    /// Number of shortcut columns at most, before FlowBox wraps.
    pub shortcut_max_columns: u32,
    /// Product version shown in the welcome section.
    pub version: String,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            window_width: 1180,
            window_height: 760,
            min_window_width: 720,
            min_window_height: 480,
            clock_interval_secs: 1,
            status_interval_secs: 2,
            shortcut_max_columns: 5,
            version: "0.1".to_string(),
        }
    }
}

impl DesktopConfig {
    /// Loads the desktop configuration.
    ///
    /// For now configuration is compile-time defaults. Reading from disk
    /// is deliberately deferred so no configuration framework is pulled
    /// in before the story actually needs one.
    pub fn load() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let config = DesktopConfig::load();
        assert!(config.window_width >= config.min_window_width);
        assert!(config.window_height >= config.min_window_height);
        assert!(config.clock_interval_secs >= 1);
        assert!(config.shortcut_max_columns >= 1);
    }
}