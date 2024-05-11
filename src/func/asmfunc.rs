use iced_x86::code_asm::*;
use crate::contxt::link::Link;

/// Stores ir for function which can be compiled
pub struct AsmFunction {
    pub name: String,
    pub asm: CodeAssembler,
}

impl AsmFunction {
    /// Creates a function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            asm: CodeAssembler::new(64).unwrap(), // unwrap because i i made it just so it can't give error
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Compiles the function
    pub fn compile(&mut self) -> Vec<u8> {
        self.asm.assemble(0).unwrap()
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