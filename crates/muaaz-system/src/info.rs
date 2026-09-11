use std::fmt;

/// Static information about the host system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemInfo {
    /// Hostname of the machine.
    pub hostname: String,
    /// Kernel release string (e.g. `6.11.3-arch1-1`).
    pub kernel_release: String,
    /// Kernel name (e.g. `Linux`).
    pub kernel_name: String,
    /// Human-readable operating system identifier, if known.
    pub os_pretty_name: Option<String>,
    /// Machine architecture (e.g. `x86_64`).
    pub architecture: Option<String>,
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.hostname, self.kernel_release)
    }
}

/// Reads static system information.
pub trait SystemInfoProvider {
    /// Returns the current static system information.
    fn read(&self) -> Result<SystemInfo, crate::SystemError>;
}

/// Linux implementation backed by `/proc` and `/sys`.
#[derive(Debug, Default, Clone)]
pub struct LinuxSystemInfo;

const OS_RELEASE_PATH: &str = "/etc/os-release";
const UTS_NAME_PATH: &str = "/proc/sys/kernel/osrelease";
const UTS_NODENAME_PATH: &str = "/proc/sys/kernel/hostname";
const ARCH_NAME_PATH: &str = "/proc/sys/kernel/arch";

impl SystemInfoProvider for LinuxSystemInfo {
    fn read(&self) -> Result<SystemInfo, crate::SystemError> {
        let hostname = read_trimmed(UTS_NODENAME_PATH)
            .or_else(|_| read_pretty_field(OS_RELEASE_PATH, "PRETTY_NAME"))?;

        Ok(SystemInfo {
            hostname,
            kernel_release: read_trimmed(UTS_NAME_PATH)?,
            kernel_name: "Linux".to_string(),
            os_pretty_name: read_pretty_field(OS_RELEASE_PATH, "PRETTY_NAME").ok(),
            architecture: read_trimmed(ARCH_NAME_PATH).ok(),
        })
    }
}

fn read_trimmed(path: &str) -> Result<String, crate::SystemError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents.trim().to_string())
}

fn read_pretty_field(path: &str, key: &str) -> Result<String, crate::SystemError> {
    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        if k == key {
            return Ok(v.trim().trim_matches('"').to_string());
        }
    }
    Err(crate::SystemError::Parse(format!(
        "{key} not found in {path}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pretty_field() {
        let dir = std::env::temp_dir().join("muaaz-system-test-os-release");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("os-release");
        std::fs::write(
            &path,
            "NAME=\"Big OS\"\nPRETTY_NAME=\"Big OS 1.0 (Test)\"\nID=bigos\n",
        )
        .unwrap();
        let result = read_pretty_field(path.to_str().unwrap(), "PRETTY_NAME");
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(result.unwrap(), "Big OS 1.0 (Test)");
    }

    #[test]
    fn missing_pretty_field_is_parse_error() {
        let dir = std::env::temp_dir().join("muaaz-system-test-os-release-2");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("os-release");
        std::fs::write(&path, "NAME=\"No Pretty\"\n").unwrap();
        let result = read_pretty_field(path.to_str().unwrap(), "PRETTY_NAME");
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(matches!(result, Err(crate::SystemError::Parse(_))));
    }
}