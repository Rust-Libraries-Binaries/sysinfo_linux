use sysinfo_linux::SystemInfo;

fn main() {
    // Other system info calls...

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

