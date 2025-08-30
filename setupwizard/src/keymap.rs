use std::process::Command;

/// Discover available keymaps from the system.
///
/// Tries `localectl list-keymaps` first. If that fails, falls back to scanning
/// `/usr/share/kbd/keymaps` for map files. Returns a Vec of keymap names or an
/// Err describing the failure.
pub fn available_keymaps() -> Result<Vec<String>, String> {
    // Try localectl which prints a list of keymaps, one per line.
    if let Ok(output) = Command::new("localectl").arg("list-keymaps").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            let keys: Vec<String> = s
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            if !keys.is_empty() {
                return Ok(keys);
            }
        }
    }

    // Fallback: look under /usr/share/kbd/keymaps for files and strip extensions.
    // Use a small shell pipeline so we don't add dependencies for recursive walking.
    let find_cmd = r#"find /usr/share/kbd/keymaps -type f \( -name '*.map.gz' -o -name '*.map' \) -printf '%f\n' 2>/dev/null | sed -E 's/\.(map|map.gz)$//' | sort -u"#;
    let output = Command::new("sh")
        .arg("-c")
        .arg(find_cmd)
        .output()
        .map_err(|e| format!("failed to execute fallback keymap scan: {}", e))?;

    if !output.status.success() {
        return Err("failed to discover keymaps from system".into());
    }

    let s = String::from_utf8_lossy(&output.stdout);
    let keys: Vec<String> = s
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if keys.is_empty() {
        Err("no keymaps found on the system".into())
    } else {
        Ok(keys)
    }
}

/// Validate the provided keymap is in the list and attempt to apply it using `loadkeys`.
/// Returns Ok(()) on success or an Err(message) on failure.
pub fn set_keymap(map: &str) -> Result<(), String> {
    let allowed = available_keymaps().map_err(|e| format!("could not get available keymaps: {}", e))?;
    if !allowed.iter().any(|s| s == map) {
        return Err(format!(
            "unknown keymap '{}' - run with --list-keymaps to see supported values",
            map
        ));
    }

    // Try `loadkeys <map>` which is common in live/installer environments.
    let status = Command::new("loadkeys")
        .arg(map)
        .status()
        .map_err(|e| format!("failed to execute loadkeys: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        let code = status.code().map(|c| c.to_string()).unwrap_or_else(|| "unknown".into());
        Err(format!("loadkeys exited with code: {}", code))
    }
}
