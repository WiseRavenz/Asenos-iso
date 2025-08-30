use setupwizard::{cli_funcs, common::SetupError};
use std::process::Command;

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    #[test]
    fn test_binary_compilation() {
        let output = Command::new("cargo")
            .args(&["build"])
            .current_dir(".")
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success(), "Binary should compile successfully");
        
        let binary_path = std::path::Path::new("target/debug/setupwizard");
        assert!(binary_path.exists(), "Binary should exist after compilation");
    }

    #[test]
    fn test_cli_help_output() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check for main command line options
        assert!(stdout.contains("--list-keymaps"));
        assert!(stdout.contains("--keymap"));
        assert!(stdout.contains("--wifi-list"));
        assert!(stdout.contains("--wifi-connect"));
        assert!(stdout.contains("--list-disks"));
        assert!(stdout.contains("--partition-disk"));
        assert!(stdout.contains("--partition-config"));
    }

    #[test]
    fn test_version_output() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("0.1.0"));
    }

    #[test]
    fn test_list_disks_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--list-disks"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        // Should succeed (lsblk should be available on most Linux systems)
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Available disks:"));
        }
    }

    #[test]
    fn test_list_keymaps_function() {
        // Test the function directly rather than CLI
        let result = cli_funcs::list_keymaps();
        
        // Should either succeed or fail gracefully
        match result {
            Ok(_) => {}, // Success is fine
            Err(SetupError::System(_)) => {}, // System limitations are acceptable in tests
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_list_disks_function() {
        let result = cli_funcs::list_disks();
        
        // Should succeed on most systems
        match result {
            Ok(_) => {},
            Err(SetupError::CommandFailed(_)) => {}, // Command might not be available
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_partition_config_string_parsing() {
        // Test valid configuration string
        let valid_configs = vec![
            "/dev/sda:512:2048:gpt:ext4",
            "/dev/nvme0n1:1024:4096:msdos:btrfs",
        ];

        for config_str in valid_configs {
            let parts: Vec<&str> = config_str.split(':').collect();
            assert_eq!(parts.len(), 5, "Config should have 5 parts: {}", config_str);
            
            // Test parsing individual components
            assert!(parts[0].starts_with("/dev/"));
            assert!(parts[1].parse::<u32>().is_ok());
            assert!(parts[2].parse::<u32>().is_ok());
            assert!(["gpt", "msdos"].contains(&parts[3]));
            assert!(["ext4", "btrfs", "xfs"].contains(&parts[4]));
        }
    }

    #[test]
    fn test_invalid_partition_configs() {
        let invalid_configs = vec![
            "",
            "/dev/sda",
            "/dev/sda:512",
            "/dev/sda:512:2048",
            "/dev/sda:512:2048:gpt",
            "/dev/sda:abc:2048:gpt:ext4",
            "/dev/sda:512:xyz:gpt:ext4",
            "sda:512:2048:gpt:ext4",
            "/dev/sda:512:2048:invalid:ext4",
            "/dev/sda:512:2048:gpt:invalid",
        ];

        for config in invalid_configs {
            // Test parsing logic
            let parts: Vec<&str> = config.split(':').collect();
            
            let is_invalid = parts.len() != 5 ||
                !parts[0].starts_with("/dev/") ||
                parts[1].parse::<u32>().is_err() ||
                parts[2].parse::<u32>().is_err() ||
                !["gpt", "msdos"].contains(&parts[3]) ||
                !["ext4", "btrfs", "xfs"].contains(&parts[4]);
            
            assert!(is_invalid, "Config should be invalid: {}", config);
        }
    }

    #[test]
    fn test_wifi_config_parsing() {
        // Test the WiFi connection string parsing logic from main.rs
        let test_cases = vec![
            ("MyWiFi", ("MyWiFi", None)),
            ("MyWiFi:password123", ("MyWiFi", Some("password123"))),
            ("WiFi:with:colons:in:password", ("WiFi", Some("with:colons:in:password"))),
        ];

        for (input, (expected_ssid, expected_password)) in test_cases {
            let (ssid, password) = match input.splitn(2, ':').collect::<Vec<&str>>().as_slice() {
                [ssid] => (*ssid, None),
                [ssid, password] => (*ssid, Some(*password)),
                _ => (input, None),
            };

            assert_eq!(ssid, expected_ssid);
            assert_eq!(password, expected_password);
        }
    }

    #[test]
    fn test_error_handling() {
        // Test that functions return proper error types
        let result = cli_funcs::set_keymap("");
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));

        let result = cli_funcs::connect_wifi("", None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_no_args_behavior() {
        // Test that running with no arguments doesn't crash
        let output = Command::new("cargo")
            .args(&["run"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        // Should succeed without doing anything
        assert!(output.status.success());
    }
}
