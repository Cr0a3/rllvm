use std::error::Error;

use iced_x86::code_asm::*;
use crate::contxt::link::Link;

/// Stores ir for function which can be compiled
pub struct AsmFunction {
    pub name: String,
    pub asm: CodeAssembler,
    gen: Vec<u8>,
    pub relocs: Vec<(Link, usize)>,
}

impl AsmFunction {
    /// Creates a function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            asm: CodeAssembler::new(64).unwrap(), // unwrap because i i made it just so it can't give error
            relocs: vec![],
            gen: vec![],
        }
    }

    /// Returns the name of the function
    pub fn name(&mut self) -> &str {
        &self.name
    }

    /// Compiles the function
    pub fn compile(&mut self) -> Vec<u8> {
        self.gen_current().unwrap();
        self.gen.clone()
    }

    fn gen_current(&mut self) -> Result<(), Box<dyn Error>>{
        let gen = self.asm.assemble(0)?;

        for byte in gen {
            self.gen.push( byte );
        }

        self.asm.reset();

        Ok(())
    }

    pub fn reloc_at_current_pos(&mut self, to: &str, rel: isize, size: usize) -> Result<(), Box<dyn Error>> {
        self.gen_current()?;

        let pos = self.gen.len();

        let link = Link { from: self.name.clone(), to: to.to_string(), at: (pos as isize + rel) as usize, size: size };

        self.relocs.push((link, pos));

        Ok(())
    }

    /// Returns the relocs of the function
    pub fn relocs(&self) -> Vec<Link> {
        let mut ret = vec![];

        for reloc in &self.relocs {
            ret.push(reloc.0.clone())
        }

        ret
    }

    /// Returns the data this function works with
    pub fn data(&self) -> Vec<(&str, Vec<u8>)> {
        vec![]
    }
}