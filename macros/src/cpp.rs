use self::class::Class;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

mod class;

/// Generates Rust code for `items`.
pub fn render(items: Declarations) -> syn::Result<TokenStream> {
    Ok(quote! {})
}

/// Contains C++ declarations parsed from [cpp](super::cpp) macro.
pub struct Declarations(Vec<Declaration>);

impl Parse for Declarations {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            let l = input.lookahead1();

            if l.peek(kw::class) {
                items.push(Declaration::Class(input.parse()?));
            } else {
                return Err(l.error());
            }
        }

        Ok(Self(items))
    }
}

/// Single C++ declaration.
enum Declaration {
    Class(Class),
}

mod kw {
    syn::custom_keyword!(class);
}
