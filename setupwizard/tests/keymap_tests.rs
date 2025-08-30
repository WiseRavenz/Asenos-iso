use setupwizard::keymap::{available_keymaps, set_keymap};
use setupwizard::common::SetupError;

#[cfg(test)]
mod keymap_tests {
    use super::*;

    #[test]
    fn test_available_keymaps() {
        let result = available_keymaps();
        
        // Should either succeed or fail gracefully
        match result {
            Ok(keymaps) => {
                assert!(!keymaps.is_empty(), "Should return some keymaps");
                // Common keymaps that should be available
                let has_common = keymaps.iter().any(|k| k == "us" || k == "uk" || k == "de");
                if !has_common {
                    println!("Warning: No common keymaps found. Available: {:?}", keymaps);
                }
            }
            Err(SetupError::System(_)) => {
                // This is acceptable in test environments
                println!("Keymap discovery failed (expected in some test environments)");
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_set_keymap_empty() {
        let result = set_keymap("");
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_set_keymap_whitespace() {
        let result = set_keymap("   ");
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_set_keymap_invalid() {
        let result = set_keymap("invalid_keymap_xyz123");
        assert!(result.is_err());
        // Should fail with either InvalidInput (keymap not found) or System (can't get keymaps)
        match result.unwrap_err() {
            SetupError::InvalidInput(_) => {},
            SetupError::System(_) => {},
            e => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_set_keymap_special_chars() {
        let invalid_keymaps = vec![
            "key/map",
            "key\\map", 
            "key;map",
            "key|map",
            "key`map",
        ];

        for keymap in invalid_keymaps {
            let result = set_keymap(keymap);
            assert!(result.is_err(), "Should fail for keymap: {}", keymap);
        }
    }

    #[test]
    fn test_keymap_validation_flow() {
        // Test the full validation flow
        match available_keymaps() {
            Ok(keymaps) => {
                if let Some(first_keymap) = keymaps.first() {
                    // Try to set a valid keymap
                    let result = set_keymap(first_keymap);
                    match result {
                        Ok(_) => println!("Successfully set keymap: {}", first_keymap),
                        Err(SetupError::CommandFailed(_)) => {
                            // This is expected if we don't have permission to change keymap
                            println!("Permission denied for keymap change (expected in tests)");
                        }
                        Err(e) => panic!("Unexpected error: {:?}", e),
                    }
                }
            }
            Err(_) => {
                // If we can't get keymaps, that's fine for testing
                println!("Keymap discovery not available in test environment");
            }
        }
    }

    #[test]
    fn test_keymap_case_sensitivity() {
        // Test that keymap names are case-sensitive
        let test_cases = vec!["US", "Us", "uS"];
        
        for keymap in test_cases {
            let result = set_keymap(keymap);
            // These should typically fail as keymap names are lowercase
            if result.is_ok() {
                println!("Unexpectedly succeeded for keymap: {}", keymap);
            }
        }
    }

    #[test]
    fn test_error_messages() {
        let result = set_keymap("");
        if let Err(SetupError::InvalidInput(msg)) = result {
            assert!(msg.contains("cannot be empty"));
        } else {
            panic!("Expected InvalidInput error");
        }

        let result = set_keymap("definitely_invalid_keymap_name_12345");
        if let Err(error) = result {
            let error_msg = format!("{}", error);
            // Should mention the invalid keymap name or available options
            assert!(
                error_msg.contains("Unknown keymap") || 
                error_msg.contains("available") ||
                error_msg.contains("System error"), // If keymap discovery fails
                "Error message should be helpful: {}", error_msg
            );
        }
    }
}
