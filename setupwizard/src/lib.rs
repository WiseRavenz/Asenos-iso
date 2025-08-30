//! Asenos Setup Wizard Library
//! 
//! This library provides functionality for setting up Asenos Linux systems,
//! including partitioning, network configuration, and system setup.

pub mod common;
pub mod keymap;
pub mod partition;
pub mod wifi;
pub mod cli_funcs;

// Re-export commonly used types
pub use partition::PartitionConfig;
