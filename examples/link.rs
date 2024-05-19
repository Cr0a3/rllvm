use rllvm::prelude::*;

#[rustfmt::skip]
fn main() {
    let mut linker = JitLinker::new();

    linker.add_func("main", vec![
        0xe8, 0x00, 0x00, 0x00, 0x00,   // call test
        0xc3,                           // ret
    ], true);

    linker.add_func("test", vec![
        0xb8, 0x00, 0x00, 0x00, 0x00,   // eax = 5
        0xc3,                           // ret
    ], false);

    linker.add_reloc( Link { from: "main".into(), to: "test".into(), at: 1, size: 4, replace: false} );
    linker.add_reloc( Link { from: "test".into(), to: "test_data".into(), at: 1, size: 4, replace: true} );
    linker.add_label("test_data", vec![5]);

    unsafe {
        let mut func: JitFunction<unsafe extern "C" fn() -> u32> = linker.engine();
        let out = func.call();
        println!("{}", out);
    }
}