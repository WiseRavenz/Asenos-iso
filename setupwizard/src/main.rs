use clap::Parser;

mod cli_funcs;
mod common;
mod keymap;
mod partition;
mod wifi;

#[derive(Parser)]
#[command(name = "setupwizard")]
#[command(version = "1.0")]
#[command(about = "Asenos Setup Wizard", long_about = None)]
struct Cli {
    /// Print the available keymaps and exit
    #[arg(long)]
    list_keymaps: bool,

    /// Set keymap from the supported list (e.g. "us", "uk", "de")
    #[arg(long)]
    keymap: Option<String>,


    /// List available Wi-Fi networks
    #[arg(long)]
    wifi_list: bool,

    /// Connect to a Wi-Fi network
    #[arg(long)]
    wifi_connect: Option<String>,

    /// List available disks
    #[arg(long)]
    list_disks: bool,

    /// Create partitions on a disk (interactive mode)
    #[arg(long)]
    partition_disk: bool,

    /// Create partitions with specified configuration
    #[arg(long)]
    partition_config: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list_keymaps {
        cli_funcs::list_keymaps()?;
    }

    if let Some(map) = cli.keymap.as_deref() {
        cli_funcs::keymap_set(map)?;
    }
    
    if cli.wifi_list {
        cli_funcs::wifi_list()?;
    }

    if let Some(s) = cli.wifi_connect.as_deref() {
    // Expect input in "ssid:password" format; password is optional
    let mut parts = s.splitn(2, ':');
    let ssid = parts.next().unwrap_or("");
    // leave password as Option<&str> (None if omitted)
    let passwd = parts.next();
    cli_funcs::wifi_connect(ssid, passwd)?;
    }

    if cli.list_disks {
        cli_funcs::list_disks()?;
    }

    if cli.partition_disk {
        cli_funcs::partition_disk_interactive()?;
    }

    if let Some(config_str) = cli.partition_config.as_deref() {
        cli_funcs::partition_disk_config(config_str)?;
    }

    Ok(())
}
