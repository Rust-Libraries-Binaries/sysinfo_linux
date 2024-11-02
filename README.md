# oxide_linux2

A minimalist Rust utility library for fetching common Linux system information.

## Features
- **Get Linux Kernel Version**: Retrieve the Linux kernel version using `uname`.
- **Get System Uptime**: Fetch the system uptime from `/proc/uptime`.
- **Get Available Memory**: Parse `/proc/meminfo` to get available memory in kilobytes.

## Changes and improvements over original, minimal oxide_linux
1. **Custom Error Handling**: The `OxideLinuxError` enum provides structured and meaningful error messages.
2. **SystemInfo Struct**: Encapsulates the functions for better organization and allows easy future expansion.
3. **Logging**: I've added the `log` dependency, but you can initialize it in your main application using crates like `env_logger`.
4. **Testing**: Added unit tests to ensure the correctness of each function.

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
oxide_linux2 = "0.1.0"
```
## Usage
Here's how to use oxide_linux2:
```rust
use oxide_linux2::SystemInfo;

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
## Error Handling
oxide_linux2 uses a custom error type OxideLinuxError for improved error clarity.

## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>
