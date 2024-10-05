use syn::parse::{Parse, ParseStream};

/// Parameter of a C++ function/method.
pub struct Param {}

impl Parse for Param {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
