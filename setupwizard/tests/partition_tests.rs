use setupwizard::partition::*;

#[cfg(test)]
mod simple_partition_tests {
    use super::*;

    #[test]
    fn test_partition_config_creation() {
        let config = PartitionConfig {
            disk: "/dev/sda".to_string(),
            boot_size_mb: 512,
            swap_size_mb: 2048,
            use_gpt: true,
            filesystem: "ext4".to_string(),
        };

        assert_eq!(config.disk, "/dev/sda");
        assert_eq!(config.boot_size_mb, 512);
        assert_eq!(config.swap_size_mb, 2048);
        assert!(config.use_gpt);
        assert_eq!(config.filesystem, "ext4");
    }

    #[test]
    fn test_validate_config_valid() {
        let config = PartitionConfig {
            disk: "/dev/sda".to_string(),
            boot_size_mb: 512,
            swap_size_mb: 2048,
            use_gpt: true,
            filesystem: "ext4".to_string(),
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_boot_too_small() {
        let config = PartitionConfig {
            disk: "/dev/sda".to_string(),
            boot_size_mb: 50, // Too small
            swap_size_mb: 2048,
            use_gpt: true,
            filesystem: "ext4".to_string(),
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_filesystem() {
        let config = PartitionConfig {
            disk: "/dev/sda".to_string(),
            boot_size_mb: 512,
            swap_size_mb: 2048,
            use_gpt: true,
            filesystem: "invalid".to_string(),
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_clone() {
        let config = PartitionConfig {
            disk: "/dev/sda".to_string(),
            boot_size_mb: 512,
            swap_size_mb: 2048,
            use_gpt: true,
            filesystem: "ext4".to_string(),
        };

        let cloned = config.clone();
        assert_eq!(config.disk, cloned.disk);
        assert_eq!(config.use_gpt, cloned.use_gpt);
    }
}
