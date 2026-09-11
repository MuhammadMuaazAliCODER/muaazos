//! The top system panel.
//!
//! Contains the brand mark, the application launcher trigger, the live
//! system readout (CPU/memory/battery) and the clock.

pub mod clock;
pub mod panel;
pub mod status;

pub use panel::Panel;