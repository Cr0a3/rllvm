use crate::contxt::link::Link;

/// Stores ir for function
pub struct Function {
    name: String,
}

impl Function {
    /// Creates a function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Compiles the function
    pub fn compile(&self) -> Vec<u8> {
        vec![]
    }

    /// Returns the relocs of the function
    pub fn relocs(&self) -> Vec<Link> {
        vec![]
    }

    /// Returns the data this function works with
    pub fn data(&self) -> Vec<(&str, Vec<u8>)> {
        vec![]
    }
}