pub trait PraticeTrait {
    fn new(description: String) -> Self;
}

pub struct PraticeStruct {
    pub(crate) description: String,
}

impl PraticeTrait for PraticeStruct {
    fn new(description: String) -> Self {
        Self { description }
    }
}
