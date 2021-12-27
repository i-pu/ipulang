use std::env::VarError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// 定数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Const(pub i32);

impl Const {
    pub fn new(val: i32) -> Const {
        Const(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp {
    pub left: Expr,
    pub op: Op,
    pub right: Expr,
}

impl BinOp {
    pub fn new(left: Expr, op: Op, right: Expr) -> Self {
        Self { left, op, right }
    }
}

/// 式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Const),
    Variable(Variable),
    BinOp(Box<BinOp>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDecl(pub String);
impl VariableDecl {
    pub fn new(id: String) -> Self {
        VariableDecl(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(pub String);

impl Variable {
    pub fn new(id: String) -> Self {
        Variable(id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmts(pub Vec<Stmt>);

impl Stmts {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Stmts(stmts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDecl {
    pub id: String,
    pub stmts: Stmts,
}

impl FunctionDecl {
    pub fn new(id: String, stmts: Stmts) -> Self {
        Self { id, stmts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    VariableDecl(VariableDecl),
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Stmts(pub Vec<Stmt>);
