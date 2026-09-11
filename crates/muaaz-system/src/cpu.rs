use std::fmt;

/// A snapshot of per-CPU aggregate usage.
#[derive(Debug, Clone, Copy)]
pub struct CpuUsage {
    /// Usage in the range `0.0 ..= 1.0`.
    pub usage_fraction: f64,
    /// Number of CPU cores.
    pub cores: u32,
}

impl fmt::Display for CpuUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.0}% of {} cores",
            self.usage_fraction * 100.0,
            self.cores
        )
    }
}

/// Provides CPU usage over time by comparing two snapshots.
pub struct CpuProvider {
    prev: CpuSample,
}

#[derive(Debug, Clone, Copy, Default)]
struct CpuSample {
    idle: u64,
    total: u64,
}

impl Default for CpuProvider {
    fn default() -> Self {
        Self {
            prev: Self::read_stat().unwrap_or_default(),
        }
    }
}

impl CpuProvider {
    /// Creates a new provider and takes an initial baseline sample.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of CPU cores.
    pub fn core_count() -> u32 {
        std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1)
    }

    /// Reads the current CPU usage since the last call to `read()`.
    pub fn read(&mut self) -> Result<CpuUsage, crate::SystemError> {
        let current = Self::read_stat()?;
        let delta_idle = current.idle.saturating_sub(self.prev.idle);
        let delta_total = current.total.saturating_sub(self.prev.total);
        self.prev = current;

        let usage = if delta_total == 0 {
            0.0
        } else {
            1.0 - (delta_idle as f64 / delta_total as f64)
        };

        Ok(CpuUsage {
            usage_fraction: usage.clamp(0.0, 1.0),
            cores: Self::core_count(),
        })
    }

    fn read_stat() -> Result<CpuSample, crate::SystemError> {
        let contents = std::fs::read_to_string("/proc/stat")?;
        let first_line = contents
            .lines()
            .next()
            .ok_or_else(|| crate::SystemError::Parse("empty /proc/stat".into()))?;

        let fields: Vec<&str> = first_line.split_whitespace().collect();
        if fields.len() < 5 {
            return Err(crate::SystemError::Parse(
                "unexpected /proc/stat format".into(),
            ));
        }

        // Fields after "cpu": user nice system idle iowait irq softirq steal
        let values: Vec<u64> = fields[1..]
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        let idle = values.get(3).copied().unwrap_or(0);
        let total: u64 = values.iter().sum();

        Ok(CpuSample { idle, total })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_count_is_at_least_one() {
        assert!(CpuProvider::core_count() >= 1);
    }

    #[test]
    fn read_returns_valid_range() {
        let mut provider = CpuProvider::new();
        // Small sleep to let counters advance
        std::thread::sleep(std::time::Duration::from_millis(50));
        let usage = provider.read().unwrap();
        assert!(usage.usage_fraction >= 0.0);
        assert!(usage.usage_fraction <= 1.0);
    }
}