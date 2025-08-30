use std::io::Write;
use std::process::{Command, Stdio};

/// Run a command and optionally write to its stdin. Returns combined stdout+stderr
/// on success, or an Err with the same text on failure.
pub fn run_command(args: &[&str], input: Option<&str>) -> Result<String, String> {
	if args.is_empty() {
		return Err("no command provided".into());
	}

	let mut cmd = Command::new(args[0]);
	if args.len() > 1 {
		cmd.args(&args[1..]);
	}

	// If we need to provide input, open stdin as piped.
	if input.is_some() {
		cmd.stdin(Stdio::piped());
	}
	cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

	let mut child = cmd.spawn().map_err(|e| format!("failed to spawn {}: {}", args[0], e))?;

	if let Some(s) = input {
		if let Some(mut stdin) = child.stdin.take() {
			// write the password and a newline
			let _ = stdin.write_all(s.as_bytes());
			let _ = stdin.write_all(b"\n");
		}
	}

	let output = child
		.wait_with_output()
		.map_err(|e| format!("failed to wait for {}: {}", args[0], e))?;

	let mut combined = String::new();
	combined.push_str(&String::from_utf8_lossy(&output.stdout));
	combined.push_str(&String::from_utf8_lossy(&output.stderr));

	if output.status.success() {
		Ok(combined)
	} else {
		Err(combined)
	}
}
