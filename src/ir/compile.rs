use iced_x86::code_asm::*;

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
        
        Ok(())
    }
}

impl Compile for Return<i64> {
    fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        asm.asm.mov(asm.call.ret64(), self.inner1)?;
        
        Ok(())
    }
}