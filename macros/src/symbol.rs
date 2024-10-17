use std::borrow::Cow;
use thiserror::Error;

mod itanium;

/// C++ symbol.
#[derive(Debug)]
pub struct Symbol {
    name: Vec<Segment<'static>>,
}

impl Symbol {
    pub fn parse(mangled: impl AsRef<[u8]>) -> Result<Self, SymbolError> {
        let mangled = mangled.as_ref();

        if mangled.starts_with(b"_Z") {
            self::itanium::parse(&mangled[2..])
        } else if mangled.starts_with(b"__Z") {
            self::itanium::parse(&mangled[3..])
        } else {
            Err(SymbolError::UnknownSymbol)
        }
    }

    pub fn name(&self) -> &[Segment<'static>] {
        &self.name
    }
}

/// Segment of a C++ name.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    Ident(Cow<'a, str>),
    TemplateArg(TemplateArg<'a>),
}

/// Argument of a template instantiation.
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateArg<'a> {
    Ident(Cow<'a, str>),
}

/// Represents an error when [`Symbol`] fails to parse from a mangled name.
#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("unknown symbol")]
    UnknownSymbol,
}
