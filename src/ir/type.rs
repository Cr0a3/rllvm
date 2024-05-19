#![allow(non_camel_case_types)]

/// Stores Type information 
/// used for arguments and return types
/// 
/// ## Example
/// ```rust
/// use rllvm::ir::r#type::Type;
/// 
/// assert_eq!(Type::u64.reg(), true);
/// assert_eq!(Type::f32.reg(), true);
/// assert_eq!(Type::u64.stack(), false);
/// assert_eq!(Type::i8.size(), 1);
/// assert_eq!(Type::i32.name(), "i32");
/// ```
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
    /// Returns the size of the type
    /// Example:
    /// ```rust
    /// use rllvm::ir::r#type::Type;
    /// assert_eq!(Type::i16.size(), 2)
    /// ```
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

    /// Returns if the type is normally
    /// stored on the stack
    pub fn stack(&self) -> bool {
        match self {
            Type::u64 | Type::i64 => false,
            Type::u32 | Type::i32 => false,
            Type::u16 | Type::i16 => false,
            Type::u8  | Type::i8  => false,
            Type::f32 | Type::f64 => false,
        }
    }

    /// Returns if the type is normally
    /// stored in registers
    pub fn reg(&self) -> bool {
        match self {
            Type::u64 | Type::i64 => true,
            Type::u32 | Type::i32 => true,
            Type::u16 | Type::i16 => true,
            Type::u8  | Type::i8  => true,
            Type::f32 | Type::f64 => true,
        }
    }

    /// Returns the name of the types
    pub fn name(&self) -> &str {
        match self {
            Type::u64 => "u64",
            Type::u32 => "u32",
            Type::u16 => "u16",
            Type::u8 => "u8",
            Type::i64 => "i64",
            Type::i32 => "i32",
            Type::i16 => "i16",
            Type::i8 => "i8",
            Type::f64 => "f64",
            Type::f32 => "f32",
        }
    }
}