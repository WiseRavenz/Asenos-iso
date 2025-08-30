use clap::Parser;
mod keymap;
mod cli_funcs;

#[derive(Parser)]
#[command(name = "setupwizard")]
#[command(version = "1.0")]
#[command(about = "Asenos Setup Wizard", long_about = None)]
struct Cli {
    /// Set keymap from the supported list (e.g. "us", "uk", "de")
    #[arg(long)]
    keymap: Option<String>,

    /// Print the available keymaps and exit
    #[arg(long)]
    list_keymaps: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list_keymaps {
        cli_funcs::list_keymaps()?;
    }

    if let Some(map) = cli.keymap.as_deref() {
        cli_funcs::keymap_set(map)?;
    }

    Ok(())
}
