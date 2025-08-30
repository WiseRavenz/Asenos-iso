use crate::common;

#[derive(Debug, Clone)]
pub struct PartitionConfig {
    pub disk: String,
    pub boot_size_mb: u32,
    pub swap_size_mb: u32,
    pub use_gpt: bool,
    pub filesystem: String,
}

pub fn list_disks() -> Result<String, String> {
    common::run_command(&["lsblk", "-o", "NAME,SIZE,TYPE,MOUNTPOINT"], None)
}

/// Create partitions: boot (FAT32), swap, and root with specified filesystem
pub fn create_partitions(config: &PartitionConfig) -> Result<(), String> {
    let table_type = if config.use_gpt { "gpt" } else { "msdos" };
    
    // Basic validation
    if !config.disk.starts_with("/dev/") {
        return Err("Disk path must start with /dev/".to_string());
    }
    
    println!("Creating {} partition table on {}", table_type, config.disk);
    
    // Create partition table
    common::run_command(&["parted", "-s", &config.disk, "mklabel", table_type], None)?;
    
    // Create partitions
    let boot_end = config.boot_size_mb;
    let swap_end = boot_end + config.swap_size_mb;
    
    // Boot partition
    common::run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", "fat32", 
        "1MiB", &format!("{}MiB", boot_end)
    ], None)?;
    
    // Swap partition  
    common::run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", "linux-swap",
        &format!("{}MiB", boot_end), &format!("{}MiB", swap_end)
    ], None)?;
    
    // Root partition (remaining space)
    common::run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", &config.filesystem,
        &format!("{}MiB", swap_end), "100%"
    ], None)?;
    
    // Set boot flag
    common::run_command(&["parted", "-s", &config.disk, "set", "1", 
        if config.use_gpt { "esp" } else { "boot" }, "on"], None)?;
    
    // Update kernel
    common::run_command(&["partprobe", &config.disk], None)?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    // Format partitions
    let prefix = if config.disk.contains("nvme") || config.disk.contains("mmc") {
        format!("{}p", config.disk)
    } else {
        config.disk.clone()
    };
    
    // Format boot as FAT32
    common::run_command(&["mkfs.fat", "-F32", &format!("{}1", prefix)], None)?;
    
    // Format swap
    common::run_command(&["mkswap", &format!("{}2", prefix)], None)?;
    
    // Format root with chosen filesystem
    let root_part = format!("{}3", prefix);
    match config.filesystem.as_str() {
        "ext4" => common::run_command(&["mkfs.ext4", "-F", &root_part], None)?,
        "btrfs" => common::run_command(&["mkfs.btrfs", "-f", &root_part], None)?,
        "xfs" => common::run_command(&["mkfs.xfs", "-f", &root_part], None)?,
        _ => return Err(format!("Unsupported filesystem: {}", config.filesystem)),
    };
    
    println!("Partitions created successfully!");
    Ok(())
}

/// Simple validation
pub fn validate_config(config: &PartitionConfig) -> Result<(), String> {
    if config.boot_size_mb < 100 || config.boot_size_mb > 2048 {
        return Err("Boot size must be 100-2048 MB".to_string());
    }
    if config.swap_size_mb < 512 {
        return Err("Swap size must be at least 512 MB".to_string());
    }
    if !["ext4", "btrfs", "xfs"].contains(&config.filesystem.as_str()) {
        return Err("Filesystem must be ext4, btrfs, or xfs".to_string());
    }
    Ok(())
}

/// Get partition info after creation
pub fn get_partition_info(disk: &str) -> Result<String, String> {
    common::run_command(&["lsblk", "-o", "NAME,SIZE,TYPE,FSTYPE", disk], None)
}
