use rllvm::naming::NamingGenerator;

fn main() {
    let gen = NamingGenerator::new();

    let add = gen.generate(
        vec!["test"], 
        None, 
        "add", 
        vec!["u32", "u32"], 
        "u32"
    );

    println!("{}", add);

    assert_eq!(&add, "_R4testZ0Z3addZ3u323u32Z3u32");
}