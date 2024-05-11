use crate::func::Function;

use super::{jit::JitFunction, link::JitLinker};
/// Stores all functions
pub struct Context {
    funcs: Vec<Function>
}

impl Context {
    /// Adds a function to the context
    pub fn add_function(&mut self, name: &str) -> &mut Function {
        let func = Function::new(name);
        self.funcs.push(func);

        self.funcs.last_mut().unwrap()
    }

    /// Requests jit function
    pub unsafe fn get_jit_function<T>(&self, name: &str) -> Result<JitFunction<T>, Box<dyn std::error::Error>> {
        let mut linker = JitLinker::new();

        for func in &self.funcs {
            let entry = func.name() == name;

            linker.add_func(func.name(), func.compile(), entry);

            for reloc in func.relocs() {
                linker.add_reloc(reloc);
            }

            for (name, data) in func.data() {
                linker.add_label(name, data);
            }
        } 

        let func = linker.engine();
        Ok(func)
    }
}