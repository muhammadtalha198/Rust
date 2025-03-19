
#[derive(Debug)]
pub struct Asparagus {
    pub name: String,
    pub quantity: i32,
}

impl Asparagus {
    pub fn new(name: String, quantity: i32) -> Self {
        Self { name, quantity }
    }
}
