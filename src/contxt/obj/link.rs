#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub from: String,
    pub to: String,
    pub at: usize,
}