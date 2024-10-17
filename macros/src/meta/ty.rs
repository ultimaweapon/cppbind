/// Contains information for a C++ class.
#[derive(Default)]
pub struct TypeInfo {
    pub size: Option<usize>,
    pub align: Option<usize>,
}
