/// Snapshot of current memory usage.
#[derive(Debug, Clone, Copy)]
pub struct MemoryInfo {
    /// Total RAM in bytes.
    pub total_bytes: u64,
    /// Used RAM in bytes (total minus free minus buffers/cache where
    /// available).
    pub used_bytes: u64,
    /// Available RAM in bytes.
    pub available_bytes: u64,
}

impl MemoryInfo {
    /// Usage in the range `0.0 ..= 1.0`.
    pub fn usage_fraction(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64).clamp(0.0, 1.0)
        }
    }

    /// Total memory in mebibytes.
    pub fn total_mib(&self) -> u64 {
        self.total_bytes / (1024 * 1024)
    }

    /// Used memory in mebibytes.
    pub fn used_mib(&self) -> u64 {
        self.used_bytes / (1024 * 1024)
    }
}

impl std::fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.0} / {:.0} MiB ({:.0}%)",
            self.used_mib(),
            self.total_mib(),
            self.usage_fraction() * 100.0
        )
    }
}

/// Reads memory information from the system.
pub trait MemoryProvider {
    fn read(&self) -> Result<MemoryInfo, crate::SystemError>;
}

/// Linux implementation reading `/proc/meminfo`.
#[derive(Debug, Default, Clone)]
pub struct LinuxMemoryProvider;

impl MemoryProvider for LinuxMemoryProvider {
    fn read(&self) -> Result<MemoryInfo, crate::SystemError> {
        let contents = std::fs::read_to_string("/proc/meminfo")?;
        let mut total: Option<u64> = None;
        let mut available: Option<u64> = None;

        for line in contents.lines() {
            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim();
            // Value like "12345678 kB" or just "12345678"
            let numeric: u64 = value
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            // The unit in /proc/meminfo is always kB (1024 bytes)
            let bytes = numeric * 1024;

            match key {
                "MemTotal" => total = Some(bytes),
                "MemAvailable" => available = Some(bytes),
                _ => {}
            }
        }

        let total = total
            .ok_or_else(|| crate::SystemError::Parse("MemTotal missing".into()))?;
        let available = available
            .ok_or_else(|| crate::SystemError::Parse("MemAvailable missing".into()))?;

        Ok(MemoryInfo {
            total_bytes: total,
            used_bytes: total.saturating_sub(available),
            available_bytes: available,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_fraction_clamps() {
        let m = MemoryInfo {
            total_bytes: 1024,
            used_bytes: 512,
            available_bytes: 512,
        };
        assert!((m.usage_fraction() - 0.5).abs() < 0.01);
        assert_eq!(m.total_mib(), 0); // too small to show in MiB
    }
}