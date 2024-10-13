use std::borrow::Cow;
use thiserror::Error;

mod itanium;

/// C++ symbol.
pub struct Symbol {
    name: Vec<Segment<'static>>,
}

impl Symbol {
    pub fn parse(mangled: impl AsRef<[u8]>) -> Result<Self, SymbolError> {
        let mangled = mangled.as_ref();

        if mangled.starts_with(b"_Z") {
            self::itanium::parse(&mangled[2..])
        } else {
            todo!()
        }
    }

    pub fn name(&self) -> &[Segment] {
        &self.name
    }
}

/// Segment of a C++ name.
#[derive(PartialEq, Eq)]
pub enum Segment<'a> {
    Ident(Cow<'a, str>),
    TemplateArg(TemplateArg<'a>),
}

/// Argument of a template instantiation.
#[derive(PartialEq, Eq)]
pub enum TemplateArg<'a> {
    Ident(Cow<'a, str>),
}

/// Represents an error when [`Symbol`] fails to parse from a mangled name.
#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("unknown symbol")]
    UnknownSymbol,
}
