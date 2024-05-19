use std::{error::Error, fmt::Display};

use target_lexicon::{Architecture::{X86_32, X86_64}, CallingConvention::*, Triple, X86_32Architecture::*};
use crate::{func::Function, ir::r#type::Type, target::call_conv::TargetCallConv};
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

/// Stores all functions/data/etc
/// 
/// Example usage (creates add function):
/// ```
/// use std::error::Error;
/// use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::Return, r#type::Type}};
/// use target_lexicon::Triple;
/// 
/// fn main() -> Result<(), Box<dyn Error>>{
///     let mut contxt = Context::new( Triple::host() )?;
///     let func = contxt.add_function("add", vec![Type::u32, Type::u32], Type::u32);
///     let asm = func.asm_func()?;
/// 
///     let x = asm.arg(0).unwrap();
///     let y = asm.arg(1).unwrap();
/// 
///     func.ir.push( Return::new(*(x + y) ) );
/// 
/// 
///     unsafe {
///         let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
///         let out = func.call(5, 5);
/// 
///         println!("main() -> {}", out);
///     }
/// 
///     Ok(())
/// }
/// ```
pub struct Context {
    funcs: Vec<Function>,

    pub call: TargetCallConv,
    triple: Triple,
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
            triple: target,
        })
    }

    /// Adds a function to the context
    pub fn add_function(&mut self, name: &str, args: Vec<Type>, ret: Type) -> &mut Function {
        let func = Function::new(name, &self, args, ret);
        self.funcs.push(func);

        self.funcs.last_mut().unwrap()
    }

    #[cfg(feature = "jit")]
    /// Compiles the context and requests the given jit function
    /// 
    /// Example:
    /// 
    /// ```
    /// use std::error::Error;
    /// use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::*, r#type::Type}};
    /// use target_lexicon::Triple;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>>{
    ///     let mut contxt = Context::new( Triple::host() )?;
    ///     let func = contxt.add_function("main", vec![], Type::u32);
    /// 
    ///     func.ir.push( Return::new(5) );
    /// 
    ///     unsafe {
    ///         let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
    ///         let out = func.call();
    /// 
    ///         println!("main() -> {}", out);
    /// 
    ///         assert_eq!(out, 5);
    ///     }
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub unsafe fn get_jit_function<T>(&mut self, name: &str) -> Result<JitFunction<T>, Box<dyn std::error::Error>> {
        let mut linker = JitLinker::new();

        for func in self.funcs.iter_mut() {
            let func = func.asm_func()?;
            let compiled = func.compile()?;

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

    #[cfg(feature = "obj")]
    /// Writes all functions/data/relocs/etc. into one object file
    pub fn write(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        use std::collections::HashMap;

        use object::Architecture;

        use super::obj::{BinFormat, Decl, ObjectBuilder, Scope};
        use crate::contxt::link::Link;

        let arch = match self.triple.architecture {
            X86_64 | X86_32(_) =>  Architecture::X86_64,
            _ => Architecture::Unknown,
        };

        let fmt  = match self.triple.binary_format {
            target_lexicon::BinaryFormat::Elf => BinFormat::Elf,
            target_lexicon::BinaryFormat::Coff => BinFormat::Coff,
            target_lexicon::BinaryFormat::Macho => BinFormat::Macho,
            _ => BinFormat::host(),
        };

        let mut obj = ObjectBuilder::new(path);

        let mut funcs: HashMap<String, (Vec<u8>, Vec<Link>, Vec<(&str, Vec<u8>)>)> = HashMap::new();

        let mut renames: HashMap<String, String> = HashMap::new();

        // Insert values
        for func in self.funcs.iter_mut() {
            if func.export {
                let old_name = func.name().to_string();
                func.maybe_renaming(); // rename
                renames.insert(old_name, func.name().to_string());
            }

            let asm = func.asm_func()?;

            let code = asm.compile()?;
            let data = asm.data();
            let relocs = asm.relocs();
            let name = asm.name.clone();

            funcs.insert(name.to_string(), (code, relocs, data));
        }

        for func in funcs {
            obj.define(&func.0, func.1.0);
            obj.add_decl(&func.0, Decl::Function(Scope::Private));

            for data in func.1.2 {
                let name = format!(".L{}", data.0);
                obj.define(&name, data.1);
                obj.add_decl(&name, Decl::RData(Scope::Private));
            }

            for link in func.1.1 {
                let mut link = link;

                if renames.contains_key(&link.from) {
                    link.from = renames.get(&link.from).unwrap().to_string();
                }
                
                if renames.contains_key(&link.to) {
                    link.from = renames.get(&link.to).unwrap().to_string();
                }

                let formatic_link = super::obj::Link {from: link.from, to: link.to, at: link.at};
                obj.link(formatic_link);
            }
        }

        obj.write(fmt, arch, object::Endianness::Little)?;

        Ok(())
    }
}