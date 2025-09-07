#[derive(Debug)]
/// VM flags
pub struct Flag {
    pub zero: bool,
    pub negative: bool,
    pub overflow: bool,
    pub carry: bool,
}

impl Flag {
    pub fn new() -> Self {
        Self { zero: false, negative: false, overflow: false, carry: false }
    }
}