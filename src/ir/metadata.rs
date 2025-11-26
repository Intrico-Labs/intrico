//! Circuit IR metadata.
//!
//! This module defines metadata attached to circuit IR instances,
//! including version, name, depth, tags, and creation timestamp.

use chrono::{DateTime, Utc};

/// Metadata associated with circuit IR.
///
/// Contains circuit information like version, name, depth, and creation time.
#[derive(Debug)]
pub struct IRMetadata {
    pub version: String,
    pub name: String,
    pub depth: Option<usize>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>
}

impl Default for IRMetadata {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            name: "CircuitIR".to_string(),
            depth: None,
            tags: Vec::new(),
            created_at: Utc::now()
        }
    }
}