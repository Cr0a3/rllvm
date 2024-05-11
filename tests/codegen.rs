use std::error::Error;

use iced_x86::code_asm::*;
use rllvm::contxt::{contxt::Context, jit::JitFunction};

#[test]
fn asm_function_jit() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new(target_lexicon::Triple::host())?;
    
    let add = contxt.add_function("add");

    let add = add.asm_func();
    
    add.asm.add(contxt.call.arg32(1), contxt.call.arg32(1))?;

    #[cfg(target_os = "windows")]
    add.asm.mov(contxt.call.ret32(), contxt.call.arg32(1))?;
    add.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
        let out = func.call(69, 69);

        println!("main() -> {}", out);

        assert_eq!(out, 138);
    }

    Ok(())
}