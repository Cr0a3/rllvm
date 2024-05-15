use std::{error::Error, ops::{Add, Div, Mul, Sub}};

use iced_x86::{code_asm::*, Code, Instruction, Register};
use super::{compile::Compile, r#type::Type};

/// A variable code gen helper
pub struct VarGen {
    pub on_stack: bool,
    pub in_reg: bool,

    pub stack_adr: usize,
    pub reg: Register,

    pub typ: Type,
}

impl VarGen {
    /// Creates a new VarGen
    pub fn new(typ: Type) -> Self {
        Self {
            on_stack: typ.stack(),
            in_reg: typ.reg(),
            stack_adr: 0,
            reg: Register::None,

            typ: typ,
        }
    }

    /// Creates a new VarGen where the var is stored in an register
    pub fn new_reg(typ: Type, reg: Register) -> Self {
        Self {
            on_stack: typ.stack(),
            in_reg: true,
            stack_adr: 0,
            reg: reg,

            typ: typ,
        }
    }

    /// Mov the value from the target register into the register in which the var is stored
    pub fn set_reg(&mut self, target: Register, asm: &mut CodeAssembler) -> Result<(), Box<dyn Error>> {
        match self.typ {
            Type::u64 | Type::i64 => {
                asm.add_instruction(
                    Instruction::with2(Code::Mov_rm64_r64, self.reg, target)?
                )?;
            },
            Type::u32 | Type::i32 => {
                asm.add_instruction(
                    Instruction::with2(Code::Mov_rm32_r32, self.reg, target)?
                )?;
            },
            Type::u16 | Type::i16 => {
                asm.add_instruction(
                    Instruction::with2(Code::Mov_rm16_r16, self.reg, target)?
                )?;
            },
            Type::u8 | Type::i8 => {
                asm.add_instruction(
                    Instruction::with2(Code::Mov_rm8_r8, self.reg, target)?
                )?;
            },
        };

        Ok(())
    }

    /// Moves the stored value onto the stack
    pub fn mov_to_stack(&mut self, stack_base: usize, asm: &mut CodeAssembler) -> Result<(usize, usize), Box<dyn Error>> {
        let size = self.typ.size();

        let new_base = stack_base + size;
        let mut adr = stack_base;

        if self.on_stack {
            adr = self.stack_adr;
            return Ok( (new_base, adr) );
        }

        self.in_reg     = false;
        self.on_stack   = true;

        match self.typ {
            Type::u64 | Type::i64 => {
                asm.add_instruction(
                    Instruction::with1(Code::Mov_rm64_r64, self.reg)?
                )?;
            },
            Type::u32 | Type::i32 => {
                asm.add_instruction(
                    Instruction::with1(Code::Mov_rm32_r32, self.reg)?
                )?;
            },
            Type::u16 | Type::i16 => {
                asm.add_instruction(
                    Instruction::with1(Code::Mov_rm16_r16, self.reg)?
                )?;
            },
            Type::u8 | Type::i8 => {
                asm.add_instruction(
                    Instruction::with1(Code::Mov_rm8_r8, self.reg)?
                )?;
            },
        };

        Ok( (new_base, adr) )
    }
}
/* 
impl Add<VarGen> for VarGen {
    fn add(self, rhs: VarGen) -> Add<VarGen, VarGen> {
        super::ir::Add::new(
            self, rhs
        )
    }
}*/