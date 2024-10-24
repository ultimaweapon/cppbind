use crate::ty::Type;
use std::borrow::Cow;
use thiserror::Error;

mod itanium;

/// C++ symbol.
#[derive(Debug)]
pub struct Symbol<'a> {
    name: Name<'a>,
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
    pub fn new(name: Name<'a>, sig: Option<Signature>) -> Self {
        Self { name, sig }
    }

    pub fn name(&self) -> &Name<'a> {
        &self.name
    }

    pub fn to_itanium(&self) -> String {
        // Build name.
        let mut name = String::from("\u{1}_Z");
        let push_segment = |n: &mut String, s: &Segment| {
            use std::fmt::Write;

            match s {
                Segment::Ident(v) => write!(n, "{}{}", v.len(), v).unwrap(),
                Segment::TemplateArg(_) => todo!(),
                Segment::Ctor => n.push_str("C1"),
                Segment::Dtor => n.push_str("D1"),
                Segment::New => n.push_str("nw"),
                Segment::Delete => n.push_str("dl"),
            }
        };

        match &self.name {
            Name::Nested(v) => {
                name.push('N');

                for s in v {
                    push_segment(&mut name, s);
                }

                name.push('E');
            }
            Name::Unscoped(s) => push_segment(&mut name, s),
        }

        // Build signature.
        fn push_type(n: &mut String, t: &Type) {
            match t {
                Type::Void => n.push('v'),
                Type::Ulong => n.push('m'),
                Type::Ptr { c: _, t } => push_type(n, &t),
            }
        }

        if let Some(s) = &self.sig {
            for p in &s.params {
                push_type(&mut name, p);
            }
        }

        name
    }
}

/// Name of a C++ symbol.
#[derive(Debug)]
pub enum Name<'a> {
    Nested(Vec<Segment<'a>>),
    Unscoped(Segment<'a>),
}

/// Segment of a C++ name.
#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    Ident(Cow<'a, str>),
    TemplateArg(TemplateArg<'a>),
    Ctor,
    Dtor,
    New,
    Delete,
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

/// Represents an error when [`Symbol`] fails to parse from a mangled name.
#[derive(Debug, Error)]
pub enum SymbolError {
    #[error("unknown symbol")]
    UnknownSymbol,
}
