use std::{collections::HashMap, error::Error};

use iced_x86::code_asm::*;
use crate::{contxt::{contxt::Context, link::Link}, target::call_conv::TargetCallConv};

/// Stores ir for function which can be compiled
pub struct AsmFunction {
    pub name: String,
    pub asm: CodeAssembler,
    gen: Vec<u8>,
    pub relocs: Vec<(Link, usize)>,
    pub data: HashMap<String, Vec<u8>>,

    pub call: TargetCallConv,

    req_names: usize,
}

impl AsmFunction {
    /// Creates a function
    pub fn new(name: &str, contxt: &Context) -> Self {
        Self {
            name: name.to_string(),
            asm: CodeAssembler::new(64).unwrap(), // unwrap because i i made it just so it can't give error
            relocs: vec![],
            gen: vec![],
            data: HashMap::new(),
            call: contxt.call.clone(),
            req_names: 0,
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

        let link = Link { from: self.name.clone(), to: to.to_string(), at: (pos as isize + rel) as usize, size: size, replace: false };

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
        let mut ret: Vec<(&str, Vec<u8>)> = vec![];

        for data in &self.data {
            ret.push( (data.0, data.1.to_owned()) );
        }

        ret
    }

    /// Requests a new name for a label
    pub fn req_name(&mut self) -> String {
        let req = format!("{}", self.req_names);
        self.req_names += 1;

        req
    }
}