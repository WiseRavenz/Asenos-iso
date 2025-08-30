use clap::Parser;

mod cli_funcs;
mod common;
mod keymap;
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

    Ok(())
}
