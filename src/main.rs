use sysinfo::{System, Networks};
use std::io::{self, Write};

fn format_bytes(bytes: f64) -> String {
    const UNITS: [&str; 4] = ["B/s", "KB/s", "MB/s", "GB/s"];
    let mut bytes = bytes;
    let mut unit_index = 0;

    while bytes >= 1024.0 && unit_index < UNITS.len() - 1 {
        bytes /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1}{}", bytes, UNITS[unit_index])
}

fn main() {
    let alien_art = r#"
                                  _       _         _             
 ___ _   _ ___     __      ____ _| |_ ___| |__   __| | ___   __ _ 
/ __| | | / __|____\ \ /\ / / _` | __/ __| '_ \ / _` |/ _ \ / _` |
\__ \ |_| \__ \_____\ V  V / (_| | || (__| | | | (_| | (_) | (_| |
|___/\__, |___/      \_/\_/ \__,_|\__\___|_| |_|\__,_|\___/ \__, |
     |___/                                                  |___/ 

Created by: Anderson Álvarez
"#;

    let mut sys = System::new();
    
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", alien_art);
    println!("Monitoring system");

    let mut networks = Networks::new_with_refreshed_list();
    
    let curr_rx: u64 = networks.iter().map(|(_, data)| data.received()).sum();
    let curr_tx: u64 = networks.iter().map(|(_, data)| data.transmitted()).sum();
    
    let mut prev_rx = curr_rx;
    let mut prev_tx = curr_tx;

    loop {
        // Updating the network, CPU and memory info
        networks.refresh_list();
        sys.refresh_cpu();
        sys.refresh_memory();

        // Get the actual state of the network
        let curr_rx: u64 = networks.iter().map(|(_, data)| data.received()).sum();
        let curr_tx: u64 = networks.iter().map(|(_, data)| data.transmitted()).sum();
        
        // Calculate the network speed
        let rx_speed = curr_rx.saturating_sub(prev_rx) as f64;
        let tx_speed = curr_tx.saturating_sub(prev_tx) as f64;

        // Update network speed values
        prev_rx = curr_rx;
        prev_tx = curr_tx;

        // CPU and RAM information
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let total_ram = sys.total_memory() / 1024 / 1024;
        let used_ram = (sys.total_memory() - sys.available_memory()) / 1024 / 1024;
        let ram_usage = (used_ram as f32 / total_ram as f32) * 100.0;

        // Preparing the output
        print!("\r\x1B[KCPU: {:.1}% | RAM: {:.1}% ({} MB / {} MB) | ↓ {} ↑ {}", 
            cpu_usage,
            ram_usage, used_ram, total_ram,
            format_bytes(rx_speed),
            format_bytes(tx_speed));

        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
