use self::class::Class;
use crate::META;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::Error;

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
    // Get metadata.
    let class = item.name;
    let name = class.to_string();
    let meta = match META.get_type(&name) {
        Some(v) => v,
        None => {
            return Err(Error::new_spanned(
                class,
                format_args!("cppbind::type_info<{name}> not found"),
            ))
        }
    };

    // Get object size.
    let size = match meta.size {
        Some(v) => v,
        None => {
            return Err(Error::new_spanned(
                class,
                format_args!("cppbind::type_info<{name}>::size not found"),
            ));
        }
    };

    // Get alignment.
    let align = match meta.align {
        Some(v) => v,
        None => {
            return Err(Error::new_spanned(
                class,
                format_args!("cppbind::type_info<{name}>::align not found"),
            ))
        }
    };

    // Render constructors.
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
    let align = Literal::usize_unsuffixed(align);
    let mem = if name.chars().next().unwrap().is_uppercase() {
        format_ident!("{name}Memory")
    } else {
        format_ident!("{name}_memory")
    };

    Ok(quote! {
        #[allow(non_camel_case_types)]
        pub struct #class<T> {
            mem: T,
            phantom: ::std::marker::PhantomData<::std::rc::Rc<()>>,
        }

        impl<T: ::cppbind::Memory<Class = Self>> #class<T> {
            #impls
        }

        #[allow(non_camel_case_types)]
        #[repr(C, align(#align))]
        pub struct #mem([::std::mem::MaybeUninit<u8>; #size]);

        impl #mem {
            pub const fn new() -> Self {
                Self([const { ::std::mem::MaybeUninit::uninit() }; #size])
            }
        }

        impl ::cppbind::Memory for &mut #mem {
            type Class = #class<Self>;
        }

        impl Default for #mem {
            fn default() -> Self {
                Self::new()
            }
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
