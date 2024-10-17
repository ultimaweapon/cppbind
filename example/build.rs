use std::path::{Path, PathBuf};

fn main() {
    // Build C++ sources.
    let root = Path::new("src");
    let mut b = cc::Build::new();
    let files = ["main.cpp"];

    for f in files.into_iter().map(|f| root.join(f)) {
        b.file(&f);

        println!("cargo::rerun-if-changed={}", f.to_str().unwrap());
    }

    b.cpp(true).std("c++14").compile("example");

    // Set variables required by cppbind. The CPPBIND_METADATA variable need to be a path to a
    // static library that defines class metadata with CPPBIND_CLASS you want to use on Rust side.
    // This library can also contains other C++ code. In this example this library was built on the
    // above using cc crate.
    let mut lib = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    if cfg!(windows) {
        lib.push("example.lib");
    } else {
        lib.push("libexample.a");
    }

    println!(
        "cargo::rustc-env=CPPBIND_METADATA={}",
        lib.to_str().unwrap()
    );
}
