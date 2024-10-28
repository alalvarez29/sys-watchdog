use sysinfo::System;
use std::io::{self, Write};

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

    //Initialize system
    let mut sys = System::new();

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", alien_art);
    println!("Monitoring system...");

    loop {
        //Refresh CPU info
        sys.refresh_cpu();
        //Refresh memory info
        sys.refresh_memory();

        //Get CPU global use information
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let total_ram = sys.total_memory() / 1024 / 1024;
        let used_ram = (sys.total_memory() - sys.available_memory()) / 1024         /1024;
        let ram_usage = (used_ram as f32 / total_ram as f32) * 100.0;

        print!("\r\x1B[K");
        
        //Print the info
        print!("CPU: {:.1}% | RAM: {:.1}% ({} MB / {} MB)", 
        cpu_usage, ram_usage, used_ram, total_ram);

        io::stdout().flush().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
