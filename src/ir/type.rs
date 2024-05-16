#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    u64,
    u32,
    u16,
    u8,

    i64,
    i32,
    i16,
    i8,

    f64,
    f32,
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::u64 | Type::i64 => 8,
            Type::u32 | Type::i32 => 4,
            Type::u16 | Type::i16 => 2,
            Type::u8  | Type::i8  => 1,

            Type::f64 => 8,
            Type::f32 => 4,
        }
    }

    pub fn stack(&self) -> bool {
        match self {
            Type::u64 | Type::i64 => false,
            Type::u32 | Type::i32 => false,
            Type::u16 | Type::i16 => false,
            Type::u8  | Type::i8  => false,
            Type::f32 | Type::f64 => false,
        }
    }

    pub fn reg(&self) -> bool {
        match self {
            Type::u64 | Type::i64 => true,
            Type::u32 | Type::i32 => true,
            Type::u16 | Type::i16 => true,
            Type::u8  | Type::i8  => true,
            Type::f32 | Type::f64 => true,
        }
    }
}