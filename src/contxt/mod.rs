use crate::func::Function;

pub mod jit;
pub mod jit_link;
pub mod exec_engine;

pub struct Context {
    funcs: Vec<Function>
}

impl Context {
    pub fn add_function(&mut self, name: &str) -> &mut Function {
        let func = Function::new(name);
        self.funcs.push(func);

        self.funcs.last_mut().unwrap()
    }
}