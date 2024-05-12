use iced_x86::code_asm::*;

use crate::func::AsmFunction;

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

macro_rules! IrTypeImpl {
    ( <$( $types:ident ),*>, $name:tt, $asm_name:tt => $def:expr) => {
        impl<$( $types ),*> Compile for $name<$( $types ),*> {
            fn compile(&self, $asm_name: &mut AsmFunction) -> Result<(), Box<dyn std::error::Error>> {
                $def

                Ok(())
            }
        }
    };
}


IrTypeImpl!(<T, U>, Add, asm => {
    asm.asm.mov(eax, 5)?;
});