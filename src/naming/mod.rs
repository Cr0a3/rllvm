//! RLLVM Naming Convention

/// Generate names in in the **RLLVM Naming Convention** format
pub struct NamingGenerator {}

impl NamingGenerator {
    /// Creates a new naming generator
    pub fn new() -> Self {
        Self {

        }
    }

    /// Generates a new name which follows rllvm naming convention
    /// 
    /// Args:
    /// - namespaces: The namespaces the func is in (e.g: `rllvm::naming`)
    /// - class: The class the func is in (can be empty) (e.g: `NamingGenerator`)
    /// - func: The name of the func (e.g. `generate`)
    /// - args: The arguments of the function (e.g. u32, u32)
    /// - ret: The return type of the function (e.g. String)
    pub fn generate(&self, namespaces: Vec<&str>, class: Option<&str>, func: &str, args: Vec<&str>, ret: &str) -> String {
        let mut gen = String::from("_R");

        if namespaces.len() == 0 {
            gen.push('0');
        }

        for namespace in namespaces {
            let len = namespace.len();
            gen.push_str(&format!("{}",len));
            gen.push_str(namespace);
        }    

        gen.push('Z'); // seperator

        if let Some(name) = class {
            let len = name.len();
            gen.push_str(&format!("{}",len));
            gen.push_str(name);
        } else {
            gen.push('0');
        }  

        gen.push('Z'); // seperator

        let len = func.len();
        gen.push_str(&format!("{}",len));
        gen.push_str(func);

        gen.push('Z'); // seperator

        if args.len() == 0 {
            gen.push('0');
        }

        for arg in args {
            let len = arg.len();
            gen.push_str(&format!("{}",len));
            gen.push_str(arg);
        }

        gen.push('Z'); // seperator

        let len = ret.len();
        gen.push_str(&format!("{}",len));
        gen.push_str(ret);

        gen
    }
}