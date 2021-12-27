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
    Call(Call),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDecl {
    pub id: String,
    pub init: Option<Expr>,
}
impl VariableDecl {
    pub fn new(id: String, init: Option<Expr>) -> Self {
        VariableDecl { id, init }
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
    pub argc: usize,
    pub stmts: Stmts,
}

impl FunctionDecl {
    pub fn new(id: String, argc: usize, stmts: Stmts) -> Self {
        Self { id, argc, stmts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub id: String,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn new(id: String, args: Vec<Expr>) -> Self {
        Self { id, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    VariableDecl(VariableDecl),
    Assing(Assing),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program(pub Vec<FunctionDecl>);

impl Program {
    pub fn new(functions: Vec<FunctionDecl>) -> Self {
        Self(functions)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assing {
    pub left: String,
    pub right: Expr,
}

impl Assing {
    pub fn new(left: String, right: Expr) -> Self {
        Self { left, right }
    }
}