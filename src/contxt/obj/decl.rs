#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Decl {
    /// Function
    Function(Scope),
    
    /// Uninitialized data
    UData(Scope),

    /// Read only data
    RData(Scope),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    Import,
    Export,
    Private,
}