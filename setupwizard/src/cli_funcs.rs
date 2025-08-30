use crate::keymap;
use crate::partition;
use crate::wifi;

pub fn list_keymaps() -> Result<(), String> {
    match keymap::available_keymaps() {
        Ok(list) => {
            println!("Available keymaps:");
            for km in list {
                println!(" - {}", km);
            }
        }
        Err(e) => {
            eprintln!("Failed to discover keymaps: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn keymap_set(map: &str) -> Result<(), String> {
    match keymap::set_keymap(map) {
        Ok(()) => {
            println!("Keymap set to '{}'.", map);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to set keymap: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn wifi_list() -> Result<(), String> {
    wifi::list_ssids()
}

pub fn wifi_connect(ssid: &str, passwd: Option<&str>) -> Result<(), String> {
    wifi::connect(ssid, passwd)
}

pub fn list_disks() -> Result<(), String> {
    match partition::list_disks() {
        Ok(disks) => {
            println!("Available disks:");
            println!("{}", disks);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to list disks: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn partition_disk_interactive() -> Result<(), String> {
    use std::io::{self, Write};
    
    println!("=== Simple Partition Wizard ===");
    
    // List disks
    partition::list_disks().map(|disks| println!("Available disks:\n{}", disks))?;
    
    // Get inputs
    print!("Disk (e.g., /dev/sda): ");
    io::stdout().flush().unwrap();
    let mut disk = String::new();
    io::stdin().read_line(&mut disk).unwrap();
    
    print!("Boot size MB (default 512): ");
    io::stdout().flush().unwrap();
    let mut boot = String::new();
    io::stdin().read_line(&mut boot).unwrap();
    let boot_size_mb = boot.trim().parse().unwrap_or(512);
    
    print!("Swap size MB (default 2048): ");
    io::stdout().flush().unwrap();
    let mut swap = String::new();
    io::stdin().read_line(&mut swap).unwrap();
    let swap_size_mb = swap.trim().parse().unwrap_or(2048);
    
    print!("Use GPT? (y/n, default y): ");
    io::stdout().flush().unwrap();
    let mut gpt = String::new();
    io::stdin().read_line(&mut gpt).unwrap();
    let use_gpt = gpt.trim().to_lowercase() != "n";
    
    print!("Filesystem (ext4/btrfs/xfs, default ext4): ");
    io::stdout().flush().unwrap();
    let mut fs = String::new();
    io::stdin().read_line(&mut fs).unwrap();
    let filesystem = if fs.trim().is_empty() { "ext4".to_string() } else { fs.trim().to_string() };
    
    let config = partition::PartitionConfig {
        disk: disk.trim().to_string(),
        boot_size_mb,
        swap_size_mb,
        use_gpt,
        filesystem,
    };
    
    // Validate and create
    partition::validate_config(&config)?;
    partition::create_partitions(&config)?;
    
    // Show result
    if let Ok(info) = partition::get_partition_info(&config.disk) {
        println!("\nResult:\n{}", info);
    }
    
    Ok(())
}

pub fn partition_disk_config(config_str: &str) -> Result<(), String> {
    // Parse: "disk:boot_size:swap_size:gpt/msdos:filesystem"
    let parts: Vec<&str> = config_str.split(':').collect();
    
    if parts.len() != 5 {
        return Err("Format: disk:boot_size:swap_size:gpt/msdos:filesystem".to_string());
    }
    
    let config = partition::PartitionConfig {
        disk: parts[0].to_string(),
        boot_size_mb: parts[1].parse().map_err(|_| "Invalid boot size")?,
        swap_size_mb: parts[2].parse().map_err(|_| "Invalid swap size")?,
        use_gpt: parts[3] == "gpt",
        filesystem: parts[4].to_string(),
    };
    
    // Validate and create
    partition::validate_config(&config)?;
    println!("Creating partitions on {} with {} table...", config.disk, 
        if config.use_gpt { "GPT" } else { "MBR" });
    partition::create_partitions(&config)?;
    
    // Show result
    if let Ok(info) = partition::get_partition_info(&config.disk) {
        println!("\nResult:\n{}", info);
    }
    
    Ok(())
}
