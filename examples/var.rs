use std::error::Error;
use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::Return, r#type::Type, var::VarGen}};
use target_lexicon::Triple;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let func = contxt.add_function("add", vec![Type::u32, Type::u32], Type::u32);
    let asm = func.asm_func()?;

    let x = VarGen::new_reg(Type::u32, asm.call.arg64_reg(0).unwrap());
    let y = VarGen::new_reg(Type::u32, asm.call.arg64_reg(1).unwrap());

    func.ir.push( Return::new(*(x + y) ) );


    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
        let out = func.call(5, 5);

        println!("main() -> {}", out);

        //assert_eq!(out, 10);
    }

    Ok(())
}