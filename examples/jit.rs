use rllvm::contxt::jit::JitFunction;

type SumFunc = unsafe extern "C" fn(u64, u64) -> u64;

#[rustfmt::skip]
pub fn main() {
    let mut func: JitFunction<SumFunc> = JitFunction::new(
        vec![
                0xb8, 0x05, 0x00, 0x00, 0x00,   // mov eax, 5
                0xc3,                           // ret
            ],
    );
    unsafe {
        let out = func.call(1, 1);
        println!("{}", out);
        func.free();
    }
}