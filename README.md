# sysinfo_linux

A minimalist Rust utility library for fetching common Linux system information.

## Features
- **Get Linux Kernel Version**: Retrieve the Linux kernel version using `uname`.
- **Get System Uptime**: Fetch the system uptime from `/proc/uptime`.
- **Get Available Memory**: Parse `/proc/meminfo` to get available memory in kilobytes.

## Changes and improvements over original, minimal oxide_linux
1. **Custom Error Handling**: The `SysInfoLinuxError` enum provides structured and meaningful error messages.
2. **SystemInfo Struct**: Encapsulates the functions for better organization and allows easy future expansion.
3. **Logging**: I've added the `log` dependency, but you can initialize it in your main application using crates like `env_logger`.
4. **Testing**: Added unit tests to ensure the correctness of each function.

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
sysinfo_linux = "0.1.1"  # Update to the correct version
```
## Usage
Here's how to use sysinfo_linux:
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
}
```
##Error Handling
sysinfo_linux uses a custom error type SysInfoLinuxError for improved error clarity.

## License
This project is licensed under the MIT License.

## Author
Ben Santora bensatlantik@gmail.com


### Summary of Changes:
- **Library Name**: Changed all occurrences of `oxide_linux2` to `sysinfo_linux`.
- **Error Enum Name**: Updated from `OxideLinuxError` to `SysInfoLinuxError`.
- **Version Update**: Adjusted the version in the installation instructions to reflect the new version `0.1.1`.

