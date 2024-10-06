use thiserror::Error;

/// C++ symbol.
pub struct Symbol {}

impl Symbol {
    pub fn parse(mangled: impl AsRef<[u8]>) -> Result<Self, SymbolError> {
        Ok(Self {})
    }
}

/// Represents an error when [`Symbol`] fails to parse from a mangled name.
#[derive(Debug, Error)]
pub enum SymbolError {}
