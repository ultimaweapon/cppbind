pub use self::ty::*;

use crate::symbol::{Segment, Symbol, TemplateArg};
use memmap2::Mmap;
use object::read::archive::ArchiveFile;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

mod ty;

/// Contains C++ metadata loaded from a static library.
pub struct Metadata {
    types: HashMap<String, TypeInfo>,
}

impl Metadata {
    pub fn from_static_lib(path: impl AsRef<Path>) -> Result<Self, MetadataError> {
        // Open file.
        let file = File::open(path).map_err(MetadataError::OpenFileFailed)?;
        let file = unsafe { Mmap::map(&file).map_err(MetadataError::MapFileFailed) }?;
        let file = ArchiveFile::parse(file.as_ref()).map_err(MetadataError::ParseFileFailed)?;

        // Get symbols.
        let symbols = file.symbols().map_err(MetadataError::GetSymbolsFailed)?;
        let mut types = HashMap::new();
        let symbols = match symbols {
            Some(v) => v,
            None => return Ok(Self { types }),
        };

        // Parse symbols.
        for sym in symbols {
            let sym = sym.map_err(MetadataError::ReadSymbolFailed)?;
            let sym = match Symbol::parse(sym.name()) {
                Ok(v) => v,
                Err(_) => continue, // Ignore unknown symbol.
            };

            // Check namespace.
            let mut iter = sym.name().iter();
            let ns = iter.next();

            if !ns.is_some_and(|s| *s == Segment::Ident("cppbind".into())) {
                continue;
            }

            // Check metadata type.
            let ty = iter.next();

            if !ty.is_some_and(|s| *s == Segment::Ident("type_info".into())) {
                continue;
            }

            // Get class name.
            let ty = iter.next().expect("invalid cppbind::type_info definition");
            let class = match ty {
                Segment::Ident(_) => panic!("invalid argument for cppbind::type_info"),
                Segment::TemplateArg(TemplateArg::Ident(v)) => v,
            };

            types.insert(class.clone().into_owned(), TypeInfo::new());
        }

        Ok(Self { types })
    }

    pub fn get_type(&self, name: impl AsRef<str>) -> Option<&TypeInfo> {
        self.types.get(name.as_ref())
    }
}

/// Represents an error when [`Metadata`] fails to load.
#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("couldn't open the specified file")]
    OpenFileFailed(#[source] std::io::Error),

    #[error("couldn't map the specified file")]
    MapFileFailed(#[source] std::io::Error),

    #[error("couldn't parse the specified file")]
    ParseFileFailed(#[source] object::read::Error),

    #[error("couldn't get available symbols")]
    GetSymbolsFailed(#[source] object::read::Error),

    #[error("couldn't read a symbol")]
    ReadSymbolFailed(#[source] object::read::Error),
}
