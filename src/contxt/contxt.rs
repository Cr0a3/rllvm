use crate::func::Function;

use super::jit::JitFunction;
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
    pub fn get_jit_function<T>(&self, name: &str) -> Result<JitFunction<T>, Box<dyn std::error::Error>> {
        Ok(JitFunction::new(vec![]))
    }
}