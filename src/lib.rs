pub use cppbind_macros::*;

/// Memory of a C++ class that live on a heap.
pub struct Heap<T>(*mut T);

impl<T: Memory> Memory for Heap<T> {
    type Class = T::Class;
}

unsafe impl<T: Send> Send for Heap<T> {}
unsafe impl<T: Sync> Sync for Heap<T> {}

/// Memory of a C++ class.
pub trait Memory {
    type Class;
}
