use std::error::Error;

use rllvm::contxt::{contxt::Context, jit::JitFunction};
use x86asm::{Instruction, Mnemonic, Operand, Reg};

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new();
    let func = contxt.add_function("main");

    let asm = func.asm_func();
    asm.x86_asm_ir.push( 
        Instruction::new2(
            Mnemonic::MOV, 
            Operand::Direct(Reg::EAX), 
            Operand::Literal32(5) 
        ) // mov eax, 5
    );
    asm.x86_asm_ir.push(
        Instruction::new0(Mnemonic::RET)
    );

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
        let out = func.call();
        
        println!("main() -> {}", out);

        assert_eq!(out, 5);
    }

    Ok(())
}