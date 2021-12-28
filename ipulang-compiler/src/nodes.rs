use crate::types::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Or,  // ||
    And, // &&
    Eq,  // ==
    Neq, // !=
    Geq, // >=
    Leq, // <=
    Gt,  // >
    Lt,  // <
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
}

/// 定数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Const {
    I32Const(i32),
    I64Const(i64),
    BoolConst(bool),
}

impl Const {
    pub fn new_i32(val: i32) -> Const {
        Const::I32Const(val)
    }

    pub fn new_i64(val: i64) -> Const {
        Const::I64Const(val)
    }

    pub fn new_bool(val: bool) -> Const {
        Const::BoolConst(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp {
    pub left: Expr,
    pub op: Op,
    pub right: Expr,
    pub ty: Type,
}

impl BinOp {
    pub fn new(left: Expr, op: Op, right: Expr, ty: Type) -> Self {
        Self {
            left,
            op,
            right,
            ty,
        }
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
    pub ty: Type,
    pub init: Option<Expr>,
}
impl VariableDecl {
    pub fn new(id: String, ty: Type, init: Option<Expr>) -> Self {
        VariableDecl { id, ty, init }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub id: String,
    pub ty: Type,
}

impl Variable {
    pub fn new(id: String, ty: Type) -> Self {
        Variable { id, ty }
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
    pub args: Vec<Variable>,
    pub ret_typ: Type,
    pub stmts: Stmts,
}

impl FunctionDecl {
    pub fn new(id: String, args: Vec<Variable>, ret_typ: Type, stmts: Stmts) -> Self {
        Self {
            id,
            args,
            ret_typ,
            stmts,
        }
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
pub struct IfElse {
    pub cond: Expr,
    pub success: Stmts,
    pub failure: Option<Stmts>,
}

impl IfElse {
    pub fn new(cond: Expr, success: Stmts, failure: Option<Stmts>) -> Self {
        Self {
            cond,
            success,
            failure,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For {
    pub var_decl: VariableDecl,
    pub cond: Expr,
    pub assign: Assign,
    pub stmts: Stmts,
}

impl For {
    pub fn new(var_decl: VariableDecl, cond: Expr, assign: Assign, stmts: Stmts) -> Self {
        Self {
            var_decl,
            cond,
            assign,
            stmts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    VariableDecl(VariableDecl),
    Assign(Assign),
    IfElse(IfElse),
    For(For),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program(pub Vec<FunctionDecl>);

impl Program {
    pub fn new(functions: Vec<FunctionDecl>) -> Self {
        Self(functions)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
    pub left: String,
    pub right: Expr,
}

impl Assign {
    pub fn new(left: String, right: Expr) -> Self {
        Self { left, right }
    }
}
