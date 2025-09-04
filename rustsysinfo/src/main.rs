fn bytes_to_human_readable(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn main() {
    let mut sys = sysinfo::System::new();
    let mut networks = sysinfo::Networks::new();
    let mut components = sysinfo::Components::new();
    loop {
        sys.refresh_memory();
        sys.refresh_cpu_usage();
        networks.refresh(false);
        components.refresh(false);
        let global_cpu = sys.global_cpu_usage();
        let mem_used = sys.used_memory();
        let mem_total = sys.total_memory();
        println!(
            "CPU: {:.2}%, Memory: {}/{} KB",
            global_cpu, mem_used, mem_total
        );
        for (interface_name, data) in networks.iter() {
            let received_human_readable = bytes_to_human_readable(data.received());
            let transmitted_human_readable = bytes_to_human_readable(data.transmitted());
            println!(
                "Interface: {}, Received: {}, Transmitted: {}",
                interface_name, received_human_readable, transmitted_human_readable
            );
        }
        for component in components.iter() {
            println!(
                "Component: {} = {}",
                component.label(),
                component.temperature().unwrap(),
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
}
