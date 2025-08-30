use clap::Parser;
use setupwizard::cli_funcs;
use std::process;

#[derive(Parser)]
#[command(name = "setupwizard")]
#[command(version = "0.1.0")]
#[command(about = "Asenos Setup Wizard - System configuration tool for Asenos Linux")]
struct Cli {
    /// List available keymaps
    #[arg(long)]
    list_keymaps: bool,

    /// Set system keymap (e.g., "us", "uk", "de")
    #[arg(long)]
    keymap: Option<String>,

    /// List available WiFi networks
    #[arg(long)]
    wifi_list: bool,

    /// Connect to WiFi network (format: "ssid" or "ssid:password")
    #[arg(long)]
    wifi_connect: Option<String>,

    /// List available storage devices
    #[arg(long)]
    list_disks: bool,

    /// Launch interactive partition wizard
    #[arg(long)]
    partition_disk: bool,

    /// Create partitions with configuration string
    /// Format: disk:boot_size:swap_size:gpt/msdos:filesystem
    /// Example: /dev/sda:512:2048:gpt:ext4
    #[arg(long)]
    partition_config: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run_cli(&cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_cli(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if cli.list_keymaps {
        cli_funcs::list_keymaps()?;
    }

    if let Some(keymap) = &cli.keymap {
        cli_funcs::set_keymap(keymap)?;
    }
    
    if cli.wifi_list {
        cli_funcs::list_wifi_networks()?;
    }

    if let Some(wifi_config) = &cli.wifi_connect {
        let (ssid, password) = parse_wifi_config(wifi_config);
        cli_funcs::connect_wifi(ssid, password)?;
    }

    if cli.list_disks {
        cli_funcs::list_disks()?;
    }

    if cli.partition_disk {
        cli_funcs::partition_disk_interactive()?;
    }

    if let Some(config_str) = &cli.partition_config {
        cli_funcs::partition_disk_config(config_str)?;
    }

    Ok(())
}

/// Parse WiFi connection string in format "ssid" or "ssid:password"
fn parse_wifi_config(config: &str) -> (&str, Option<&str>) {
    match config.splitn(2, ':').collect::<Vec<&str>>().as_slice() {
        [ssid] => (ssid, None),
        [ssid, password] => (ssid, Some(password)),
        _ => (config, None), // Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wifi_config_ssid_only() {
        let (ssid, password) = parse_wifi_config("MyWiFi");
        assert_eq!(ssid, "MyWiFi");
        assert_eq!(password, None);
    }

    #[test]
    fn test_parse_wifi_config_with_password() {
        let (ssid, password) = parse_wifi_config("MyWiFi:mypassword");
        assert_eq!(ssid, "MyWiFi");
        assert_eq!(password, Some("mypassword"));
    }

    #[test]
    fn test_parse_wifi_config_with_colon_in_password() {
        let (ssid, password) = parse_wifi_config("MyWiFi:pass:word");
        assert_eq!(ssid, "MyWiFi");
        assert_eq!(password, Some("pass:word"));
    }
}
