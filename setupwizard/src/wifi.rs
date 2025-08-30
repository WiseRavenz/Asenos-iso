use crate::common;

/// List available SSIDs using iwd (iwctl).
/// This will run a scan on the first wireless device found and print
/// the output of `iwctl station <dev> get-networks` to stdout.
pub fn list_ssids() -> Result<(), String> {
	let dev = get_first_device()?;

	// trigger a scan (best-effort)
	let _ = common::run_command(&["iwctl", "station", &dev, "scan"], None);

	let out = common::run_command(&["iwctl", "station", &dev, "get-networks"], None)?;
	println!("{}", out);
	Ok(())
}

/// Connect to an SSID using iwd (iwctl). If `passwd` is Some, it will be
/// provided on stdin to satisfy the passphrase prompt.
pub fn connect(ssid: &str, passwd: Option<&str>) -> Result<(), String> {
	let dev = get_first_device()?;

	// trigger a scan first
	let _ = common::run_command(&["iwctl", "station", &dev, "scan"], None);

	let args = &["iwctl", "station", &dev, "connect", ssid];
	let output = common::run_command(args, passwd)?;

	// Print command output for the user. iwctl normally prints connection info.
	println!("{}", output);

	Ok(())
}

fn get_first_device() -> Result<String, String> {
	let out = common::run_command(&["iwctl", "device", "list"], None)?;
	for line in out.lines() {
		let trimmed = line.trim();
		if trimmed.is_empty() {
			continue;
		}
		// skip header/separators
		if trimmed.starts_with("Device") || trimmed.starts_with('-') {
			continue;
		}

		// first whitespace-separated token is usually the device name (e.g. wlan0)
		if let Some(first) = trimmed.split_whitespace().next() {
			return Ok(first.to_string());
		}
	}
	Err("No wireless device found (iwctl device list returned no device)".into())
}
