use anyhow::Result;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind, Disks, Networks};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn show() -> Result<()> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    // Wait a bit to get CPU usage
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();

    println!("{}", Theme::header("--- System Information ---"));
    
    let mut info_table = TableFormatter::create_table();
    info_table.add_row(vec![TableFormatter::header_cell("OS"), TableFormatter::value_cell(System::name().unwrap_or_default())]);
    info_table.add_row(vec![TableFormatter::header_cell("Version"), TableFormatter::value_cell(System::os_version().unwrap_or_default())]);
    info_table.add_row(vec![TableFormatter::header_cell("Kernel"), TableFormatter::value_cell(System::kernel_version().unwrap_or_default())]);
    info_table.add_row(vec![TableFormatter::header_cell("Hostname"), TableFormatter::value_cell(System::host_name().unwrap_or_default())]);
    info_table.add_row(vec![TableFormatter::header_cell("Uptime"), TableFormatter::value_cell(format!("{} seconds", System::uptime()))]);
    println!("{info_table}");
    
    println!("\n{}", Theme::header("--- CPU ---"));
    let mut cpu_table = TableFormatter::create_table();
    cpu_table.add_row(vec![TableFormatter::header_cell("Brand"), TableFormatter::value_cell(sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown"))]);
    cpu_table.add_row(vec![TableFormatter::header_cell("Cores"), TableFormatter::value_cell(sys.cpus().len().to_string())]);
    
    let global_cpu_usage = sys.global_cpu_info().cpu_usage();
    cpu_table.add_row(vec![
        TableFormatter::header_cell("Global Usage"), 
        TableFormatter::value_cell(format!("{:.2}%", global_cpu_usage))
    ]);
    println!("{cpu_table}");
    print_progress_bar(global_cpu_usage);

    println!("\n{}", Theme::header("--- Memory ---"));
    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let free_mem = sys.free_memory() / 1024 / 1024;
    let mem_percentage = (used_mem as f32 / total_mem as f32) * 100.0;
    
    let mut mem_table = TableFormatter::create_table();
    mem_table.add_row(vec![TableFormatter::header_cell("Total"), TableFormatter::value_cell(format!("{} MB", total_mem))]);
    mem_table.add_row(vec![TableFormatter::header_cell("Used"), TableFormatter::highlight_cell(format!("{} MB ({:.1}%)", used_mem, mem_percentage))]);
    mem_table.add_row(vec![TableFormatter::header_cell("Free"), TableFormatter::value_cell(format!("{} MB", free_mem))]);
    println!("{mem_table}");
    print_progress_bar(mem_percentage);

    println!("\n{}", Theme::header("--- Disks ---"));
    let mut disks_table = TableFormatter::create_table();
    disks_table.set_header(vec![
        TableFormatter::header_cell("Mount"),
        TableFormatter::header_cell("FS"),
        TableFormatter::header_cell("Usage"),
        TableFormatter::header_cell("Total"),
    ]);

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total = disk.total_space() / 1024 / 1024 / 1024;
        let available = disk.available_space() / 1024 / 1024 / 1024;
        let used = total - available;
        let usage_percentage = (used as f32 / total as f32) * 100.0;
        
        disks_table.add_row(vec![
            TableFormatter::value_cell(disk.mount_point().to_string_lossy()),
            TableFormatter::value_cell(disk.file_system().to_string_lossy()),
            TableFormatter::highlight_cell(format!("{} GB ({:.1}%)", used, usage_percentage)),
            TableFormatter::value_cell(format!("{} GB", total)),
        ]);
    }
    println!("{disks_table}");

    println!("\n{}", Theme::header("--- Networks ---"));
    let mut net_table = TableFormatter::create_table();
    net_table.set_header(vec![
        TableFormatter::header_cell("Interface"),
        TableFormatter::header_cell("Received"),
        TableFormatter::header_cell("Transmitted"),
    ]);

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        let received = data.total_received() as f64 / 1024.0 / 1024.0;
        let transmitted = data.total_transmitted() as f64 / 1024.0 / 1024.0;
        
        if received > 0.0 || transmitted > 0.0 {
            net_table.add_row(vec![
                TableFormatter::value_cell(interface_name),
                TableFormatter::value_cell(format!("{:.2} MB", received)),
                TableFormatter::value_cell(format!("{:.2} MB", transmitted)),
            ]);
        }
    }
    println!("{net_table}");

    Ok(())
}

fn print_progress_bar(percentage: f32) {
    let width = 50;
    let filled = (percentage / 100.0 * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    
    let bar = format!(
        " {}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    );
    
    let colored_bar = if percentage > 90.0 {
        Theme::error(bar)
    } else if percentage > 70.0 {
        Theme::warning(bar)
    } else {
        Theme::highlight(bar)
    };
    
    println!("{}\n", colored_bar);
}
