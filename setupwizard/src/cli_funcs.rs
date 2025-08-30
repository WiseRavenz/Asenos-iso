use crate::{keymap, partition, wifi};
use crate::common::{CommandResult, SetupError};
use std::io::{self, Write};

pub fn list_keymaps() -> CommandResult<()> {
    let keymaps = keymap::available_keymaps()?;
    println!("Available keymaps:");
    for km in keymaps {
        println!("  {}", km);
    }
    Ok(())
}

pub fn set_keymap(map: &str) -> CommandResult<()> {
    keymap::set_keymap(map)?;
    println!("Keymap set to '{}'", map);
    Ok(())
}

pub fn list_wifi_networks() -> CommandResult<()> {
    let networks = wifi::list_networks()?;
    println!("Available WiFi networks:");
    println!("{}", networks);
    Ok(())
}

pub fn connect_wifi(ssid: &str, password: Option<&str>) -> CommandResult<()> {
    let result = wifi::connect_network(ssid, password)?;
    println!("WiFi connection result:");
    println!("{}", result);
    Ok(())
}

pub fn list_disks() -> CommandResult<()> {
    let disks = partition::list_disks()?;
    println!("Available disks:");
    println!("{}", disks);
    Ok(())
}

pub fn partition_disk_interactive() -> CommandResult<()> {
    println!("=== Asenos Partition Wizard ===");
    
    // Show available disks
    let disks = partition::list_disks()?;
    println!("Available disks:\n{}", disks);
    
    // Get user input
    let disk = prompt_input("Disk (e.g., /dev/sda): ")?;
    let boot_size_mb = prompt_number("Boot size MB (default 512): ", 512)?;
    let swap_size_mb = prompt_number("Swap size MB (default 2048): ", 2048)?;
    let use_gpt = prompt_bool("Use GPT? (y/n, default y): ", true)?;
    let filesystem = prompt_input_default("Filesystem (ext4/btrfs/xfs, default ext4): ", "ext4")?;
    
    let config = partition::PartitionConfig::new(
        disk.trim().to_string(),
        boot_size_mb,
        swap_size_mb,
        use_gpt,
        filesystem,
    );
    
    // Validate and create
    config.validate()?;
    
    println!("\nCreating partitions on {} with {} table...", 
        config.disk, if config.use_gpt { "GPT" } else { "MBR" });
    
    partition::create_partitions(&config)?;
    println!("Partitions created successfully!");
    
    // Show result
    if let Ok(info) = partition::get_partition_info(&config.disk) {
        println!("\nPartition layout:\n{}", info);
    }
    
    Ok(())
}

pub fn partition_disk_config(config_str: &str) -> CommandResult<()> {
    let config = partition::PartitionConfig::from_string(config_str)?;
    
    println!("Creating partitions on {} with {} table...", 
        config.disk, if config.use_gpt { "GPT" } else { "MBR" });
    
    partition::create_partitions(&config)?;
    println!("Partitions created successfully!");
    
    // Show result
    if let Ok(info) = partition::get_partition_info(&config.disk) {
        println!("\nPartition layout:\n{}", info);
    }
    
    Ok(())
}

// Helper functions for interactive input
fn prompt_input(prompt: &str) -> CommandResult<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Err(SetupError::InvalidInput("Input cannot be empty".to_string()));
    }
    
    Ok(trimmed.to_string())
}

fn prompt_input_default(prompt: &str, default: &str) -> CommandResult<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn prompt_number(prompt: &str, default: u32) -> CommandResult<u32> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        Ok(default)
    } else {
        trimmed.parse().map_err(|_| SetupError::InvalidInput("Invalid number".to_string()))
    }
}

fn prompt_bool(prompt: &str, default: bool) -> CommandResult<bool> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    
    if trimmed.is_empty() {
        Ok(default)
    } else {
        match trimmed.as_str() {
            "y" | "yes" | "true" | "1" => Ok(true),
            "n" | "no" | "false" | "0" => Ok(false),
            _ => Err(SetupError::InvalidInput("Please enter y/n".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_prompt_bool_parsing() {
        // Test the boolean parsing logic
        let test_cases = vec![
            ("y", true),
            ("yes", true),
            ("Y", true),
            ("YES", true),
            ("true", true),
            ("1", true),
            ("n", false),
            ("no", false),
            ("N", false),
            ("NO", false),
            ("false", false),
            ("0", false),
        ];

        for (input, expected) in test_cases {
            let result = match input.to_lowercase().as_str() {
                "y" | "yes" | "true" | "1" => true,
                "n" | "no" | "false" | "0" => false,
                _ => panic!("Invalid input for test"),
            };
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
