use std::fmt;

/// Battery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

impl fmt::Display for BatteryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Charging => f.write_str("Charging"),
            Self::Discharging => f.write_str("Discharging"),
            Self::Full => f.write_str("Full"),
            Self::NotCharging => f.write_str("Not Charging"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

/// A snapshot of battery information.
#[derive(Debug, Clone)]
pub struct BatteryStateInfo {
    /// Current charge percentage (0–100).
    pub percentage: u8,
    /// State of the battery.
    pub state: BatteryState,
    /// Whether a battery was detected at all.
    pub present: bool,
}

impl Default for BatteryStateInfo {
    fn default() -> Self {
        Self {
            percentage: 0,
            state: BatteryState::Unknown,
            present: false,
        }
    }
}

/// Reads battery state.
pub trait BatteryProvider {
    fn read(&self) -> Result<BatteryStateInfo, crate::SystemError>;
}

/// Linux implementation reading `/sys/class/power_supply/`.
#[derive(Debug, Default, Clone)]
pub struct LinuxBatteryProvider;

impl BatteryProvider for LinuxBatteryProvider {
    fn read(&self) -> Result<BatteryStateInfo, crate::SystemError> {
        let base = std::path::PathBuf::from("/sys/class/power_supply");

        if !base.exists() {
            return Ok(BatteryStateInfo::default());
        }

        let mut entries: Vec<_> = std::fs::read_dir(&base)
            .map_err(std::io::Error::from)?
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in &entries {
            let dir = entry.path();
            let type_path = dir.join("type");
            let type_str = std::fs::read_to_string(&type_path).unwrap_or_default();
            if type_str.trim() != "Battery" {
                continue;
            }

            let capacity = std::fs::read_to_string(dir.join("capacity"))
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
                .unwrap_or(0);

            let status = match std::fs::read_to_string(dir.join("status"))
                .ok()
                .map(|s| s.trim().to_uppercase())
            {
                Some(ref s) if s == "CHARGING" => BatteryState::Charging,
                Some(ref s) if s == "DISCHARGING" => BatteryState::Discharging,
                Some(ref s) if s == "FULL" => BatteryState::Full,
                Some(ref s) if s == "NOT CHARGING" => BatteryState::NotCharging,
                _ => BatteryState::Unknown,
            };

            return Ok(BatteryStateInfo {
                percentage: capacity,
                state: status,
                present: true,
            });
        }

        Ok(BatteryStateInfo::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_battery_is_absent() {
        let info = BatteryStateInfo::default();
        assert!(!info.present);
    }
}