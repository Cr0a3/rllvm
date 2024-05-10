use rllvm::contxt::{jit::JitFunction, jit_link::{JitLinker, Link}};

fn main() {
    let mut linker = JitLinker::new();

    linker.funcs.insert("main".into(), vec![
        0xe8, 0x00, 0x00, 0x00, 0x00,   // call test
        0xc3,                           // ret
    ]);

    linker.funcs.insert("test".into(), vec![
        0xb8, 0x05, 0x00, 0x00, 0x00,   // eax = 5
        0xc3,                           // ret
    ]);

    linker.relocs.push( Link { from: "main".into(), to: "test".into(), at: 1, size: 4});

    let mut func: JitFunction<unsafe extern "C" fn() -> u32> = unsafe { linker.engine() };

    unsafe {
        let out = func.call();
        println!("{}", out);
    }
}