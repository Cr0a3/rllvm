pub struct Function {
    name: String,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}