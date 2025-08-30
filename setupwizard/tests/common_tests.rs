use setupwizard::common::{run_command, command_exists, SetupError};

#[cfg(test)]
mod common_tests {
    use super::*;

    #[test]
    fn test_run_command_success() {
        let result = run_command(&["echo", "test"], None);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test"));
    }

    #[test]
    fn test_run_command_with_input() {
        let result = run_command(&["cat"], Some("hello"));
        assert!(result.is_ok());
        assert!(result.unwrap().contains("hello"));
    }

    #[test]
    fn test_run_command_empty_args() {
        let result = run_command(&[], None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::InvalidInput(_));
    }

    #[test]
    fn test_run_command_nonexistent() {
        let result = run_command(&["nonexistent_command_12345"], None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::CommandFailed(_));
    }

    #[test]
    fn test_run_command_failure() {
        // false command should return exit code 1
        let result = run_command(&["false"], None);
        assert!(result.is_err());
        matches!(result.unwrap_err(), SetupError::CommandFailed(_));
    }

    #[test]
    fn test_command_exists() {
        // These commands should exist on most systems
        assert!(command_exists("echo"));
        assert!(command_exists("cat"));
        assert!(command_exists("ls"));
        
        // This command should not exist
        assert!(!command_exists("nonexistent_command_xyz_12345"));
    }

    #[test]
    fn test_command_exists_empty() {
        assert!(!command_exists(""));
    }

    #[test]
    fn test_error_types() {
        // Test that errors implement the right traits
        let error = SetupError::InvalidInput("test".to_string());
        let error_str = format!("{}", error);
        assert!(error_str.contains("Invalid input"));
        
        let error = SetupError::CommandFailed("test".to_string());
        let error_str = format!("{}", error);
        assert!(error_str.contains("Command failed"));
        
        let error = SetupError::System("test".to_string());
        let error_str = format!("{}", error);
        assert!(error_str.contains("System error"));
    }

    #[test]
    fn test_error_from_io() {
        // Test that IO errors can be converted to SetupError
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let setup_error: SetupError = io_error.into();
        matches!(setup_error, SetupError::Io(_));
    }

    #[test]
    fn test_run_command_multiline_output() {
        let result = run_command(&["echo", "-e", "line1\\nline2\\nline3"], None);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("line1"));
        assert!(output.contains("line2"));
        assert!(output.contains("line3"));
    }

    #[test]
    fn test_run_command_large_input() {
        let large_input = "x".repeat(1000);
        let result = run_command(&["cat"], Some(&large_input));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), large_input);
    }
}
