use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Unknown,
    Int32,
    Int64,
    String,
    Bool,
    Unit,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unknown => write!(f, "unknown"),
            Type::Int32 => write!(f, "int32"),
            Type::Int64 => write!(f, "int64"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Unit => write!(f, "unit"),
        }
    }
}
