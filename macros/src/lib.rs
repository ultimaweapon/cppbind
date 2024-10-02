use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

mod cpp;

/// Generate binding to C++ functions and methods.
#[proc_macro]
pub fn cpp(body: TokenStream) -> TokenStream {
    let items = parse_macro_input!(body as self::cpp::Declarations);

    self::cpp::render(items)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
