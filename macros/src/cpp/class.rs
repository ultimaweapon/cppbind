use super::kw;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parenthesized, Ident, Token};

/// C++ class declaration.
pub struct Class {
    name: Ident,
    members: Vec<Member>,
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Skip declaration.
        input.parse::<kw::class>()?;

        // Parse name.
        let class = input.call(Ident::parse_any)?;
        let body;

        braced!(body in input);

        // Parse body.
        let mut accessibility = Accessibility::Private;
        let mut members = Vec::new();

        while !body.is_empty() {
            let l = body.lookahead1();

            if l.peek(kw::public) {
                body.parse::<kw::public>()?;
                body.parse::<Token![:]>()?;

                accessibility = Accessibility::Public;
            } else if l.peek(Ident) {
                let r = body.parse::<Ident>()?;
                let l = body.lookahead1();

                if l.peek(Ident) {
                    todo!()
                } else if r == class {
                    let args;

                    parenthesized!(args in body);

                    while !args.is_empty() {
                        todo!()
                    }

                    body.parse::<Token![;]>()?;
                } else {
                    return Err(l.error());
                }
            } else {
                return Err(l.error());
            }
        }

        // Require ; after } to people can copy & paste C++ class.
        input.parse::<Token![;]>()?;

        Ok(Self {
            name: class,
            members,
        })
    }
}

/// Accessibility of a member.
#[derive(Clone, Copy)]
pub enum Accessibility {
    Public,
    Private,
}

/// Member of a C++ class (exclude constructor and destructor).
pub enum Member {}
