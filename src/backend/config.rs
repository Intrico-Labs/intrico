//! Backend configurations representation
//! This module contains all the backend configurations

/// Backend config struct to hold all the backend configs
#[derive(Debug)]
pub struct BackendConfig {
    /// Optimization level (0-2)
    /// - 0: Validation and direct pass
    /// - 1, 2: In progress
    pub optimization_level: u8
}

impl Default for BackendConfig {
    /// Default Settings:
    /// - Optimization level - 0
    fn default() -> Self {
        Self {
            optimization_level: 0
        }
    }
}