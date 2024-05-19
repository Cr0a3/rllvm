use rllvm::prelude::*;

#[rustfmt::skip]
fn main() {
    let mut func: JitFunction<unsafe extern "C" fn() -> u32> = JitFunction::new(
        vec![
                0xb8, 0x05, 0x00, 0x00, 0x00,   // mov eax, 5
                0xc3,                           // ret
            ],
    );
    unsafe {
        let out = func.call();
        println!("{}", out);
    }
}