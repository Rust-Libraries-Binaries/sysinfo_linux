## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# sysinfo_linux

A Rust library to fetch and display various system information on Linux systems. This library provides methods to retrieve kernel version, system uptime, available memory, and network interface statistics.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sysinfo_linux = "0.1.2"
thiserror = "1.0" 
```
## Usage
Here's a basic example of how to use the library:

```rust
use sysinfo_linux::SystemInfo;

fn main() {
    // Get the Linux kernel version
    match SystemInfo::kernel_version() {
        Some(version) => println!("Kernel Version: {}", version),
        None => eprintln!("Failed to get kernel version"),
    }

    // Get the system uptime
    match SystemInfo::system_uptime() {
        Ok(uptime) => println!("System Uptime: {:.2} seconds", uptime),
        Err(e) => eprintln!("Error getting uptime: {}", e),
    }

    // Get the available memory
    match SystemInfo::available_memory() {
        Ok(memory) => println!("Available Memory: {} kB", memory),
        Err(e) => eprintln!("Error getting available memory: {}", e),
    }

    // Get network interface statistics
    match SystemInfo::network_interface_stats() {
        Ok(interfaces) => {
            for interface in interfaces {
                println!("Interface: {}\n  RX Bytes: {}\n  TX Bytes: {}\n",
                    interface.name, interface.rx_bytes, interface.tx_bytes);
            }
        }
        Err(e) => eprintln!("Error getting network interface statistics: {}", e),
    }
}
```
## Features
Kernel Version: Retrieve the Linux kernel version using uname.

System Uptime: Fetch the system uptime from /proc/uptime.

Available Memory: Get the available memory in kilobytes from /proc/meminfo.

Network Interface Statistics: Fetch statistics for each network interface from /proc/net/dev.

## Contributing
Feel free to submit issues or pull requests. Contributions are always welcome!

## License
This project is licensed under the MIT License

## Author
Ben Santora 
