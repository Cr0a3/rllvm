use std::{collections::{HashMap, VecDeque}, error::Error};

use iced_x86::code_asm::*;
use crate::{contxt::{contxt::Context, link::Link}, ir::{r#type::Type, var::VarGen}, target::call_conv::TargetCallConv};

/// Stores the ir for function which can be compiled
pub struct AsmFunction {
    pub name: String,
    pub asm: CodeAssembler,
    gen: VecDeque<u8>,
    pub relocs: Vec<(Link, usize)>,
    pub data: HashMap<String, Vec<u8>>,

    pub call: TargetCallConv,

    req_names: usize,

    stack_safe: bool,

    pub args: Vec<Type>,
    pub ret: Type,
}

impl AsmFunction {
    /// Creates a function
    pub fn new(name: &str, contxt: &Context) -> Self {
        Self {
            name: name.to_string(),
            asm: CodeAssembler::new(64).unwrap(), // unwrap because i i made it just so it can't give error
            relocs: vec![],
            gen: VecDeque::new(),
            data: HashMap::new(),
            call: contxt.call.clone(),
            req_names: 0,
            stack_safe: false,
            args: vec![],
            ret: Type::u32,
        }
    }

    /// Returns the name of the function
    pub fn name(&mut self) -> &str {
        &self.name
    }

    /// Makes the function stack safe so you can use the stack
    pub fn make_stack_safe(&mut self) -> Result<(), Box<dyn Error>> {
        self.stack_safe = true;

        let mut asm = CodeAssembler::new(64)?;

        asm.endbr64()?;
        asm.push(rbp)?;
        asm.mov(rbp, rsp)?;
        asm.sub(rsp, self.call.shadow as i32)?;

        let gen = asm.assemble(0)?;

        for byte in gen {
            self.gen.push_front( byte );
        }

        Ok(())
    }

    /// Compiles the function (a return will automaticly be added)
    pub fn compile(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        self.gen_current().unwrap();
        let mut ret: Vec<u8> = self.gen.clone().into();

        if self.stack_safe {
            let mut asm = CodeAssembler::new(64)?;

            asm.add(rsp, self.call.shadow as i32)?;
            asm.pop(rbp)?;
            asm.ret()?;

            let gen = asm.assemble(0)?;

            for byte in gen {
                ret.push( byte );
            }
        }

        Ok(ret)
    }

    fn gen_current(&mut self) -> Result<(), Box<dyn Error>>{
        let gen = self.asm.assemble(0)?;

        for byte in gen {
            self.gen.push_back( byte );
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

    /// Returns the argument as a variable (or None if the index isn't found)
    /// 
    /// **IMPORTANT:** maybe argument registers get overwritten so it points to an invalid value
    pub fn arg(&self, nr: usize) -> Option<VarGen> {
        let get = self.args.get(nr);

        if get.is_none() {
            return None;
        }

        let mut index = 0;
        let mut reg_args = 0;
        let mut adr = 0;

        for arg in &self.args {

            if index == nr { break; }

            if arg.reg() {
                reg_args += 1;
            }

            if arg.stack() {
                adr += arg.size();
            }

            index += 1;
        }

        let typ = get.unwrap();

        if typ.reg() {
            let reg = {
                match typ {
                    Type::u64 | Type::i64 => self.call.arg64_reg(reg_args),
                    Type::u32 | Type::i32 => self.call.arg32_reg(reg_args),
                    Type::u16 | Type::i16 => self.call.arg16_reg(reg_args),
                    Type::u8  | Type::i8  => self.call.arg16_reg(reg_args),
                    Type::f64 | Type::f32 => self.call.argf_reg(reg_args),
                }
            };

            Some(VarGen::new_reg(*typ, reg?))
        } else if typ.stack() {
            Some(VarGen::new_stack(*typ, adr))
        } else {
            None // invalid type or dummy type
        }
    }
}