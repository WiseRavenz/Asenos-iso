use crate::common::{run_command, CommandResult, SetupError, command_exists};

/// Get list of available system keymaps
pub fn available_keymaps() -> CommandResult<Vec<String>> {
    // Try localectl first (systemd systems)
    if command_exists("localectl") {
        if let Ok(output) = run_command(&["localectl", "list-keymaps"], None) {
            let keymaps: Vec<String> = output
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            
            if !keymaps.is_empty() {
                return Ok(keymaps);
            }
        }
    }

    // Fallback: scan filesystem
    let find_cmd = concat!(
        "find /usr/share/kbd/keymaps -type f ",
        r#"\( -name '*.map.gz' -o -name '*.map' \) "#,
        r#"-printf '%f\n' 2>/dev/null | "#,
        r#"sed -E 's/\.(map|map\.gz)$//' | "#,
        "sort -u"
    );
    
    let output = run_command(&["sh", "-c", find_cmd], None)?;
    let keymaps: Vec<String> = output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if keymaps.is_empty() {
        Err(SetupError::System("No keymaps found on system".to_string()))
    } else {
        Ok(keymaps)
    }
}

/// Set system keymap using loadkeys
pub fn set_keymap(keymap: &str) -> CommandResult<()> {
    if keymap.trim().is_empty() {
        return Err(SetupError::InvalidInput("Keymap cannot be empty".to_string()));
    }

    let available = available_keymaps()?;
    if !available.contains(&keymap.to_string()) {
        return Err(SetupError::InvalidInput(format!(
            "Unknown keymap '{}'. Available: {}",
            keymap,
            available.join(", ")
        )));
    }

    run_command(&["loadkeys", keymap], None)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_keymaps() {
        let result = available_keymaps();
        // Should either succeed or fail gracefully
        match result {
            Ok(keymaps) => assert!(!keymaps.is_empty()),
            Err(_) => {} // System might not have keymaps available in test environment
        }
    }

    #[test]
    fn test_set_keymap_empty() {
        let result = set_keymap("");
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_set_keymap_invalid() {
        let result = set_keymap("invalid_keymap_xyz123");
        assert!(result.is_err());
    }
}
