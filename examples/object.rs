use std::error::Error;
use rllvm::contxt::contxt::Context;
use target_lexicon::Triple;

fn main() -> Result<(), Box<dyn Error>>{
    let mut contxt = Context::new( Triple::host() )?;
    let call = contxt.call.clone();
    let arg1 = call.arg32(0).unwrap();
    let arg2 = call.arg32(1).unwrap();

    let add = contxt.add_function("add");

    let add = add.asm_func()?;

    add.asm.add(arg1, arg2)?;
    add.asm.mov(call.ret32(), arg1)?;
    add.asm.ret()?;

    contxt.write("test.o")?;

    Ok(())
}