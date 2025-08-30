use crate::common;

/// Discover available keymaps from the system.
///
/// Tries `localectl list-keymaps` first. If that fails, falls back to scanning
/// `/usr/share/kbd/keymaps` for map files. Returns a Vec of keymap names or an
/// Err describing the failure.
pub fn available_keymaps() -> Result<Vec<String>, String> {
    // Try localectl which prints a list of keymaps, one per line.
    if let Ok(s) = common::run_command(&["localectl", "list-keymaps"], None) {
        let keys: Vec<String> = s
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        if !keys.is_empty() {
            return Ok(keys);
        }
    }

    // Fallback: look under /usr/share/kbd/keymaps for files and strip extensions.
    // Use a small shell pipeline so we don't add dependencies for recursive walking.
    let find_cmd = r#"find /usr/share/kbd/keymaps -type f \( -name '*.map.gz' -o -name '*.map' \) -printf '%f\n' 2>/dev/null | sed -E 's/\.(map|map.gz)$//' | sort -u"#;
    let output = common::run_command(&["sh", "-c", find_cmd], None)
        .map_err(|e| format!("failed to execute fallback keymap scan: {}", e))?;

    let keys: Vec<String> = output
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

    // Use common::run_command to run loadkeys. run_command reports failures as Err
    // with combined stdout/stderr which we forward here.
    match common::run_command(&["loadkeys", map], None) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("loadkeys failed: {}", e)),
    }
}
