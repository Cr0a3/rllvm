use iced_x86::{code_asm::*, Code, Instruction, MemoryOperand, Register};

use crate::func::AsmFunction;

use self::{ir::*, var::VarGen};

use super::{*};

pub trait Compile {
    /// Compiles the ir to 
    /// * `HashMap<String, Vec<u8>>` -> Data
    /// * `Vec<Link>`   -> Relocs (at is rel from the start of the ir)
    /// * `Vec<u8>`     -> Machine code
    fn compile(&self, _asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn out_reg(&self) -> Option<Register> {
        None
    }
}

impl Compile for Add<VarGen, VarGen> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {     
        let target = &self.inner1;   
        let src = &self.inner2;   

        if target.in_reg && src.in_reg {
            asm.asm.add_instruction(
                Instruction::with2(Code::Add_rm32_r32, target.reg, src.reg)?
            )?;
        } else if target.on_stack && src.on_stack {
            asm.make_stack_safe()?;
            println!("stack");
        }
        
        Ok(())
    }

    fn out_reg(&self) -> Option<Register> {
        Some(self.inner1.reg)
    }
}

impl Compile for Return<i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.mov(asm.call.ret32(), self.inner1)?;
        
        Ok(())
    }
}

impl Compile for Return<i64> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.mov(asm.call.ret64(), self.inner1)?;

        Ok(())
    }
}

impl Compile for Return<f32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        let mem = MemoryOperand::new(Register::RIP, Register::None, 1, 8, 1, false, Register::None);
        let instr = Instruction::with2(Code::Movd_xmm_rm32, asm.call.retf().into(), mem)?;

        asm.asm.add_instruction(instr)?;

        let req = asm.req_name();
        asm.reloc_at_current_pos(&req, 5, 4)?;
        asm.data.insert(req, self.inner1.to_be_bytes().into());
        Ok(())
    }
}

impl Compile for Return<f64> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        let mem = MemoryOperand::new(Register::RIP, Register::None, 1, 8, 1, false, Register::None);
        let instr = Instruction::with2(Code::Movd_xmm_rm32, asm.call.retf().into(), mem)?;

        asm.asm.add_instruction(instr)?;

        let req = asm.req_name();
        asm.reloc_at_current_pos(&req, 5, 4)?;
        asm.data.insert(req, self.inner1.to_be_bytes().into());
        
        Ok(())
    }
}

impl Compile for Return<Add<VarGen, VarGen>>{

    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        self.inner1.compile(asm)?;
        let reg = self.inner1.out_reg().unwrap(); // Implemented for add so it won't panic

        if reg.is_gpr64() {
            if reg != asm.call.ret64_reg() {
                asm.asm.add_instruction(Instruction::with2(Code::Mov_rm64_r64, asm.call.ret64_reg(), reg)?)?;
            }
        } else if reg.is_gpr32() {
            if reg.is_gpr32() {
                if reg != asm.call.ret64_reg() {
                    asm.asm.add_instruction(Instruction::with2(Code::Mov_rm32_r32, asm.call.ret32_reg(), reg)?)?;
                }
            }
        } else if reg.is_gpr16() {
            if reg != asm.call.ret16_reg() {
                asm.asm.add_instruction(Instruction::with2(Code::Mov_rm16_r16, asm.call.ret16_reg(), reg)?)?;
            }
        } else if reg.is_gpr8() {
            if reg != asm.call.ret8_reg() {
                asm.asm.add_instruction(Instruction::with2(Code::Mov_rm64_r64, asm.call.ret8_reg(), reg)?)?;
            }
        }

        Ok(())
    }
    
    fn out_reg(&self) -> Option<Register> {
        None
    }
}