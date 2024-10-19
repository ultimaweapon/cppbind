# cppbind

This is a Rust crate to generate binding to C++ functions and methods with **zero overhead**. It works by generate FFI functions with symbol matching with the C++ side. Thus, allows you to call those C++ functions and methods directly from Rust without any overhead the same as C++ call themself. It also **allow you to construct a C++ object directly on Rust stack**.

The goal of this crate is to allows efficient integration between Rust and C++, which mean most of the generated code will be unsafe. You need to know how the C++ code you are going to use is working otherwise you will be hit by undefined behaviors.

## Requirements

- C++11 and its toolchain.

## Limitations

- Rust cannot access an instance variable directly. You need to create a getter/setter for each variable you want to access.
- Inline method is not supported. That mean you cannot define a method you want to use inside a class declaration.

## Usage

Copy `cppbind.hpp` on the root of this repository to your crate and create C++ files for the code you want to use on the Rust side. You still need some C++ files here even if the code you want to use living somewhere else. You need the following line for each C++ class you want to use on Rust side:

```cpp
#include "cppbind.hpp"

CPPBIND_CLASS(class1);
```

`class1` must be a complete type before `CPPBIND_CLASS` line. The next step is setup `build.rs` to build the C++ files you just created. The following example use [cc](https://crates.io/crates/cc) to build those C++ files:

```rust
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
```

See [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for more details about `build.rs`. If you are using other build system make sure you set `CPPBIND_METADATA` environment variable when invoke `rustc`. See the above example for more details.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
