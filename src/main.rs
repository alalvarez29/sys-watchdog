use sysinfo::System;

fn main() {
    // Inicializar sistema
    let mut sys = System::new();

    println!("Monitoreando uso de CPU...");
    println!("Presiona Ctrl+C para detener\n");

    loop {
        // Refrescar CPU info
        sys.refresh_cpu();

        // Obtener el uso global de CPU
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        
        // Mostrar el porcentaje
        println!("CPU: {:.1}%", cpu_usage);

        // Esperar un segundo antes de la siguiente lectura
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
