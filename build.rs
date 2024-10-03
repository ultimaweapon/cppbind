use std::path::Path;

fn main() {
    if std::env::var_os("CARGO_TARGET_TMPDIR").is_some() {
        let root = Path::new("tests");
        let mut b = cc::Build::new();
        let files = ["binding.cpp"];

        for f in files.into_iter().map(|f| root.join(f)) {
            b.file(&f);

            println!("cargo::rerun-if-changed={}", f.to_str().unwrap());
        }

        b.cpp(true).compile("tests");
    }
}
