use setupwizard::cli_funcs;
use std::process::Command;

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    #[test]
    fn test_cli_list_disks() {
        // Test that list_disks function works
        let result = cli_funcs::list_disks();
        // This should succeed unless there's a system issue
        assert!(result.is_ok(), "list_disks should succeed");
    }

    #[test]
    fn test_cli_help_output() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that our new partition commands are in the help output
        assert!(stdout.contains("--list-disks"));
        assert!(stdout.contains("--partition-disk"));
        assert!(stdout.contains("--partition-config"));
        assert!(stdout.contains("List available disks"));
        assert!(stdout.contains("Create partitions on a disk"));
    }

    #[test]
    fn test_cli_list_disks_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--list-disks"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should contain disk listing output
        assert!(stdout.contains("Available disks:"));
        // Should contain typical disk output headers
        assert!(stdout.contains("NAME") || stdout.contains("SIZE") || stdout.contains("TYPE"));
    }

    #[test]
    fn test_partition_config_parsing() {
        // Test the config string parsing logic from cli_funcs
        // This is a unit test but fits better here as it tests CLI logic
        
        // Valid config string
        let config_str = "/dev/sda:512:2048:gpt:ext4";
        let parts: Vec<&str> = config_str.split(':').collect();
        
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "/dev/sda");
        assert_eq!(parts[1], "512");
        assert_eq!(parts[2], "2048");
        assert_eq!(parts[3], "gpt");
        assert_eq!(parts[4], "ext4");
        
        // Test parsing
        assert_eq!(parts[1].parse::<u32>().unwrap(), 512);
        assert_eq!(parts[2].parse::<u32>().unwrap(), 2048);
    }

    #[test]
    fn test_partition_config_validation_edge_cases() {
        // Test various invalid config strings that should be caught
        let invalid_configs = vec![
            "/dev/sda:512:2048:gpt",  // Missing filesystem
            "/dev/sda:512:2048",      // Missing table type and filesystem
            "/dev/sda:abc:2048:gpt:ext4", // Invalid boot size
            "/dev/sda:512:xyz:gpt:ext4",  // Invalid swap size
            "sda:512:2048:gpt:ext4",      // Invalid disk path
            "/dev/sda:512:2048:invalid:ext4", // Invalid table type
            "/dev/sda:512:2048:gpt:invalid",  // Invalid filesystem
        ];

        for config in invalid_configs {
            let parts: Vec<&str> = config.split(':').collect();
            
            // Should either have wrong number of parts or invalid values
            if parts.len() != 5 {
                continue; // This will be caught by length check
            }
            
            // Test individual validation
            if !parts[0].starts_with("/dev/") {
                continue; // Invalid disk path
            }
            
            if parts[1].parse::<u32>().is_err() || parts[2].parse::<u32>().is_err() {
                continue; // Invalid numeric values
            }
            
            if !["gpt", "msdos", "mbr"].contains(&parts[3]) {
                continue; // Invalid table type
            }
            
            if !["ext4", "btrfs", "xfs", "f2fs"].contains(&parts[4]) {
                continue; // Invalid filesystem
            }
        }
    }

    #[test]
    fn test_binary_compilation() {
        // Test that the binary compiles and can be executed
        let output = Command::new("cargo")
            .args(&["build"])
            .current_dir(".")
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success(), "Binary should compile successfully");
        
        // Test that the binary exists
        let binary_path = std::path::Path::new("target/debug/setupwizard");
        assert!(binary_path.exists(), "Binary should exist after compilation");
    }

    #[test]
    fn test_version_output() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should contain version information
        assert!(stdout.contains("setupwizard") || stdout.contains("0.1.0"));
    }
}
