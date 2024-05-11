use target_lexicon::CallingConvention;
use iced_x86::code_asm::*;

pub struct TargetCallConv {
    arg16: Vec<AsmRegister16>,
    arg32: Vec<AsmRegister32>,
    arg64: Vec<AsmRegister64>,

    argf32: Vec<AsmRegisterSt>,
}

impl TargetCallConv {
    pub fn new(conv: CallingConvention) -> Self {
        match conv {
            CallingConvention::SystemV => TargetCallConv::linux(),
            CallingConvention::WindowsFastcall => TargetCallConv::windows(),
            _ => todo!(),
        }
    }

    pub fn linux() -> Self {
        Self {
            arg16:  vec![si,     di,    dx,     cx,     r8w,    r9w ],
            arg32:  vec![esi,   edi,    edx,    ecx,    r8d,    r9d ],
            arg64:  vec![rsi,   rdi,    rdx,    rcx,    r8,     r9  ],

            argf32: vec![st0,   st1,    st2,    st3,    st4,    st5 ],
        }
    }

    pub fn windows() -> Self {
        Self {
            arg16:  vec![dx,    cx,     r8w,    r9w                 ],
            arg32:  vec![edx,   ecx,    r8d,    r9d                 ],
            arg64:  vec![rdx,   rcx,    r8,     r9                  ],

            argf32: vec![st0,   st1,    st2,    st3,    st4,    st5 ],
        }
    }
}