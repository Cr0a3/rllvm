use iced_x86::{Code, Instruction, MemoryOperand, Register};

use crate::func::AsmFunction;

use self::{ir::*, r#type::Type, var::VarGen};

use super::{*};


pub trait Compile {
    fn compile(&self, _asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn out_reg(&self) -> Option<Register> {
        None
    }
}
macro_rules! MathStructVarGenAdd {
    ($name:tt, $_64:expr, $_32:expr, $_16:expr, $_8:expr, $_f64:expr, $_f32:expr) => {
        impl Compile for $name<VarGen, VarGen> {
            fn compile(&self, asm: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {     
                let target = &self.inner1;   
                let src = &self.inner2;   
        
                if target.in_reg && src.in_reg {
                    let target_reg = target.reg;
        
                    if target_reg.is_gpr64() {
                        asm.asm.add_instruction(
                            Instruction::with2($_64, target.reg, src.reg)?
                        )?;
                    } else if target_reg.is_gpr32() {
                        asm.asm.add_instruction(
                            Instruction::with2($_32, target.reg, src.reg)?
                        )?;
                    } else if target_reg.is_gpr16() {
                        asm.asm.add_instruction(
                            Instruction::with2($_16, target.reg, src.reg)?
                        )?;
                    } else if target_reg.is_gpr8() {
                        asm.asm.add_instruction(
                            Instruction::with2($_8, target.reg, src.reg)?
                        )?;
                    } else if target_reg.is_xmm() {
                        let code = {
                            if target.typ == Type::f64 {
                                $_f64
                            } else {
                                $_f32
                            }
                        };
        
                        asm.asm.add_instruction(
                            Instruction::with2(code, target.reg, src.reg)?
                        )?;
                    } 
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
    }
}

MathStructVarGenAdd!(Add,
    Code::Add_rm64_r64, 
    Code::Add_rm32_r32, 
    Code::Add_rm16_r16, 
    Code::Add_rm8_r8, 
    Code::Addsd_xmm_xmmm64, 
    Code::Addss_xmm_xmmm32
);

MathStructVarGenAdd!(Sub,
    Code::Sub_rm64_r64, 
    Code::Sub_rm32_r32, 
    Code::Sub_rm16_r16, 
    Code::Sub_rm8_r8, 
    Code::Subsd_xmm_xmmm64, 
    Code::Subss_xmm_xmmm32
);



MathStructVarGenAdd!(Mul,
    Code::Imul_r64_rm64, 
    Code::Imul_r32_rm32, 
    Code::Imul_r16_rm16, 
    Code::Zero_bytes, 
    Code::Mulsd_xmm_xmmm64, 
    Code::Mulss_xmm_xmmm32
);

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
        asm.reloc_at_current_pos(&req, 4, 4)?;
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

macro_rules! ExprReturn {
    ($name:tt) => {
        impl<T, U> Compile for Return<$name<T, U>> where $name<T, U>: Compile {
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
    };
}

ExprReturn!(Add);
ExprReturn!(Sub);
ExprReturn!(Mul);