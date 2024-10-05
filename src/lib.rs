pub use cppbind_macros::*;

/// RAII struct to free a C++ object on the heap.
pub struct Ptr<T>(*mut T);

impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe { std::ptr::drop_in_place(self.0) };
    }
}
