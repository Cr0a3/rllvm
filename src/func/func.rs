use std::error::Error;

use crate::{contxt::contxt::Context, ir::compile::Compile};

use super::AsmFunction;

/// Stores ir for function
pub struct Function {
    name: String,
    asm: AsmFunction,
    pub ir: Vec<Box<dyn Compile>>,
}

impl Function {
    /// Creates a function
    pub fn new(name: &str, contxt: &Context) -> Self {
        Self {
            name: name.to_string(),
            asm: AsmFunction::new(name, &contxt),
            ir: vec![],
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function as a compilable version
    pub fn asm_func(&mut self) -> Result<&mut AsmFunction, Box<dyn Error>> {
        for ir in &self.ir {
            ir.compile(&mut self.asm)?;
        }

        Ok( &mut self.asm )
    }
}