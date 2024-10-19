unsafe extern "C-unwind" {
    #[link_name = "\u{1}_Znwm"]
    pub fn new(len: usize) -> *mut ();
    #[link_name = "\u{1}_ZdlPvm"]
    pub fn delete(ptr: *mut (), len: usize);
}
