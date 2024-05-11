use std::error::Error;

use iced_x86::code_asm::*;
use rllvm::contxt::{contxt::Context, jit::JitFunction};
use target_lexicon::Triple;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let func = contxt.add_function("main");

    let asm = func.asm_func();

    asm.asm.mov(ecx, 5)?;
    asm.asm.mov(edx, 5)?;
    asm.asm.call(0)?;
    asm.reloc_at_current_pos("add", -4, 4)?;

    asm.asm.ret()?;

    
    let add = contxt.add_function("add");

    let add = add.asm_func();

    add.asm.add(ecx, edx)?;
    add.asm.mov(eax, ecx)?;
    add.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
        let out = func.call();

        println!("main() -> {}", out);

        assert_eq!(out, 10);
    }

    Ok(())
}