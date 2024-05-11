use std::fmt::Display;

use target_lexicon::{Architecture::{X86_32, X86_64}, CallingConvention::*, Triple, X86_32Architecture::*};
use crate::{func::Function, target::call_conv::TargetCallConv};
use super::{jit::JitFunction, link::JitLinker};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextError {
    UnsuportedArch(String),
    UnsuportedCall(String),
}

impl Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ContextError::UnsuportedArch(arch) => format!("given architecture {arch} isn't currently supported"),
            ContextError::UnsuportedCall(call) => format!("given calling convention {call} isn't currently supported"),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for ContextError {}

/// Stores all functions
pub struct Context {
    funcs: Vec<Function>,

    pub call: TargetCallConv,
}

impl Context {
    /// Creates new context
    pub fn new(target: Triple) -> Result<Self, ContextError> {
        let arch = target.architecture;
        let call = target.default_calling_convention().unwrap();

        if  !(arch == X86_64 ||         // Supported
            arch == X86_32(I386) ||     // archs
            arch == X86_32(I586) ||     //
            arch == X86_32(I686)) {
                return Err( ContextError::UnsuportedArch( format!("{}", arch)) );
        }

        if  !(
            call == WindowsFastcall ||
            call == SystemV
        ) {
                return Err( ContextError::UnsuportedCall( "call".into() ) );
        }

        Ok(Self { 
            funcs: vec![],
            call: TargetCallConv::new(call),
        })
    }

    /// Adds a function to the context
    pub fn add_function(&mut self, name: &str) -> &mut Function {
        let func = Function::new(name);
        self.funcs.push(func);

        self.funcs.last_mut().unwrap()
    }

    /// Requests jit function
    pub unsafe fn get_jit_function<T>(&mut self, name: &str) -> Result<JitFunction<T>, Box<dyn std::error::Error>> {
        let mut linker = JitLinker::new();

        for func in self.funcs.iter_mut() {
            let func = func.asm_func();
            let compiled = func.compile();

            let func_name = func.name();

            let entry = func_name == name;

            linker.add_func(&func_name, compiled.to_vec(), entry);

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