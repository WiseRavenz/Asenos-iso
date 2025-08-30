use crate::keymap;

pub fn list_keymaps() -> Result<(), String> {
    match keymap::available_keymaps() {
        Ok(list) => {
            println!("Available keymaps:");
            for km in list {
                println!(" - {}", km);
            }
        }
        Err(e) => {
            eprintln!("Failed to discover keymaps: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn keymap_set(map: &str) -> Result<(), String> {
    match keymap::set_keymap(map) {
        Ok(()) => {
            println!("Keymap set to '{}'.", map);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to set keymap: {}", e);
            std::process::exit(1);
        }
    }
}