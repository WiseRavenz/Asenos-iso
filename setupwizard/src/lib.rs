//! Asenos Setup Wizard Library
//! 
//! A system setup wizard for Asenos Linux providing functionality for:
//! - Keymap configuration
//! - WiFi network management  
//! - Disk partitioning
//! - Basic system configuration

pub mod common;
pub mod keymap;
pub mod partition;
pub mod wifi;
pub mod cli_funcs;

// Re-export commonly used types
pub use partition::PartitionConfig;
pub use common::{CommandResult, SetupError};
