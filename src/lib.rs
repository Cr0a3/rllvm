//! RLLVM

#[cfg(feature = "function")]
pub mod func;

#[cfg(feature = "ir")]
pub mod ir;

pub mod contxt;
pub mod target;

pub mod naming;

pub use target_lexicon;