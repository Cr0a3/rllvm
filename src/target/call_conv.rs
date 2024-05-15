use target_lexicon::CallingConvention;
use iced_x86::{code_asm::*, Register};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetCallConv {
    arg16: Vec<AsmRegister16>,
    arg32: Vec<AsmRegister32>,
    arg64: Vec<AsmRegister64>,
    argf: Vec<AsmRegisterXmm>,

    ret8: AsmRegister8,
    ret16: AsmRegister16,
    ret32: AsmRegister32,
    ret64: AsmRegister64,
    retf: AsmRegisterXmm,
    
    arg16_reg: Vec<Register>,
    arg32_reg: Vec<Register>,
    arg64_reg: Vec<Register>,
    argf_reg: Vec<Register>,

    ret8_reg: Register,
    ret16_reg: Register,
    ret32_reg: Register,
    ret64_reg: Register,
    retf_reg: Register,

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

            ret8: al,
            ret16: ax,
            ret32: eax,
            ret64: rax,
            retf: xmm0,

            
            arg16_reg:  vec![Register::SI, Register::DI, Register::DX, Register::CX, Register::R8W, Register::R9W],
            arg32_reg:  vec![Register::ESI,   Register::EDI,    Register::RDX,    Register::ECX,    Register::R8D,    Register::R9D ],
            arg64_reg:  vec![Register::RSI,   Register::RDI,    Register::RDX,    Register::RCX,    Register::R8,     Register::R9  ],

            argf_reg: vec![Register::XMM0,    Register::XMM1,   Register::XMM2,   Register::XMM3,   Register::XMM4,   Register::XMM5, Register::XMM6, Register::XMM7 ],

            ret8_reg: Register::AL,
            ret16_reg: Register::AX,
            ret32_reg: Register::EAX,
            ret64_reg: Register::RAX,
            retf_reg: Register::XMM0,

            shadow: 32,
        }
    }

    pub fn windows() -> Self {
        Self {
            arg16:  vec![dx,    cx,     r8w,    r9w                 ],
            arg32:  vec![edx,   ecx,    r8d,    r9d                 ],
            arg64:  vec![rdx,   rcx,    r8,     r9                  ],

            argf: vec![xmm0,    xmm1,   xmm2,   xmm3,   xmm4,   xmm5, xmm6, xmm7 ],

            ret8: al,
            ret16: ax,
            ret32: eax,
            ret64: rax,
            retf: xmm0,

            
            arg16_reg:  vec![Register::DX, Register::CX, Register::R8W, Register::R9W],
            arg32_reg:  vec![Register::EDX, Register::ECX, Register::R8D, Register::R9D],
            arg64_reg:  vec![Register::RDX, Register::RCX, Register::R8, Register::R9],

            argf_reg: vec![Register::XMM0, Register::XMM1, Register::XMM2, Register::XMM3, Register::XMM4, Register::XMM5, Register::XMM6, Register::XMM7 ],

            ret8_reg: Register::AL,
            ret16_reg: Register::AX,
            ret32_reg: Register::EAX,
            ret64_reg: Register::RAX,
            retf_reg: Register::XMM0,

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

    pub fn ret8(&self) -> AsmRegister8 {
        self.ret8
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

    pub fn arg16_reg(&self, nr: usize) -> Option<Register> {
        self.arg16_reg.get(nr).copied()
    }

    pub fn arg32_reg(&self, nr: usize) -> Option<Register> {
        self.arg32_reg.get(nr).copied()
    }

    pub fn arg64_reg(&self, nr: usize) -> Option<Register> {
        self.arg64_reg.get(nr).copied()
    }

    pub fn argf_reg(&self, nr: usize) -> Option<Register> {
        self.argf_reg.get(nr).copied()
    }

    pub fn ret8_reg(&self) -> Register {
        self.ret8_reg
    }

    pub fn ret16_reg(&self) -> Register {
        self.ret16_reg
    }

    pub fn ret32_reg(&self) -> Register {
        self.ret32_reg
    }

    pub fn ret64_reg(&self) -> Register {
        self.ret64_reg
    }

    pub fn retf_reg(&self) -> Register {
        self.retf_reg
    }
}