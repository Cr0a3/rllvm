use std::error::Error;

use iced_x86::code_asm::*;
use rllvm::contxt::{contxt::Context, jit::JitFunction};

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new();
    let func = contxt.add_function("main");

    let asm = func.asm_func();
    asm.asm.mov(eax, 5)?;
    asm.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
        let out = func.call();

        println!("main() -> {}", out);

        assert_eq!(out, 5);
    }

    Ok(())
}