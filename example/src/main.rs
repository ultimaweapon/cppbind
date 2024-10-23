use cppbind::{cpp, Heap};

fn main() {
    // Construct class1 directly on Rust stack.
    let mut stack = class1_memory::new();
    let stack = unsafe { class1::new1(&mut stack) };

    // Construct class1 on C++ heap.
    let heap = Heap::<class1_memory>::new();
    let heap = unsafe { class1::new1(heap) };
}

cpp! {
    class class1 {
    public:
        class1();

        const char *value() const;
    };
}
