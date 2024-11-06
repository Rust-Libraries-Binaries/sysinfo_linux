use std::fs::File;
use std::io::{self, BufRead, Read};
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
    // Other methods...

    /// Fetches network interface statistics from `/proc/net/dev`.
    pub fn network_interface_stats() -> Result<Vec<NetworkInterface>, SysInfoLinuxError> {
        let file = File::open("/proc/net/dev").map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;
        let reader = io::BufReader::new(file);
        
        let mut interfaces = Vec::new();
        
        for line in reader.lines().skip(2) {
            let line = line.map_err(|e| SysInfoLinuxError::FileReadError(e.to_string()))?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() >= 17 {
                interfaces.push(NetworkInterface {
                    name: parts[0].trim_end_matches(':').to_string(),
                    rx_bytes: parts[1].parse().map_err(|e: std::num::ParseIntError| SysInfoLinuxError::ParseError(e.to_string()))?,
                    tx_bytes: parts[9].parse().map_err(|e: std::num::ParseIntError| SysInfoLinuxError::ParseError(e.to_string()))?,
                });
            }
        }
        
        Ok(interfaces)
    }
}

#[derive(Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

