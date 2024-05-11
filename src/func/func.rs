use super::AsmFunction;

/// Stores ir for function
pub struct Function {
    name: String,
    asm: AsmFunction,
}

impl Function {
    /// Creates a function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            asm: AsmFunction::new(name),
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function as a compilable version
    pub fn asm_func(&mut self) -> &mut AsmFunction {
        &mut self.asm
    }
}