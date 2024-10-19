use self::class::Class;
use crate::symbol::{Segment, Signature, Symbol, Type};
use crate::META;
use proc_macro2::{Literal, Span, TokenStream};
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

    // Render constructor wrappers.
    let mut impls = TokenStream::new();

    for (i, ctor) in item.ctors.iter().enumerate() {
        let name = format_ident!("new{}", i + 1, span = ctor.span);
        let ffi = format_ident!("{}_new{}", class, i + 1, span = Span::call_site());

        impls.extend(quote! {
            pub unsafe fn #name(mut this: T) -> Self {
                #ffi(this.as_mut_ptr());

                Self {
                    mem: this,
                    phantom: ::std::marker::PhantomData,
                }
            }
        });
    }

    // Render constructor FFI.
    let mut externs = TokenStream::new();

    for (i, ctor) in item.ctors.iter().enumerate() {
        // Build name.
        let mut name = vec![Segment::Ident(name.as_str().into())];

        name.push(Segment::Ctor);

        // Build parameters.
        let mut params = Vec::with_capacity(ctor.params.len());

        if ctor.params.is_empty() {
            params.push(Type::Void);
        } else {
            todo!("parameterized constructor");
        }

        // Render.
        let sym = Symbol::new(name, Some(Signature::new(params)));
        let sym = sym.to_itanium();
        let name = format_ident!("{}_new{}", class, i + 1, span = Span::call_site());

        externs.extend(quote! {
            unsafe extern "C-unwind" {
                #[link_name = #sym]
                fn #name(this: *mut ());
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

            fn as_mut_ptr(&mut self) -> *mut () {
                self.0.as_mut_ptr().cast()
            }
        }

        impl Default for #mem {
            fn default() -> Self {
                Self::new()
            }
        }

        #externs
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
