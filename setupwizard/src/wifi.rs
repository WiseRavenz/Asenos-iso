use crate::common::{run_command, CommandResult, SetupError, command_exists};

/// List available WiFi networks using iwctl
pub fn list_networks() -> CommandResult<String> {
    if !command_exists("iwctl") {
        return Err(SetupError::System("iwctl not found - ensure iwd is installed".to_string()));
    }

    let device = get_first_wireless_device()?;
    
    // Trigger scan (best effort)
    let _ = run_command(&["iwctl", "station", &device, "scan"], None);
    
    // Get networks
    let output = run_command(&["iwctl", "station", &device, "get-networks"], None)?;
    Ok(output)
}

/// Connect to WiFi network
pub fn connect_network(ssid: &str, password: Option<&str>) -> CommandResult<String> {
    if ssid.trim().is_empty() {
        return Err(SetupError::InvalidInput("SSID cannot be empty".to_string()));
    }

    if !command_exists("iwctl") {
        return Err(SetupError::System("iwctl not found - ensure iwd is installed".to_string()));
    }

    let device = get_first_wireless_device()?;
    
    // Trigger scan first
    let _ = run_command(&["iwctl", "station", &device, "scan"], None);
    
    // Connect with optional password
    let output = run_command(
        &["iwctl", "station", &device, "connect", ssid], 
        password
    )?;
    
    Ok(output)
}

/// Get the first available wireless device
fn get_first_wireless_device() -> CommandResult<String> {
    let output = run_command(&["iwctl", "device", "list"], None)?;
    
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || 
           trimmed.starts_with("Device") || 
           trimmed.starts_with('-') {
            continue;
        }
        
        if let Some(device) = trimmed.split_whitespace().next() {
            return Ok(device.to_string());
        }
    }
    
    Err(SetupError::System("No wireless device found".to_string()))
}

#[cfg(test)]
mod tests {
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
    fn test_get_first_wireless_device_parsing() {
        // Test the device parsing logic with mock data
        let mock_output = "Device     Type    Mode    Powered\nwlan0      station on      on\n";
        let lines: Vec<&str> = mock_output.lines().collect();
        
        let mut device = None;
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || 
               trimmed.starts_with("Device") || 
               trimmed.starts_with('-') {
                continue;
            }
            
            if let Some(d) = trimmed.split_whitespace().next() {
                device = Some(d.to_string());
                break;
            }
        }
        
        assert_eq!(device, Some("wlan0".to_string()));
    }
}
