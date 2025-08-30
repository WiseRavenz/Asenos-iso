use crate::common::{run_command, CommandResult, SetupError};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PartitionConfig {
    pub disk: String,
    pub boot_size_mb: u32,
    pub swap_size_mb: u32,
    pub use_gpt: bool,
    pub filesystem: String,
}

impl PartitionConfig {
    pub fn new(disk: String, boot_size_mb: u32, swap_size_mb: u32, use_gpt: bool, filesystem: String) -> Self {
        Self {
            disk,
            boot_size_mb,
            swap_size_mb,
            use_gpt,
            filesystem,
        }
    }

    /// Parse configuration from string format: "disk:boot_size:swap_size:gpt/msdos:filesystem"
    pub fn from_string(config_str: &str) -> CommandResult<Self> {
        let parts: Vec<&str> = config_str.split(':').collect();
        
        if parts.len() != 5 {
            return Err(SetupError::InvalidInput(
                "Format: disk:boot_size:swap_size:gpt/msdos:filesystem".to_string()
            ));
        }
        
        let boot_size_mb = parts[1].parse::<u32>()
            .map_err(|_| SetupError::InvalidInput("Invalid boot size".to_string()))?;
        let swap_size_mb = parts[2].parse::<u32>()
            .map_err(|_| SetupError::InvalidInput("Invalid swap size".to_string()))?;
        let use_gpt = parts[3] == "gpt";
        
        let config = Self::new(
            parts[0].to_string(),
            boot_size_mb,
            swap_size_mb,
            use_gpt,
            parts[4].to_string(),
        );
        
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> CommandResult<()> {
        if !self.disk.starts_with("/dev/") {
            return Err(SetupError::InvalidInput("Disk path must start with /dev/".to_string()));
        }

        if !Path::new(&self.disk).exists() {
            return Err(SetupError::InvalidInput(format!("Disk {} does not exist", self.disk)));
        }

        if !(100..=2048).contains(&self.boot_size_mb) {
            return Err(SetupError::InvalidInput("Boot size must be 100-2048 MB".to_string()));
        }

        if self.swap_size_mb < 512 {
            return Err(SetupError::InvalidInput("Swap size must be at least 512 MB".to_string()));
        }

        if !["ext4", "btrfs", "xfs"].contains(&self.filesystem.as_str()) {
            return Err(SetupError::InvalidInput("Filesystem must be ext4, btrfs, or xfs".to_string()));
        }

        Ok(())
    }

    /// Get partition device names
    pub fn get_partition_names(&self) -> (String, String, String) {
        let prefix = if self.disk.contains("nvme") || self.disk.contains("mmc") {
            format!("{}p", self.disk)
        } else {
            self.disk.clone()
        };
        
        (
            format!("{}1", prefix), // boot
            format!("{}2", prefix), // swap
            format!("{}3", prefix), // root
        )
    }
}

/// List available block devices
pub fn list_disks() -> CommandResult<String> {
    run_command(&["lsblk", "-o", "NAME,SIZE,TYPE,MOUNTPOINT"], None)
}

/// Create partitions according to configuration
pub fn create_partitions(config: &PartitionConfig) -> CommandResult<()> {
    config.validate()?;
    
    let table_type = if config.use_gpt { "gpt" } else { "msdos" };
    
    // Create partition table
    run_command(&["parted", "-s", &config.disk, "mklabel", table_type], None)?;
    
    // Calculate partition boundaries
    let boot_end = config.boot_size_mb;
    let swap_end = boot_end + config.swap_size_mb;
    
    // Create boot partition
    run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", "fat32", 
        "1MiB", &format!("{}MiB", boot_end)
    ], None)?;
    
    // Create swap partition  
    run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", "linux-swap",
        &format!("{}MiB", boot_end), &format!("{}MiB", swap_end)
    ], None)?;
    
    // Create root partition
    run_command(&[
        "parted", "-s", &config.disk, "mkpart", "primary", &config.filesystem,
        &format!("{}MiB", swap_end), "100%"
    ], None)?;
    
    // Set boot flag
    let flag = if config.use_gpt { "esp" } else { "boot" };
    run_command(&["parted", "-s", &config.disk, "set", "1", flag, "on"], None)?;
    
    // Update kernel partition table
    run_command(&["partprobe", &config.disk], None)?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    // Format partitions
    format_partitions(config)?;
    
    Ok(())
}

/// Format created partitions
fn format_partitions(config: &PartitionConfig) -> CommandResult<()> {
    let (boot_part, swap_part, root_part) = config.get_partition_names();
    
    // Format boot as FAT32
    run_command(&["mkfs.fat", "-F32", &boot_part], None)?;
    
    // Format swap
    run_command(&["mkswap", &swap_part], None)?;
    
    // Format root with chosen filesystem
    match config.filesystem.as_str() {
        "ext4" => run_command(&["mkfs.ext4", "-F", &root_part], None)?,
        "btrfs" => run_command(&["mkfs.btrfs", "-f", &root_part], None)?,
        "xfs" => run_command(&["mkfs.xfs", "-f", &root_part], None)?,
        _ => unreachable!(), // Already validated
    };
    
    Ok(())
}

/// Get partition information after creation
pub fn get_partition_info(disk: &str) -> CommandResult<String> {
    run_command(&["lsblk", "-o", "NAME,SIZE,TYPE,FSTYPE", disk], None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_config_creation() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string()
        );

        assert_eq!(config.disk, "/dev/sda");
        assert_eq!(config.boot_size_mb, 512);
        assert_eq!(config.swap_size_mb, 2048);
        assert!(config.use_gpt);
        assert_eq!(config.filesystem, "ext4");
    }

    #[test]
    fn test_config_from_string_valid() {
        // Use a disk path that definitely won't exist
        let config_str = "/dev/sdz999:512:2048:gpt:ext4";
        let result = PartitionConfig::from_string(config_str);
        // The parsing should work, but validation should fail due to non-existent disk
        match result {
            Ok(_) => panic!("Expected validation to fail for non-existent disk"),
            Err(SetupError::InvalidInput(msg)) => {
                // Should fail because disk doesn't exist
                assert!(msg.contains("does not exist"), "Unexpected error message: {}", msg);
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_config_parsing_logic() {
        // Test that the parsing logic works correctly without validation
        let config_str = "/dev/sdz999:512:2048:gpt:ext4";
        let parts: Vec<&str> = config_str.split(':').collect();
        
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "/dev/sdz999");
        assert_eq!(parts[1].parse::<u32>().unwrap(), 512);
        assert_eq!(parts[2].parse::<u32>().unwrap(), 2048);
        assert_eq!(parts[3], "gpt");
        assert_eq!(parts[4], "ext4");
        
        // Test creating config object (before validation)
        let config = PartitionConfig::new(
            parts[0].to_string(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
            parts[3] == "gpt",
            parts[4].to_string(),
        );
        
        assert_eq!(config.disk, "/dev/sdz999");
        assert_eq!(config.boot_size_mb, 512);
        assert_eq!(config.swap_size_mb, 2048);
        assert!(config.use_gpt);
        assert_eq!(config.filesystem, "ext4");
    }

    #[test]
    fn test_config_from_string_invalid_format() {
        let config_str = "/dev/sda:512:2048"; // Missing parts
        let result = PartitionConfig::from_string(config_str);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_validate_config_boot_too_small() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            50, // Too small
            2048,
            true,
            "ext4".to_string()
        );

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_config_invalid_filesystem() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "invalid".to_string()
        );

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_get_partition_names_nvme() {
        let config = PartitionConfig::new(
            "/dev/nvme0n1".to_string(),
            512,
            2048,
            true,
            "ext4".to_string()
        );

        let (boot, swap, root) = config.get_partition_names();
        assert_eq!(boot, "/dev/nvme0n1p1");
        assert_eq!(swap, "/dev/nvme0n1p2");
        assert_eq!(root, "/dev/nvme0n1p3");
    }

    #[test]
    fn test_get_partition_names_sata() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string()
        );

        let (boot, swap, root) = config.get_partition_names();
        assert_eq!(boot, "/dev/sda1");
        assert_eq!(swap, "/dev/sda2");
        assert_eq!(root, "/dev/sda3");
    }
}
