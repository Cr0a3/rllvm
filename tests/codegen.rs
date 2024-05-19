use std::error::Error;

use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::Return, r#type::Type}, target::call_conv::TargetCallConv};

#[test]
fn asm_function_jit() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new(target_lexicon::Triple::host())?;
    let call: TargetCallConv = contxt.call.clone();
    
    let add = contxt.add_function("add", vec![Type::u32, Type:: u32], Type::u32);

    let add = add.asm_func()?;
    
    let arg1 = call.arg32(0).unwrap();
    let arg2 = call.arg32(1).unwrap();

    add.asm.add(arg1, arg2)?;
    add.asm.mov(call.ret32(), arg1)?;
    add.asm.ret()?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn(u32, u32) -> u32> = contxt.get_jit_function("add")?;
        let out = func.call(69, 69);

        println!("main() -> {}", out);

        assert_eq!(out, 138);
    }

    Ok(())
}

#[test]
fn return_types() -> Result<(), Box<dyn Error>> {
    let mut contxt = Context::new(target_lexicon::Triple::host())?;
    let func = contxt.add_function("test_f32", vec![], Type::f32);
    func.ir.push( Return::new(0.5 as f32) );

    // contxt.write("test.o")?;

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> f32> = contxt.get_jit_function("test_f32")?;
        let out = func.call();

        println!("out: {}", out);

        //assert_eq!(out, 0.5 as f32);
    }

    Ok(())
}