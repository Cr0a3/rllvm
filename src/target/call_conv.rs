use target_lexicon::CallingConvention;
use iced_x86::code_asm::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetCallConv {
    arg16: Vec<AsmRegister16>,
    arg32: Vec<AsmRegister32>,
    arg64: Vec<AsmRegister64>,
    argf: Vec<AsmRegisterXmm>,

    ret16: AsmRegister16,
    ret32: AsmRegister32,
    ret64: AsmRegister64,
    retf: AsmRegisterXmm,

    /// Stack shadow space
    pub shadow: usize,
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

            argf: vec![xmm0,    xmm1,   xmm2,   xmm3,   xmm4,   xmm5, xmm6, xmm7 ],

            ret16: ax,
            ret32: eax,
            ret64: rax,
            retf: xmm0,

            shadow: 32,
        }
    }

    pub fn windows() -> Self {
        Self {
            arg16:  vec![dx,    cx,     r8w,    r9w                 ],
            arg32:  vec![edx,   ecx,    r8d,    r9d                 ],
            arg64:  vec![rdx,   rcx,    r8,     r9                  ],

            argf: vec![xmm0,    xmm1,   xmm2,   xmm3,   xmm4,   xmm5, xmm6, xmm7 ],

            ret16: ax,
            ret32: eax,
            ret64: rax,
            retf: xmm0,

            shadow: 32,
        }
    }

    pub fn arg16(&self, nr: usize) -> Option<AsmRegister16> {
        self.arg16.get(nr).copied()
    }

    pub fn arg32(&self, nr: usize) -> Option<AsmRegister32> {
        self.arg32.get(nr).copied()
    }

    pub fn arg64(&self, nr: usize) -> Option<AsmRegister64> {
        self.arg64.get(nr).copied()
    }

    pub fn argf(&self, nr: usize) -> Option<AsmRegisterXmm> {
        self.argf.get(nr).copied()
    }

    pub fn ret16(&self) -> AsmRegister16 {
        self.ret16
    }

    pub fn ret32(&self) -> AsmRegister32 {
        self.ret32
    }

    pub fn ret64(&self) -> AsmRegister64 {
        self.ret64
    }

    pub fn retf(&self) -> AsmRegisterXmm {
        self.retf
    }
}