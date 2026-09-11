//! Shared fundamental types and utilities for Muaaz OS.
//!
//! This crate is intentionally dependency-free so that every other
//! workspace crate can depend on it without pulling in UI or system
//! dependencies. It contains domain primitives that are shared between
//! the desktop, the system layer, and future applications.

/// A unique identifier for an application registered on the desktop.
///
/// Identifiers use the reverse-domain convention, e.g. `org.muaaz.terminal`.
/// They are immutable and cheap to compare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppId(String);

impl AppId {
    /// Creates a new [`AppId`] from a string.
    ///
    /// # Panics
    ///
    /// Panics if `value` is empty. This is a programmer error caught at
    /// construction time, so [`AppId`] can never hold an invalid id.
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.trim().is_empty(), "AppId must not be empty");
        Self(value)
    }

    /// Borrows this identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for AppId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_roundtrips() {
        let id = AppId::new("org.muaaz.terminal");
        assert_eq!(id.as_str(), "org.muaaz.terminal");
        assert_eq!(id.to_string(), "org.muaaz.terminal");
    }

    #[test]
    fn app_id_equality() {
        assert_eq!(AppId::new("a"), AppId::new("a"));
        assert_ne!(AppId::new("a"), AppId::new("b"));
    }

    #[test]
    #[should_panic]
    fn app_id_rejects_empty() {
        let _ = AppId::new("   ");
    }
}