use anyhow::Result;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind, Disks};
use owo_colors::OwoColorize;

pub fn show() -> Result<()> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    // Wait a bit to get CPU usage
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();

    println!("{}", "--- System Information ---".bold().cyan());
    
    println!("{:<15}: {}", "OS Name", System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("{:<15}: {}", "OS Version", System::os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("{:<15}: {}", "Kernel", System::kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("{:<15}: {}", "Hostname", System::host_name().unwrap_or_else(|| "Unknown".to_string()));
    
    println!("\n{}", "--- CPU ---".bold().green());
    println!("{:<15}: {}", "Brand", sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown"));
    println!("{:<15}: {}", "Cores", sys.cpus().len());
    
    let global_cpu_usage = sys.global_cpu_info().cpu_usage();
    println!("{:<15}: {:.2}%", "Global Usage", global_cpu_usage);

    println!("\n{}", "--- Memory ---".bold().yellow());
    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let free_mem = sys.free_memory() / 1024 / 1024;
    
    println!("{:<15}: {:>8} MB", "Total", total_mem);
    println!("{:<15}: {:>8} MB ({:.1}%)", "Used", used_mem, (used_mem as f64 / total_mem as f64) * 100.0);
    println!("{:<15}: {:>8} MB", "Free", free_mem);

    println!("\n{}", "--- Disks ---".bold().magenta());
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total = disk.total_space() / 1024 / 1024 / 1024;
        let available = disk.available_space() / 1024 / 1024 / 1024;
        let used = total - available;
        let mount = disk.mount_point().to_string_lossy();
        
        println!("{:<15}: {} ({})", "Mount", mount.green(), disk.file_system().to_string_lossy());
        println!("{:<15}: {:>8} GB / {:>8} GB ({:.1}%)", "Usage", used, total, (used as f64 / total as f64) * 100.0);
    }

    Ok(())
}
