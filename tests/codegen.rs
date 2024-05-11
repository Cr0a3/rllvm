use std::error::Error;

use iced_x86::code_asm::*;
use rllvm::contxt::{contxt::Context, jit::JitFunction};

#[test]
fn asm_function_jit() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new();
    
    let add = contxt.add_function("add");

    let add = add.asm_func();

    #[cfg(target_os = "linux")]
    add.asm.add(esi, edi)?;
    
    #[cfg(target_os = "windows")]
    add.asm.add(ecx, edx)?;

    #[cfg(target_os = "linux")]
    add.asm.mov(eax, esi)?;
    #[cfg(target_os = "windows")]
    add.asm.mov(eax, ecx)?;
    add.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
        let out = func.call(69, 69);

        println!("main() -> {}", out);

        assert_eq!(out, 138);
    }

    Ok(())
}