use sysinfo::{System, Networks};
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);
const BYTE_UNITS: [&str; 4] = ["B/s", "KB/s", "MB/s", "GB/s"];
const BYTES_PER_UNIT: f64 = 1024.0;
const ASCII_ART: &str = r#"
                                  _       _         _             
 ___ _   _ ___     __      ____ _| |_ ___| |__   __| | ___   __ _ 
/ __| | | / __|____\ \ /\ / / _` | __/ __| '_ \ / _` |/ _ \ / _` |
\__ \ |_| \__ \_____\ V  V / (_| | || (__| | | | (_| | (_) | (_| |
|___/\__, |___/      \_/\_/ \__,_|\__\___|_| |_|\__,_|\___/ \__, |
     |___/                                                  |___/ 

Created by: Anderson Álvarez
"#;

struct NetworkInterface
{
    name: String,
    rx_speed: f64,
    tx_speed: f64,
    prev_rx: u64,
    prev_tx: u64,
}

impl NetworkInterface
{
    fn new(name: String) -> Self
    {
        Self
        {
            name,
            rx_speed: 0.0,
            tx_speed: 0.0,
            prev_rx: 0,
            prev_tx: 0,
        }    
    }    
}

struct SystemMetrics
{
    network_interfaces: HashMap<String, NetworkInterface>,
    cpu_usage: f32,
    ram_usage: f32,
    used_ram: u64,
    total_ram: u64,
}

impl SystemMetrics
{
    fn new() -> Self
    {
        Self
        {
            network_interfaces: HashMap::new(),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            used_ram: 0,
            total_ram: 0,
        }    
    }

    fn update(&mut self, sys: &System, networks: &Networks)
    {
        for(name, data) in networks.iter()
        {
            let interface = self.network_interfaces
                .entry(name.to_string())
                .or_insert_with(||
                {
                    let mut new_interface = NetworkInterface::new(name.to_string());
                    new_interface.prev_rx = data.received();
                    new_interface.prev_tx = data.transmitted();
                    new_interface
                });                

            let curr_rx = data.received();
            let curr_tx = data.transmitted();

            interface.rx_speed = (curr_rx.saturating_sub(interface.prev_rx)) as f64;
            interface.tx_speed = (curr_tx.saturating_sub(interface.prev_tx)) as f64;
            
            interface.prev_rx = curr_rx;
            interface.prev_tx = curr_tx;
        }

        self.cpu_usage = sys.global_cpu_info().cpu_usage();
        self.total_ram = sys.total_memory() / 1024 / 1024;
        self.used_ram = (sys.total_memory() - sys.available_memory()) / 1024 / 1024;
        self.ram_usage = (self.used_ram as f32 / self.total_ram as f32) * 100.0;

    }

    fn display(&self) -> io::Result<()> 
    {
        print!("\r\x1B[K");
        
        let mut output = format!(
            "CPU: {:.1}% | RAM: {:.1}% ({} MB / {} MB)", 
            self.cpu_usage,
            self.ram_usage,
            self.used_ram,
            self.total_ram
        );

        for interface in self.network_interfaces.values()
        {
            if interface.rx_speed > 0.0 || interface.tx_speed > 0.0
            {
                output.push_str(&format!(
                    " | {} ↓ {} ↑ {}", 
                    interface.name,
                    format_bytes(interface.rx_speed),
                    format_bytes(interface.tx_speed)
                ));
            }
        }

        print!("{}", output);
        io::stdout().flush()
    }
}

fn format_bytes(mut bytes: f64) -> String
{
    let mut unit_index = 0;
    while bytes >= BYTES_PER_UNIT && unit_index < BYTE_UNITS.len() - 1
    {
        bytes /= BYTES_PER_UNIT;
        unit_index += 1;
    }
    format!("{:.1}{}", bytes, BYTE_UNITS[unit_index])
}

fn main() -> io::Result<()>
{
    let mut sys = System::new();
    let mut networks = Networks::new_with_refreshed_list();
    let mut metrics = SystemMetrics::new();

    println!("{}", ASCII_ART);
    println!("Monitoring system\n");
    
    for (name, data) in networks.iter()
    {
        let mut interface = NetworkInterface::new(name.to_string());
        interface.prev_rx = data.received();
        interface.prev_tx = data.transmitted();
        metrics.network_interfaces.insert(name.to_string(), interface);
    }

    loop
    {
        networks.refresh_list();
        sys.refresh_cpu();
        sys.refresh_memory();

        metrics.update(&sys, &networks);
        metrics.display()?;

        thread::sleep(UPDATE_INTERVAL);
    }
}
