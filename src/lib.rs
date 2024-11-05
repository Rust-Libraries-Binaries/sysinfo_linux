// lib.rs for sysinfo_linux

use std::fs::File;
use std::io::{Read};
use std::str::FromStr;
use thiserror::Error;

/// Custom error type for `sysinfo_linux`
#[derive(Debug, Error)]
pub enum SysInfoLinuxError {
    #[error("Failed to read from file: {0}")]
    FileReadError(String),

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
    pub fn system_uptime() -> Result<f64, SysInfoLinuxError> {
        let mut file = File::open("/proc/uptime").map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;

        let uptime: f64 = contents
            .split_whitespace()
            .next()
            .ok_or_else(|| SysInfoLinuxError::ParseError("Missing uptime value".to_string()))?
            .parse()
            .map_err(|e| SysInfoLinuxError::ParseError(format!("Parse error: {}", e)))?;

        Ok(uptime)
    }

    /// Retrieves the available memory in kilobytes from `/proc/meminfo`.
    pub fn available_memory() -> Result<u64, SysInfoLinuxError> {
        let mut file = File::open("/proc/meminfo").map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;

        for line in contents.lines() {
            if line.starts_with("MemAvailable:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return u64::from_str(parts[1])
                        .map_err(|e| SysInfoLinuxError::ParseError(format!("Parse error: {}", e)));
                }
            }
        }
        Err(SysInfoLinuxError::ParseError(
            "MemAvailable field not found".to_string(),
        ))
    }
}
