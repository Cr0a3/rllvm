use rllvm::contxt::{jit::JitFunction, link::{JitLinker, Link}};

#[test]
pub fn jit_function() {
    let mut func: JitFunction<unsafe extern "C" fn() -> u32> = JitFunction::new(
        vec![
                0xb8, 0x05, 0x00, 0x00, 0x00,   // mov eax, 5
                0xc3,                           // ret
            ],
    );
    unsafe {
        let out = func.call();
        assert_eq!(out, 5)
    }
}

#[test]
fn jit_linker() {
    let mut linker = JitLinker::new();

    linker.add_func("main", vec![
        0xe8, 0x00, 0x00, 0x00, 0x00,   // call test
        0xc3,                           // ret
    ], true);

    linker.add_func("test", vec![
        0xb8, 0x05, 0x00, 0x00, 0x00,   // eax = 5
        0xc3,                           // ret
    ], false);

    linker.relocs.push( Link { from: "main".into(), to: "test".into(), at: 1, size: 4});

    let mut func: JitFunction<unsafe extern "C" fn() -> u32> = unsafe { linker.engine() };

    unsafe {
        let out = func.call();
        assert_eq!(out, 5);
    }
}