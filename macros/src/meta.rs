pub use self::ty::*;

use crate::symbol::Symbol;
use memmap2::Mmap;
use object::read::archive::ArchiveFile;
use object::read::elf::ElfFile64;
use object::read::macho::MachOFile64;
use object::{Endianness, LittleEndian, Object};
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
        let ar = ArchiveFile::parse(file.as_ref()).map_err(MetadataError::ParseFileFailed)?;

        // Parse members.
        let mut types = HashMap::new();

        for (i, mem) in ar.members().enumerate() {
            // Get member data.
            let mem = mem.map_err(|e| MetadataError::ParseMemberHeaderFailed(i, e))?;
            let mem = mem
                .data(file.as_ref())
                .map_err(|e| MetadataError::GetMemberDataFailed(i, e))?;

            // Parse member.
            if mem.starts_with(b"\x7FELF") {
                ElfFile64::<Endianness>::parse(mem)
                    .map_err(|e| MetadataError::ParseMemberFailed(i, e))
                    .and_then(|m| Self::parse_obj(m, &mut types))?;
            } else if mem.starts_with(&0xFEEDFACFu32.to_le_bytes()) {
                MachOFile64::<LittleEndian>::parse(mem)
                    .map_err(|e| MetadataError::ParseMemberFailed(i, e))
                    .and_then(|m| Self::parse_obj(m, &mut types))?;
            } else {
                return Err(MetadataError::UnknownMember(
                    mem.iter().take(4).map(|v| *v).collect(),
                ));
            }
        }

        Ok(Self { types })
    }

    pub fn get_type(&self, name: impl AsRef<str>) -> Option<&TypeInfo> {
        self.types.get(name.as_ref())
    }

    fn parse_obj<'a>(
        _: impl Object<'a>,
        _: &mut HashMap<String, TypeInfo>,
    ) -> Result<(), MetadataError> {
        Ok(())
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

    #[error("couldn't parse header for member #{0}")]
    ParseMemberHeaderFailed(usize, #[source] object::read::Error),

    #[error("couldn't get data for member #{0}")]
    GetMemberDataFailed(usize, #[source] object::read::Error),

    #[error("unknown member ({0:x?})")]
    UnknownMember(Vec<u8>),

    #[error("couldn't parse member #{0}")]
    ParseMemberFailed(usize, #[source] object::read::Error),

    #[error("unknown symbol in cppbind namespace")]
    UnknownDefinition(Symbol),
}
