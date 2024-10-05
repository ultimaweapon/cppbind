use std::path::Path;

fn main() {
    let root = Path::new("src");
    let mut b = cc::Build::new();
    let files = ["main.cpp"];

    for f in files.into_iter().map(|f| root.join(f)) {
        b.file(&f);

        println!("cargo::rerun-if-changed={}", f.to_str().unwrap());
    }

    b.cpp(true).compile("example");
}
