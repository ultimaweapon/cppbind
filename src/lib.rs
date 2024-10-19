pub use self::ffi::*;
pub use cppbind_macros::*;

mod ffi;

/// Memory of a C++ class that live on a heap.
pub struct Heap<T>(*mut T);

impl<T: HeapAlloc> Heap<T> {
    pub fn new() -> Self {
        Self(T::alloc().cast())
    }
}

impl<T: HeapAlloc> Memory for Heap<T> {
    type Class = T::Class;

    fn as_mut_ptr(&mut self) -> *mut () {
        self.0.cast()
    }
}

unsafe impl<T: Send> Send for Heap<T> {}
unsafe impl<T: Sync> Sync for Heap<T> {}

/// Memory of a C++ class.
pub trait Memory {
    type Class;

    fn as_mut_ptr(&mut self) -> *mut ();
}

/// Provides methods to allocate/deallocate a memory for a class.
pub trait HeapAlloc {
    type Class;

    fn alloc() -> *mut ();
}
