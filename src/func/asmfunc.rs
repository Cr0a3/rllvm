use std::io::Cursor;
use x86asm::{Instruction, InstructionWriter, Mode};
use crate::contxt::link::Link;

/// Stores ir for function which can be compiled
pub struct AsmFunction {
    name: String,
    pub x86_asm_ir: Vec<Instruction>,
}

impl AsmFunction {
    /// Creates a function
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x86_asm_ir: vec![],
        }
    }

    /// Returns the name of the function
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Compiles the function
    pub fn compile(&self) -> Vec<u8> {
        let mut writer = InstructionWriter::new(Cursor::new(vec![]), Mode::Protected);

        let mut gen: Vec<u8> = vec![];

        for instr in self.x86_asm_ir.iter() {
            writer.write(instr).unwrap();
        }

        for byte in writer.get_inner_writer_ref().get_ref() {
            gen.push(*byte)
        }

        gen
    }

    /// Returns the relocs of the function
    pub fn relocs(&self) -> Vec<Link> {
        vec![]
    }

    /// Returns the data this function works with
    pub fn data(&self) -> Vec<(&str, Vec<u8>)> {
        vec![]
    }
}