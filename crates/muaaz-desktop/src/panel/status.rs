//! Live system status shown in the top panel.
//!
//! Every statistic is read behind a `Result`: when something fails to be
//! read (memory pressure, unusual `/proc` layout, no battery) the panel
//! shows a muted dash instead of crashing the desktop.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use muaaz_system::{BatteryProvider, MemoryProvider, MuaazSystem};

/// CPU/memory/battery readout for the top panel.
pub struct Status {
    container: gtk::Box,
    cpu: gtk::Label,
    memory: gtk::Label,
    battery: gtk::Label,
    battery_holder: gtk::Box,
    /// Keeps the periodic source alive for the lifetime of the status bar.
    _source: glib::SourceId,
}

impl Status {
    /// Creates a status monitor that refreshes every `interval_secs`
    /// seconds using the shared system services.
    pub fn new(system: Rc<RefCell<MuaazSystem>>, interval_secs: u32) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 14);
        container.add_css_class("muaaz-panel-label");
        container.set_halign(gtk::Align::End);

        let cpu = status_label(&container, "muaaz-status-cpu");
        cpu.set_tooltip_text(Some("CPU usage"));

        let memory = status_label(&container, "muaaz-status-mem");
        memory.set_tooltip_text(Some("Memory usage"));

        let battery_holder = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let battery = status_label(&battery_holder, "muaaz-status-bat");
        battery.set_tooltip_text(Some("Battery"));
        battery_holder.append(&battery);
        battery_holder.set_visible(false);
        container.append(&battery_holder);

        // Initial fill.
        update_labels(&cpu, &memory, &battery, &battery_holder, &system);

        let cpu_clone = cpu.clone();
        let memory_clone = memory.clone();
        let battery_clone = battery.clone();
        let battery_holder_clone = battery_holder.clone();
        let source = glib::timeout_add_local(
            Duration::from_secs(u64::from(interval_secs)),
            move || {
                update_labels(
                    &cpu_clone,
                    &memory_clone,
                    &battery_clone,
                    &battery_holder_clone,
                    &system,
                );
                glib::ControlFlow::Continue
            },
        );

        Status { container, cpu, memory, battery, battery_holder, _source: source }
    }

    /// The widget to embed in the panel.
    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }
}

fn status_label(parent: &gtk::Box, css_class: &str) -> gtk::Label {
    let label = gtk::Label::new(Some("—"));
    label.add_css_class(css_class);
    parent.append(&label);
    label
}

/// Reads all system statistics and renders them into the given labels.
fn update_labels(
    cpu: &gtk::Label,
    memory: &gtk::Label,
    battery: &gtk::Label,
    battery_holder: &gtk::Box,
    system: &Rc<RefCell<MuaazSystem>>,
) {
    let mut guard = system.borrow_mut();

    match guard.cpu.read() {
        Ok(usage) => cpu.set_label(&format!("CPU {:.0}%", usage.usage_fraction * 100.0)),
        Err(_) => cpu.set_label("CPU —"),
    }

    match guard.memory.read() {
        Ok(mem) => memory.set_label(&format!(
            "RAM {:.1} / {:.1} GiB",
            mem.used_mib() as f64 / 1024.0,
            mem.total_mib() as f64 / 1024.0
        )),
        Err(_) => memory.set_label("RAM —"),
    }

    match guard.battery.read() {
        Ok(state) if state.present => {
            let suffix = if state.state == muaaz_system::BatteryState::Charging {
                " +"
            } else {
                ""
            };
            battery.set_label(&format!("{}{suffix}", state.percentage));
            battery_holder.set_visible(true);
        }
        _ => battery_holder.set_visible(false),
    }
}