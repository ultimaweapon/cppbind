use std::borrow::Cow;
use thiserror::Error;

mod itanium;

/// C++ symbol.
#[derive(Debug)]
pub struct Symbol<'a> {
    name: Vec<Segment<'a>>,
    sig: Option<Signature>,
}

impl Symbol<'static> {
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
}

impl<'a> Symbol<'a> {
    pub fn new(name: Vec<Segment<'a>>, sig: Option<Signature>) -> Self {
        Self { name, sig }
    }

    pub fn name(&self) -> &[Segment<'a>] {
        &self.name
    }

    pub fn to_itanium(&self) -> String {
        // Build name.
        let mut name = String::from("\u{1}_ZN");

        for s in &self.name {
            use std::fmt::Write;

            match s {
                Segment::Ident(v) => write!(name, "{}{}", v.len(), v).unwrap(),
                Segment::TemplateArg(_) => todo!(),
                Segment::Ctor => name.push_str("C1"),
            }
        }

        name.push('E');

        // Build signature.
        if let Some(s) = &self.sig {
            for p in &s.params {
                match p {
                    Type::Void => name.push('v'),
                }
            }
        }

        name
    }
}

/// Segment of a C++ name.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    Ident(Cow<'a, str>),
    TemplateArg(TemplateArg<'a>),
    Ctor,
}

/// Argument of a template instantiation.
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateArg<'a> {
    Ident(Cow<'a, str>),
}

/// Signature of C++ function.
#[derive(Debug)]
pub struct Signature {
    params: Vec<Type>,
}

impl Signature {
    pub fn new(params: Vec<Type>) -> Self {
        Self { params }
    }
}

/// C++ type.
#[derive(Debug)]
pub enum Type {
    Void,
}

/// Represents an error when [`Symbol`] fails to parse from a mangled name.
#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("unknown symbol")]
    UnknownSymbol,
}
