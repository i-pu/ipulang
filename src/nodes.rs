/// 定数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Const(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Const {
    pub fn new(val: i32) -> Const {
        Const(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp {
    left: Expr,
    op: Op,
    right: Expr,
}

impl BinOp {
    pub fn new(left: Expr, op: Op, right: Expr) -> Self {
        Self { left, op, right }
    }
}

impl Node for Const {
    fn gen_code(self) -> String {
        format!("lit {}\n", self.0)
    }
}

impl Node for BinOp {
    fn gen_code(self) -> String {
        format!(
            "{}{}{}",
            self.left.gen_code(),
            self.right.gen_code(),
            match self.op {
                Op::Add => "add\n",
                Op::Sub => "sub\n",
                Op::Mul => "mul\n",
                Op::Div => "div\n",
            }
        )
    }
}

pub trait Node {
    fn gen_code(self) -> String;
}

/// 式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Const),
    BinOp(Box<BinOp>),
}

impl Node for Expr {
    fn gen_code(self) -> String {
        match self {
            Expr::Const(val) => val.gen_code(),
            Expr::BinOp(binOp) => binOp.gen_code(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let ast = BinOp::new(
            Expr::Const(Const::new(3)),
            Op::Add,
            Expr::Const(Const::new(3)),
        );
        let code = ast.gen_code();
        assert_eq!(
            code,
            r#"lit 3
lit 3
add
"#
        )
    }
}
