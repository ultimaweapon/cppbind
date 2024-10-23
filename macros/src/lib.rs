use self::meta::Metadata;
use proc_macro::TokenStream;
use std::sync::LazyLock;
use syn::{parse_macro_input, Error};

mod cpp;
mod meta;
mod symbol;
mod ty;

/// Generate binding to C++ functions and methods.
#[proc_macro]
pub fn cpp(body: TokenStream) -> TokenStream {
    let items = parse_macro_input!(body as self::cpp::Declarations);

    self::cpp::render(items)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

static META: LazyLock<Metadata> = LazyLock::new(|| {
    Metadata::from_static_lib(std::env::var_os("CPPBIND_METADATA").unwrap()).unwrap()
});
