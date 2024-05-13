use iced_x86::{code_asm::*, Code, Instruction, MemoryOperand, Register};

use crate::func::AsmFunction;

use self::ir::*;

use super::{*};

pub trait Compile {
    /// Compiles the ir to 
    /// * `HashMap<String, Vec<u8>>` -> Data
    /// * `Vec<Link>`   -> Relocs (at is rel from the start of the ir)
    /// * `Vec<u8>`     -> Machine code
    fn compile(&self, _asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Compile for Add<AsmRegister8, i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.add(self.inner1, self.inner2)?;
        
        Ok(())
    }
}

impl Compile for Add<AsmRegister16, i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.add(self.inner1, self.inner2)?;
        Ok(())
    }
}

impl Compile for Add<AsmRegister32, i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.add(self.inner1, self.inner2)?;
        
        Ok(())
    }
}

impl Compile for Add<AsmRegister64, i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {        
        asm.asm.add(self.inner1, self.inner2)?;
        
        Ok(())
    }
}

impl Compile for Return<i32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.mov(asm.call.ret32(), self.inner1)?;
        asm.asm.ret()?;
        
        Ok(())
    }
}

impl Compile for Return<i64> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.mov(asm.call.ret64(), self.inner1)?;
        asm.asm.ret()?;
        
        Ok(())
    }
}

impl Compile for Return<f32> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        let mem = MemoryOperand::new(Register::RIP, Register::None, 1, 0, 1, false, Register::None);
        let instr = Instruction::with2(Code::Movd_xmm_rm32, asm.call.retf().into(), mem)?;

        asm.asm.add_instruction(instr)?;

        let req = asm.req_name();
        asm.reloc_at_current_pos(&req, 5, 4)?;
        asm.data.insert(req, self.inner1.to_be_bytes().into());

        asm.asm.ret()?;
        
        Ok(())
    }
}

impl Compile for Return<f64> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        let mem = MemoryOperand::new(Register::RIP, Register::None, 1, 0, 1, false, Register::None);
        let instr = Instruction::with2(Code::Movd_xmm_rm32, asm.call.retf().into(), mem)?;

        asm.asm.add_instruction(instr)?;

        let req = asm.req_name();
        asm.reloc_at_current_pos(&req, 5, 4)?;
        asm.data.insert(req, self.inner1.to_be_bytes().into());
        
        Ok(())
    }
}