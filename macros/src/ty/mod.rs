use syn::parse::{Parse, ParseStream};

/// C++ type.
#[derive(Debug)]
pub enum Type {
    Void,
    Ulong,
    Ptr { c: bool, t: Box<Self> },
}

impl Parse for Type {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
