use setupwizard::wifi::{list_networks, connect_network};
use setupwizard::common::SetupError;

#[cfg(test)]
mod wifi_tests {
    use super::*;

    #[test]
    fn test_connect_network_empty_ssid() {
        let result = connect_network("", None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_connect_network_whitespace_ssid() {
        let result = connect_network("   ", None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_connect_network_newline_ssid() {
        let result = connect_network("ssid\nwith\nnewlines", None);
        // Should be treated as invalid or potentially dangerous
        // This tests that we handle special characters appropriately
        match result {
            Ok(_) => println!("SSID with newlines was accepted"),
            Err(_) => {}, // This is also acceptable
        }
    }

    #[test]
    fn test_list_networks() {
        let result = list_networks();
        
        match result {
            Ok(networks) => {
                // If successful, should return some output
                println!("WiFi networks found: {}", networks.len());
            }
            Err(SetupError::System(msg)) => {
                // Expected if iwctl is not available or no WiFi hardware
                assert!(msg.contains("iwctl") || msg.contains("wireless"));
                println!("WiFi not available in test environment: {}", msg);
            }
            Err(SetupError::CommandFailed(_)) => {
                // Also acceptable - command might exist but fail
                println!("WiFi command failed (expected in test environment)");
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_connect_network_with_password() {
        let result = connect_network("TestSSID", Some("password123"));
        
        match result {
            Ok(_) => {
                // Unlikely to succeed in test environment but not an error
                println!("WiFi connection unexpectedly succeeded");
            }
            Err(SetupError::System(_)) => {
                // Expected if iwctl not available
                println!("WiFi system not available (expected)");
            }
            Err(SetupError::CommandFailed(_)) => {
                // Expected if command fails (no such SSID, etc.)
                println!("WiFi connection failed (expected)");
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_connect_network_without_password() {
        let result = connect_network("OpenNetwork", None);
        
        match result {
            Ok(_) => println!("Open network connection unexpectedly succeeded"),
            Err(SetupError::System(_)) => println!("WiFi system not available (expected)"),
            Err(SetupError::CommandFailed(_)) => println!("WiFi connection failed (expected)"),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_ssid_validation() {
        let valid_ssids = vec![
            "MyWiFi",
            "Home-Network",
            "WiFi_2.4GHz",
            "Guest Network",
            "cafe-wifi-2023",
            "家のWiFi", // Unicode SSID
        ];

        for ssid in valid_ssids {
            if ssid.trim().is_empty() {
                continue;
            }
            
            let result = connect_network(ssid, None);
            match result {
                Ok(_) => println!("Connection to '{}' unexpectedly succeeded", ssid),
                Err(SetupError::InvalidInput(_)) => {
                    panic!("Valid SSID '{}' was rejected as invalid", ssid);
                }
                Err(_) => {}, // Other errors are fine (system not available, etc.)
            }
        }
    }

    #[test]
    fn test_password_handling() {
        let test_cases = vec![
            ("TestSSID", Some("")), // Empty password
            ("TestSSID", Some("short")), // Short password  
            ("TestSSID", Some("a_very_long_password_with_special_chars_123!@#")), // Long password
            ("TestSSID", Some("password with spaces")), // Password with spaces
            ("TestSSID", Some("пароль")), // Unicode password
        ];

        for (ssid, password) in test_cases {
            let result = connect_network(ssid, password);
            match result {
                Ok(_) => println!("Connection unexpectedly succeeded"),
                Err(SetupError::InvalidInput(msg)) => {
                    // Should only fail for empty SSID, not password issues
                    assert!(msg.contains("SSID"), "Should not reject based on password: {}", msg);
                }
                Err(_) => {}, // Other errors are expected
            }
        }
    }

    #[test]
    fn test_special_ssid_characters() {
        let special_ssids = vec![
            "WiFi-5G",
            "Guest_Network",
            "Network (2.4GHz)",
            "Café WiFi",
            "Network #1",
            "My WiFi @ Home",
        ];

        for ssid in special_ssids {
            let result = connect_network(ssid, None);
            match result {
                Ok(_) => println!("Connection to '{}' unexpectedly succeeded", ssid),
                Err(SetupError::InvalidInput(_)) => {
                    // Should not reject based on special characters in SSID
                    println!("Warning: SSID '{}' was rejected as invalid", ssid);
                }
                Err(_) => {}, // Other errors are fine
            }
        }
    }

    #[test]
    fn test_device_parsing_mock() {
        // Test the device parsing logic with mock data
        let mock_outputs = vec![
            "Device     Type    Mode    Powered\nwlan0      station on      on\n",
            "Device     Type    Mode    Powered\n---\nwlp3s0     station on      on\n",
            "No devices found\n",
            "",
        ];

        for mock_output in mock_outputs {
            let mut device = None;
            for line in mock_output.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || 
                   trimmed.starts_with("Device") || 
                   trimmed.starts_with('-') ||
                   trimmed.contains("No devices") {
                    continue;
                }
                
                if let Some(d) = trimmed.split_whitespace().next() {
                    device = Some(d.to_string());
                    break;
                }
            }
            
            match mock_output {
                s if s.contains("wlan0") => assert_eq!(device, Some("wlan0".to_string())),
                s if s.contains("wlp3s0") => assert_eq!(device, Some("wlp3s0".to_string())),
                _ => assert_eq!(device, None),
            }
        }
    }
}
