use std::io::Write;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Command failed: {0}")]
    CommandFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("System error: {0}")]
    System(String),
}

pub type CommandResult<T> = Result<T, SetupError>;

/// Execute a system command with optional stdin input
/// Returns stdout/stderr combined on success, or SetupError on failure
pub fn run_command(args: &[&str], input: Option<&str>) -> CommandResult<String> {
    if args.is_empty() {
        return Err(SetupError::InvalidInput("No command provided".to_string()));
    }

    let mut cmd = Command::new(args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }

    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()
        .map_err(|e| SetupError::CommandFailed(format!("Failed to spawn {}: {}", args[0], e)))?;

    if let Some(s) = input {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(s.as_bytes())?;
            stdin.write_all(b"\n")?;
        }
    }

    let output = child.wait_with_output()
        .map_err(|e| SetupError::CommandFailed(format!("Failed to wait for {}: {}", args[0], e)))?;

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        Ok(combined)
    } else {
        Err(SetupError::CommandFailed(format!("Command {} failed: {}", args[0], combined)))
    }
}

/// Check if a command exists in the system
pub fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
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
    fn test_command_exists() {
        assert!(command_exists("echo"));
        assert!(!command_exists("nonexistent_command_12345"));
    }
}
