use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident};

/// C++ class declaration.
pub struct Class {
    ident: Ident,
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Skip declaration.
        input.parse::<super::kw::class>()?;

        // Parse name.
        let ident = input.call(Ident::parse_any)?;
        let body;

        braced!(body in input);

        Ok(Self { ident })
    }
}
