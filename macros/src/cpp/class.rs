use super::func::Param;
use super::kw;
use crate::ty::Type;
use proc_macro2::Span;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, Ident, Token};

/// C++ class declaration.
pub struct Class {
    pub name: Ident,
    pub ctors: Vec<Ctor>,
    pub members: Vec<Member>,
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
        let mut ctors = Vec::new();
        let mut members = Vec::new();

        while !body.is_empty() {
            let l = body.lookahead1();

            if l.peek(kw::public) {
                body.parse::<kw::public>().unwrap();
                body.parse::<Token![:]>()?;

                accessibility = Accessibility::Public;
            } else if l.peek(Token![const]) {
                body.parse::<Type>()?;
            } else if l.peek(Ident) {
                let r = body.parse::<Ident>()?;
                let l = body.lookahead1();

                if l.peek(Ident) {
                    todo!()
                } else if r == class {
                    let args;

                    parenthesized!(args in body);

                    ctors.push(Ctor {
                        access: accessibility,
                        params: Punctuated::parse_terminated(&args)?,
                        span: r.span(),
                    });

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
            ctors,
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

/// Constructor of a C++ class.
pub struct Ctor {
    pub access: Accessibility,
    pub params: Punctuated<Param, Token![,]>,
    pub span: Span,
}

/// Member of a C++ class (exclude constructor and destructor).
pub enum Member {}
