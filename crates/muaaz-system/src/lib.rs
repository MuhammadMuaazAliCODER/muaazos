//! System-level integration and services for Muaaz OS.
//!
//! This crate provides clean, trait-based interfaces for querying the
//! host operating system (currently Linux) so that the desktop and UI
//! crates never have to know *how* a value is obtained. The concrete
//! Linux implementation lives behind these traits and can be swapped
//! out when Muaaz OS grows its own system service layer.
//!
//! Layering:
//!
//! ```text
//! Muaaz Desktop
//!      |
//!      v
//! Muaaz System API  (this crate)
//!      |
//!      v
//! Linux /proc, /sys
//! ```

mod apps;
mod battery;
mod cpu;
mod info;
mod memory;

pub use apps::{App, AppCategory, AppKind, AppRegistry, LaunchError, LinuxAppRegistry};
pub use battery::{BatteryProvider, BatteryState, BatteryStateInfo, LinuxBatteryProvider};
pub use cpu::{CpuProvider, CpuUsage};
pub use info::{SystemInfo, SystemInfoProvider, LinuxSystemInfo};
pub use memory::{LinuxMemoryProvider, MemoryInfo, MemoryProvider};

use std::fmt;

/// Errors that can occur while reading system state.
///
/// These are always recoverable from the desktop's point of view: if a
/// statistic cannot be read, the UI shows a placeholder rather than
/// failing the whole application.
#[derive(Debug)]
pub enum SystemError {
    /// An underlying file could not be read (`/proc`, `/sys`, ...).
    Io(std::io::Error),
    /// The data was read but could not be parsed.
    Parse(String),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error while reading system state: {e}"),
            Self::Parse(what) => write!(f, "could not parse system data: {what}"),
        }
    }
}

impl std::error::Error for SystemError {}

impl From<std::io::Error> for SystemError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// A small façade over the system services a desktop needs.
///
/// Holding one value of this type is easier for the UI to manage than
/// juggling several independent providers, and it keeps the provider
/// wiring (which file to read, which process to spawn) out of the UI.
pub struct MuaazSystem {
    /// Static information about the host.
    pub info: LinuxSystemInfo,
    /// CPU usage over time.
    pub cpu: CpuProvider,
    /// Memory information.
    pub memory: LinuxMemoryProvider,
    /// Battery state (no-op on machines without a battery).
    pub battery: LinuxBatteryProvider,
    /// Registered desktop applications.
    pub apps: LinuxAppRegistry,
}

impl Default for MuaazSystem {
    fn default() -> Self {
        Self {
            info: LinuxSystemInfo,
            cpu: CpuProvider::new(),
            memory: LinuxMemoryProvider,
            battery: LinuxBatteryProvider,
            apps: LinuxAppRegistry::new(),
        }
    }
}

impl MuaazSystem {
    /// Creates the default set of system services.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_error_displays() {
        let err = SystemError::Parse("bad value".into());
        assert!(err.to_string().contains("bad value"));
    }
}