use crate::types::Type;
use derivative::Derivative;
use nom_locate::LocatedSpan;

// TODO: introduce lifetime parameter
pub type Span<'a> = LocatedSpan<&'a str>;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Const {
    Unit,
    I32Const(i32),
    I64Const(i64),
    BoolConst(bool),
    String(String),
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

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq)]
pub struct BinOp<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub left: Expr<'a>,
    pub op: Op,
    pub right: Expr<'a>,
    pub ty: Type,
}

impl<'a> BinOp<'a> {
    pub fn new(position: Span<'a>, left: Expr<'a>, op: Op, right: Expr<'a>, ty: Type) -> Self {
        Self {
            position,
            left,
            op,
            right,
            ty,
        }
    }
}

/// 式
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Const(Const),
    Variable(Variable<'a>),
    BinOp(Box<BinOp<'a>>),
    Call(Call<'a>),
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct VariableDecl<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub id: String,
    pub ty: Type,
    pub init: Option<Expr<'a>>,
}

impl<'a> VariableDecl<'a> {
    pub fn new(position: Span<'a>, id: String, ty: Type, init: Option<Expr<'a>>) -> Self {
        VariableDecl {
            position,
            id,
            ty,
            init,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Variable<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub id: String,
    pub ty: Type,
}

impl<'a> Variable<'a> {
    pub fn new(position: Span<'a>, id: String, ty: Type) -> Self {
        Variable { position, id, ty }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Stmts<'a>(pub Vec<Stmt<'a>>);

impl<'a> Stmts<'a> {
    pub fn new(stmts: Vec<Stmt<'a>>) -> Self {
        Stmts(stmts)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct FunctionDecl<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub id: String,
    pub args: Vec<Variable<'a>>,
    pub ret_typ: Type,
    pub stmts: Stmts<'a>,
}

impl<'a> FunctionDecl<'a> {
    pub fn new(
        position: Span<'a>,
        id: String,
        args: Vec<Variable<'a>>,
        ret_typ: Type,
        stmts: Stmts<'a>,
    ) -> Self {
        Self {
            position,
            id,
            args,
            ret_typ,
            stmts,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub id: String,
    pub args: Vec<Expr<'a>>,
}

impl<'a> Call<'a> {
    pub fn new(position: Span<'a>, id: String, args: Vec<Expr<'a>>) -> Self {
        Self { position, id, args }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct IfElse<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub cond: Expr<'a>,
    pub success: Stmts<'a>,
    pub failure: Option<Stmts<'a>>,
}

impl<'a> IfElse<'a> {
    pub fn new(
        position: Span<'a>,
        cond: Expr<'a>,
        success: Stmts<'a>,
        failure: Option<Stmts<'a>>,
    ) -> Self {
        Self {
            position,
            cond,
            success,
            failure,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct For<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub var_decl: VariableDecl<'a>,
    pub cond: Expr<'a>,
    pub assign: Assign<'a>,
    pub stmts: Stmts<'a>,
}

impl<'a> For<'a> {
    pub fn new(
        position: Span<'a>,
        var_decl: VariableDecl<'a>,
        cond: Expr<'a>,
        assign: Assign<'a>,
        stmts: Stmts<'a>,
    ) -> Self {
        Self {
            position,
            var_decl,
            cond,
            assign,
            stmts,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Return(Expr<'a>),
    VariableDecl(VariableDecl<'a>),
    Assign(Assign<'a>),
    IfElse(IfElse<'a>),
    For(For<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Program<'a>(pub Vec<FunctionDecl<'a>>);

impl<'a> Program<'a> {
    pub fn new(functions: Vec<FunctionDecl<'a>>) -> Self {
        Self(functions)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Assign<'a> {
    #[derivative(PartialEq = "ignore")]
    pub position: Span<'a>,
    pub left: String,
    pub right: Expr<'a>,
}

impl<'a> Assign<'a> {
    pub fn new(position: Span<'a>, left: String, right: Expr<'a>) -> Self {
        Self {
            position,
            left,
            right,
        }
    }
}
