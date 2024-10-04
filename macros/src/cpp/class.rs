use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Token};

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

        // Parse body.
        let mut accessibility = Accessibility::Private;

        while !body.is_empty() {
            // Parse accessibility.
            if body.parse::<Option<super::kw::public>>()?.is_some() {
                body.parse::<Token![:]>()?;
                accessibility = Accessibility::Public;
            }

            break;
        }

        Ok(Self { ident })
    }
}

/// Accessibility of a member.
enum Accessibility {
    Public,
    Private,
}
