// lib.rs for oxide_linux

use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;
use thiserror::Error;

/// Custom error type for `oxide_linux`
#[derive(Debug, Error)]
pub enum OxideLinuxError {
    #[error("Failed to read from file: {0}")]
    FileReadError(#[from] io::Error),

    #[error("Failed to parse data: {0}")]
    ParseError(String),
}

/// Struct to encapsulate system information utilities
pub struct SystemInfo;

impl SystemInfo {
    /// Gets the Linux kernel version using `uname`.
    pub fn kernel_version() -> Option<String> {
        match std::process::Command::new("uname").arg("-r").output() {
            Ok(output) if output.status.success() => {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            }
            _ => None,
        }
    }

    /// Fetches the system uptime from `/proc/uptime`.
    pub fn system_uptime() -> Result<f64, OxideLinuxError> {
        let mut file = File::open("/proc/uptime")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let uptime: f64 = contents
            .split_whitespace()
            .next()
            .ok_or_else(|| OxideLinuxError::ParseError("Missing uptime value".to_string()))?
            .parse()
            .map_err(|e| OxideLinuxError::ParseError(format!("Parse error: {}", e)))?;

        Ok(uptime)
    }

    /// Retrieves the available memory in kilobytes from `/proc/meminfo`.
    pub fn available_memory() -> Result<u64, OxideLinuxError> {
        let mut file = File::open("/proc/meminfo")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            if line.starts_with("MemAvailable:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return u64::from_str(parts[1])
                        .map_err(|e| OxideLinuxError::ParseError(format!("Parse error: {}", e)));
                }
            }
        }
        Err(OxideLinuxError::ParseError(
            "MemAvailable field not found".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_version() {
        assert!(SystemInfo::kernel_version().is_some(), "Failed to get kernel version");
    }

    #[test]
    fn test_system_uptime() {
        match SystemInfo::system_uptime() {
            Ok(uptime) => assert!(uptime >= 0.0, "Uptime should be non-negative"),
            Err(_) => panic!("Error getting system uptime"),
        }
    }

    #[test]
    fn test_available_memory() {
        match SystemInfo::available_memory() {
            Ok(memory) => assert!(memory > 0, "Available memory should be positive"),
            Err(_) => panic!("Error getting available memory"),
        }
    }
}
