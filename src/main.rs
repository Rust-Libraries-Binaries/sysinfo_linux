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
