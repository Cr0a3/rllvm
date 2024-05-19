use std::error::Error;

use crate::{contxt::contxt::Context, ir::{compile::Compile, r#type::Type}, naming::NamingGenerator};

use super::AsmFunction;

/// Stores ir for function
pub struct Function {
    name: String,
    asm: AsmFunction,
    pub ir: Vec<Box<dyn Compile>>,

    args: Vec<Type>,
    ret: Type,

    pub export: bool,
}

impl Function {
    /// Creates a function
    pub fn new(name: &str, contxt: &Context, args: Vec<Type>, ret: Type) -> Self {
        Self {
            name: name.to_string(),
            asm: AsmFunction::new(name, &contxt),
            ir: vec![],
            args: args,
            ret: ret,
            export: false,
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function as a compilable version
    pub fn asm_func(&mut self) -> Result<&mut AsmFunction, Box<dyn Error>> {
        self.asm.args = self.args.clone();
        self.asm.ret = self.ret;        

        for ir in &self.ir {
            ir.compile(&mut self.asm)?;
        }

        Ok( &mut self.asm )
    }

    /// Makes the function public
    pub fn public(&mut self)  {
        self.export = true 
    }

    /// Renames the function to the corresponding naming conventioned name
    /// If it is exported
    pub fn maybe_renaming(&mut self) {
        if !self.export { return; }

        let name_gen = NamingGenerator::new();

        let fmt_ret: &str = {
            self.ret.name()
        };

        let fmt_args: Vec<&str>  = {
            let mut ret = vec![];

            for arg in &self.args {
                ret.push(arg.name())
            }

            ret
        };

        let new_name = name_gen.generate(
            vec![], 
            None, 
            &self.name, 
            fmt_args, 
            fmt_ret
        );

        self.name = new_name;
    }
}