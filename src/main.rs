use sysinfo::System;

fn main() {
    //Initialize system
    let mut sys = System::new();

    println!("Monitoring CPU usage...");

    loop {
        //Refresh CPU info
        sys.refresh_cpu();

        //Get CPU global use information
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        
        //Print the info
        println!("CPU: {:.1}%", cpu_usage);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
