//! RLLVM - a new code generation libary
//! 
//! ## Description
//! 
//! RLLVM is a new code generation libary with the main focus of being easy to use
//! and jit execution. It isn't the fastest but the most simple.
//! 
//! You can even only use ceratein features (like only the jit feature) to have a starting
//! foundation for your code generation libary!
//! 
//! ## Examples
//! Here are a few examples on how to use certain structs ...
//! 
//! to see more examples check out our github: <https://github.com/Toni-Graphics/rllvm>
//! 
//! ### Ir
//! ```rust
//! use std::error::Error;
//! use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::Return, r#type::Type}};
//! use target_lexicon::Triple;
//! 
//! fn main() -> Result<(), Box<dyn Error>>{
//!     let mut contxt = Context::new( Triple::host() )?;
//!     let func = contxt.add_function("add", vec![Type::u32, Type::u32], Type::u32);
//!     let asm = func.asm_func()?;
//! 
//!     let x = asm.arg(0).unwrap();
//!     let y = asm.arg(1).unwrap();
//! 
//!     func.ir.push( Return::new(*(x + y) ) );
//! 
//! 
//!     unsafe {
//!         let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
//!         let out = func.call(5, 5);
//! 
//!         println!("main() -> {}", out);
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### Linker
//! ```rust
//! use rllvm::contxt::{jit::JitFunction, link::{JitLinker, Link}};
//! 
//! fn main() {
//!     let mut linker = JitLinker::new();
//! 
//!     linker.add_func("main", vec![
//!         0xe8, 0x00, 0x00, 0x00, 0x00,   // call test
//!         0xc3,                           // ret
//!     ], true);
//! 
//!     linker.add_func("test", vec![
//!         0xb8, 0x00, 0x00, 0x00, 0x00,   // eax = 5
//!         0xc3,                           // ret
//!     ], false);
//! 
//!     linker.add_reloc( Link { from: "main".into(), to: "test".into(), at: 1, size: 4, replace: false} );
//!     linker.add_reloc( Link { from: "test".into(), to: "test_data".into(), at: 1, size: 4, replace: true} ); // replace means inline
//!     linker.add_label("test_data", vec![5]);
//! 
//!     unsafe {
//!         let mut func: JitFunction<unsafe extern "C" fn() -> u32> = linker.engine();
//!         let out = func.call();
//!         println!("{}", out);
//!     }
//! }
//! ```
//! 
//! ### Jit function
//! ```rust
//! use rllvm::contxt::jit::JitFunction;
//! 
//! fn main() {
//!     let mut func: JitFunction<unsafe extern "C" fn() -> u32> = JitFunction::new(
//!         vec![
//!                 0xb8, 0x05, 0x00, 0x00, 0x00,   // mov eax, 5
//!                 0xc3,                           // ret
//!             ],
//!     );
//!     unsafe {
//!         let out = func.call();
//!         println!("{}", out);
//!     }
//! }
//! ```

#[cfg(feature = "function")]
pub mod func;

#[cfg(feature = "ir")]
pub mod ir;

pub mod contxt;
pub mod target;

pub mod naming;

pub use target_lexicon;