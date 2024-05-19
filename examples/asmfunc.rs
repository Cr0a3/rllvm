use std::error::Error;
use iced_x86::code_asm::*;
use rllvm::prelude::*;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let call = contxt.call.clone();
    let func = contxt.add_function("main", vec![], Type::u32);

    let asm = func.asm_func()?;

    let arg1 = call.arg32(0).unwrap();
    let arg2 = call.arg32(1).unwrap();

    asm.asm.mov(arg1, 5)?;
    asm.asm.mov(arg2, 5)?;
    asm.asm.call(0)?;
    asm.reloc_at_current_pos("add", -4, 4)?;

    asm.asm.ret()?;

    
    let add = contxt.add_function("add", vec![Type::u32, Type::u32], Type::u32);

    let add = add.asm_func()?;

    add.asm.add(arg1, arg2)?;
    add.asm.mov(eax, arg1)?;
    add.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
        let out = func.call();

        println!("main() -> {}", out);

        assert_eq!(out, 10);
    }

    Ok(())
}