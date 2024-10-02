# cppbind

This is a Rust crate to generate binding to C++ functions and methods with zero overhead. It work by generating a FFI function with a symbol matching with the C++ side. Thus, allows you to call those C++ functions and methods directly from Rust without any overhead the same as C++ call.

## Limitations

- Rust can only create a C++ object on the heap.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
