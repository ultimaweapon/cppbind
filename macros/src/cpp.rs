use self::class::Class;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};

mod class;
mod func;

/// Generates Rust code for `items`.
pub fn render(items: Declarations) -> syn::Result<TokenStream> {
    let mut output = TokenStream::new();

    for item in items.0 {
        match item {
            Declaration::Class(i) => output.extend(render_class(i)?),
        }
    }

    Ok(output)
}

fn render_class(item: Class) -> syn::Result<TokenStream> {
    // Render constructors.
    let class = item.name;
    let mut impls = TokenStream::new();

    for (i, ctor) in item.ctors.into_iter().enumerate() {
        let name = format_ident!("new{}", i + 1, span = ctor.span);

        impls.extend(quote! {
            pub unsafe fn #name(this: T) -> Self {
                Self {
                    mem: this,
                    phantom: ::std::marker::PhantomData,
                }
            }
        });
    }

    // Compose.
    Ok(quote! {
        #[allow(non_camel_case_types)]
        pub struct #class<T> {
            mem: T,
            phantom: ::std::marker::PhantomData<::std::rc::Rc<()>>,
        }

        impl<T: ::cppbind::Memory<Class = Self>> #class<T> {
            #impls
        }
    })
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
    syn::custom_keyword!(public);
}
