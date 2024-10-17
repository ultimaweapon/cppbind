pub use self::ty::*;

use crate::symbol::{Segment, Symbol, TemplateArg};
use memmap2::Mmap;
use object::read::archive::ArchiveFile;
use object::read::elf::ElfFile64;
use object::read::macho::MachOFile64;
use object::{Endianness, LittleEndian, Object, ObjectSection, ObjectSymbol, SymbolIndex};
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
        let mut meta = Self {
            types: HashMap::new(),
        };

        for (i, mem) in ar.members().enumerate() {
            // Get member data.
            let mem = mem.map_err(|e| MetadataError::ParseMemberHeaderFailed(i, e))?;
            let name = String::from_utf8_lossy(mem.name());
            let data = match mem.data(file.as_ref()) {
                Ok(v) => v,
                Err(e) => return Err(MetadataError::GetMemberDataFailed(name.into_owned(), e)),
            };

            // Parse member.
            let r = if data.starts_with(b"\x7FELF") {
                let obj = match ElfFile64::<Endianness>::parse(data) {
                    Ok(v) => v,
                    Err(e) => return Err(MetadataError::ParseMemberFailed(name.into_owned(), e)),
                };

                meta.parse_obj(obj)
            } else if data.starts_with(&0xFEEDFACFu32.to_le_bytes()) {
                let obj = match MachOFile64::<LittleEndian>::parse(data) {
                    Ok(v) => v,
                    Err(e) => return Err(MetadataError::ParseMemberFailed(name.into_owned(), e)),
                };

                meta.parse_obj(obj)
            } else {
                return Err(MetadataError::UnknownMember(
                    data.iter().take(4).map(|v| *v).collect(),
                ));
            };

            if let Err(e) = r {
                return Err(MetadataError::ParseObjectFailed(name.into_owned(), e));
            }
        }

        Ok(meta)
    }

    pub fn get_type(&self, name: impl AsRef<str>) -> Option<&TypeInfo> {
        self.types.get(name.as_ref())
    }

    fn parse_obj<'a>(&mut self, obj: impl Object<'a>) -> Result<(), ObjectError> {
        // Parse symbols.
        for sym in obj.symbols() {
            let index = sym.index();

            self.parse_sym(&obj, sym)
                .map_err(|e| ObjectError::ParseSymbolFailed(index, e))?;
        }

        Ok(())
    }

    fn parse_sym<'a>(
        &mut self,
        obj: &impl Object<'a>,
        sym: impl ObjectSymbol<'a>,
    ) -> Result<(), SymbolError> {
        use std::collections::hash_map::Entry;

        // Get symbol name.
        let index = sym.index();
        let raw = match sym.name_bytes() {
            Ok(v) => v,
            Err(e) => return Err(SymbolError::GetNameFailed(index, e)),
        };

        // Get section index.
        let section = match sym.section_index() {
            Some(v) => v,
            None => return Ok(()),
        };

        // Parse name.
        let off: usize = sym.address().try_into().unwrap();
        let len: usize = sym.size().try_into().unwrap();
        let sym = match Symbol::parse(raw) {
            Ok(v) => v,
            Err(_) => return Ok(()), // Ignore unknown symbol.
        };

        // Check namespace.
        let mut iter = sym.name().iter();

        if !iter
            .next()
            .is_some_and(|v| *v == Segment::Ident("cppbind".into()))
        {
            return Ok(());
        }

        // Check if type_info.
        if !iter
            .next()
            .is_some_and(|v| *v == Segment::Ident("type_info".into()))
        {
            return Err(SymbolError::UnknownCppbindSymbol);
        }

        // Get class name.
        let class = iter.next().ok_or(SymbolError::UnknownCppbindSymbol)?;
        let class = match class {
            Segment::TemplateArg(TemplateArg::Ident(v)) => v,
            _ => return Err(SymbolError::UnknownCppbindSymbol),
        };

        // Get TypeInfo.
        let info = match self.types.entry(class.as_ref().to_owned()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(TypeInfo::default()),
        };

        // Check info type.
        let ty = iter.next().ok_or(SymbolError::UnknownCppbindSymbol)?;
        let section = obj
            .section_by_index(section)
            .map_err(|e| SymbolError::GetSectionFailed(index, e))?;
        let section = section
            .data()
            .map_err(|e| SymbolError::GetSectionDataFailed(index, e))?;

        if *ty == Segment::Ident("size".into()) {
            info.size = section
                .get(off..(off + len))
                .map(|v| usize::from_ne_bytes(v.try_into().unwrap()))
                .ok_or_else(|| SymbolError::GetDataFailed(index))
                .map(Some)?;
        } else if *ty == Segment::Ident("align".into()) {
            info.align = section
                .get(off..(off + len))
                .map(|v| usize::from_ne_bytes(v.try_into().unwrap()))
                .ok_or_else(|| SymbolError::GetDataFailed(index))
                .map(Some)?;
        } else {
            return Err(SymbolError::UnknownCppbindSymbol);
        }

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

    #[error("couldn't get data of {0}")]
    GetMemberDataFailed(String, #[source] object::read::Error),

    #[error("unknown member ({0:x?})")]
    UnknownMember(Vec<u8>),

    #[error("couldn't parse {0}")]
    ParseMemberFailed(String, #[source] object::read::Error),

    #[error("couldn't parse {0}")]
    ParseObjectFailed(String, #[source] ObjectError),
}

/// Represents an error when [`Metadata`] fails to parse an object file.
#[derive(Debug, Error)]
pub enum ObjectError {
    #[error("couldn't parse a symbol #{0}")]
    ParseSymbolFailed(SymbolIndex, #[source] SymbolError),
}

/// Represents an error when [`Metadata`] fails to parse a symbol.
#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("couldn't get name of symbol #{0}")]
    GetNameFailed(SymbolIndex, #[source] object::read::Error),

    #[error("unknown symbol on cppbind namespace")]
    UnknownCppbindSymbol,

    #[error("couldn't get section for symbol #{0}")]
    GetSectionFailed(SymbolIndex, #[source] object::read::Error),

    #[error("couldn't get section data for symbol #{0}")]
    GetSectionDataFailed(SymbolIndex, #[source] object::read::Error),

    #[error("couldn't get data of symbol #{0}")]
    GetDataFailed(SymbolIndex),
}
