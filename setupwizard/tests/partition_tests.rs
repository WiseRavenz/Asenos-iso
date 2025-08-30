use setupwizard::partition::*;
use setupwizard::common::SetupError;

#[cfg(test)]
mod partition_tests {
    use super::*;

    #[test]
    fn test_partition_config_creation() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        assert_eq!(config.disk, "/dev/sda");
        assert_eq!(config.boot_size_mb, 512);
        assert_eq!(config.swap_size_mb, 2048);
        assert!(config.use_gpt);
        assert_eq!(config.filesystem, "ext4");
    }

    #[test]
    fn test_config_from_string_invalid_format() {
        let invalid_configs = vec![
            "",
            "/dev/sda",
            "/dev/sda:512",
            "/dev/sda:512:2048",
            "/dev/sda:512:2048:gpt",
            "too:many:parts:here:gpt:ext4:extra",
        ];

        for config_str in invalid_configs {
            let result = PartitionConfig::from_string(config_str);
            assert!(result.is_err(), "Should fail for: {}", config_str);
            matches!(result.unwrap_err(), SetupError::InvalidInput(_));
        }
    }

    #[test]
    fn test_config_from_string_invalid_numbers() {
        let invalid_configs = vec![
            "/dev/sda:abc:2048:gpt:ext4",
            "/dev/sda:512:xyz:gpt:ext4",
            "/dev/sda:-1:2048:gpt:ext4",
            "/dev/sda:512:-1:gpt:ext4",
        ];

        for config_str in invalid_configs {
            let result = PartitionConfig::from_string(config_str);
            assert!(result.is_err(), "Should fail for: {}", config_str);
        }
    }

    #[test]
    fn test_validate_config_invalid_disk_path() {
        let config = PartitionConfig::new(
            "sda".to_string(), // Missing /dev/
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        let result = config.validate();
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_validate_config_boot_size_bounds() {
        // Too small
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            50,
            2048,
            true,
            "ext4".to_string(),
        );
        assert!(config.validate().is_err());

        // Too large
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            3000,
            2048,
            true,
            "ext4".to_string(),
        );
        assert!(config.validate().is_err());

        // Valid range
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );
        // This will fail because /dev/sda doesn't exist in test environment
        // but it tests the size validation logic
        let result = config.validate();
        if let Err(SetupError::InvalidInput(msg)) = result {
            assert!(!msg.contains("Boot size"));
        }
    }

    #[test]
    fn test_validate_config_swap_size() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            100, // Too small
            true,
            "ext4".to_string(),
        );

        let result = config.validate();
        assert!(result.is_err());
        if let Err(SetupError::InvalidInput(msg)) = result {
            assert!(msg.contains("Swap size"));
        }
    }

    #[test]
    fn test_validate_config_invalid_filesystem() {
        let invalid_filesystems = vec!["fat32", "ntfs", "invalid", ""];

        for fs in invalid_filesystems {
            let config = PartitionConfig::new(
                "/dev/sda".to_string(),
                512,
                2048,
                true,
                fs.to_string(),
            );

            let result = config.validate();
            assert!(result.is_err(), "Should fail for filesystem: {}", fs);
            if let Err(SetupError::InvalidInput(msg)) = result {
                assert!(msg.contains("Filesystem"));
            }
        }
    }

    #[test]
    fn test_get_partition_names_nvme() {
        let config = PartitionConfig::new(
            "/dev/nvme0n1".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        let (boot, swap, root) = config.get_partition_names();
        assert_eq!(boot, "/dev/nvme0n1p1");
        assert_eq!(swap, "/dev/nvme0n1p2");
        assert_eq!(root, "/dev/nvme0n1p3");
    }

    #[test]
    fn test_get_partition_names_mmc() {
        let config = PartitionConfig::new(
            "/dev/mmcblk0".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        let (boot, swap, root) = config.get_partition_names();
        assert_eq!(boot, "/dev/mmcblk0p1");
        assert_eq!(swap, "/dev/mmcblk0p2");
        assert_eq!(root, "/dev/mmcblk0p3");
    }

    #[test]
    fn test_get_partition_names_sata() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        let (boot, swap, root) = config.get_partition_names();
        assert_eq!(boot, "/dev/sda1");
        assert_eq!(swap, "/dev/sda2");
        assert_eq!(root, "/dev/sda3");
    }

    #[test]
    fn test_clone_and_debug() {
        let config = PartitionConfig::new(
            "/dev/sda".to_string(),
            512,
            2048,
            true,
            "ext4".to_string(),
        );

        let cloned = config.clone();
        assert_eq!(config.disk, cloned.disk);
        assert_eq!(config.boot_size_mb, cloned.boot_size_mb);
        assert_eq!(config.swap_size_mb, cloned.swap_size_mb);
        assert_eq!(config.use_gpt, cloned.use_gpt);
        assert_eq!(config.filesystem, cloned.filesystem);

        // Test debug formatting
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("PartitionConfig"));
        assert!(debug_str.contains("/dev/sda"));
    }

    #[test]
    fn test_valid_filesystems() {
        let valid_filesystems = vec!["ext4", "btrfs", "xfs"];

        for fs in valid_filesystems {
            let config = PartitionConfig::new(
                "/dev/sda".to_string(),
                512,
                2048,
                true,
                fs.to_string(),
            );

            // Will fail due to non-existent disk but filesystem validation should pass
            let result = config.validate();
            if let Err(SetupError::InvalidInput(msg)) = result {
                assert!(!msg.contains("Filesystem"), "Filesystem {} should be valid", fs);
            }
        }
    }
}
