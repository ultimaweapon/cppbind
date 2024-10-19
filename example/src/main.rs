use cppbind::cpp;

fn main() {
    // Construct class1 directly on Rust stack.
    let mut v1 = class1_memory::new();
    let v1 = unsafe { class1::new1(&mut v1) };
}

cpp! {
    class class1 {
    public:
        class1();
    };
}
