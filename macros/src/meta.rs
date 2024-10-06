use std::path::Path;
use thiserror::Error;

/// Contains C++ metadata loaded from a static library.
pub struct Metadata {}

impl Metadata {
    pub fn from_static_lib(path: impl AsRef<Path>) -> Result<Self, MetadataError> {
        Ok(Self {})
    }
}

/// Represents an error when [`Metadata`] fails to load.
#[derive(Debug, Error)]
pub enum MetadataError {}
