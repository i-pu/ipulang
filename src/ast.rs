use std::str::FromStr;

use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::nodes::{BinOp, Const, Expr, Op};

/// "64hoge" -> hoge, <64>
pub fn const_parser(s: &str) -> IResult<&str, Expr> {
    // dbg!("const_parser");
    let (ss, lit) = digit1(s)?;
    let val = FromStr::from_str(lit).unwrap();
    Ok((ss, Expr::Const(Const::new(val))))
}

// 加減算
pub fn op1_parser(s: &str) -> IResult<&str, Op> {
    // dbg!("op_parser");
    let (ss, op) = one_of("+-")(s)?;
    let op = match op {
        '+' => Op::Add,
        '-' => Op::Sub,
        _ => panic!("unknown operator: {:?}", op),
    };
    Ok((ss, op))
}

// 乗除算
pub fn op2_parser(s: &str) -> IResult<&str, Op> {
    // dbg!("op_parser");
    let (ss, op) = one_of("*/")(s)?;
    let op = match op {
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => panic!("unknown operator: {:?}", op),
    };
    Ok((ss, op))
}

pub fn paren_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((char('('), expr_parser, char(')'))),
        |(_, expr, _)| expr,
    )(s)
}

pub fn factor_parser(s: &str) -> IResult<&str, Expr> {
    alt((const_parser, paren_expr_parser))(s)
}

pub fn term_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((factor_parser, opt(tuple((op2_parser, term_parser))))),
        |(fac, a)| {
            if let Some((op2, term)) = a {
                Expr::BinOp(Box::new(BinOp::new(fac, op2, term)))
            } else {
                fac
            }
        },
    )(s)
}

pub fn expr_parser(s: &str) -> IResult<&str, Expr> {
    // dbg!("expr_parser");
    map(
        tuple((term_parser, opt(tuple((op1_parser, expr_parser))))),
        |(term, a)| {
            if let Some((op1, expr)) = a {
                Expr::BinOp(Box::new(BinOp::new(term, op1, expr)))
            } else {
                term
            }
        },
    )(s)
}

pub fn program_parser(s: &str) -> Expr {
    let (ss, ast) = expr_parser(s).unwrap();
    assert_eq!(ss, "", "program parser must consume all string");
    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let code = "4";
        let result = expr_parser(code);
        assert_eq!(result, Ok(("", Expr::Const(Const::new(4)))));
    }

    #[test]
    fn test_binop_add() {
        let codes: Vec<&str> = vec!["1+2", "1   +2", "1 + 2"];
        let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
            Expr::Const(Const::new(1)),
            Op::Add,
            Expr::Const(Const::new(2)),
        )));

        for code in codes {
            let result = expr_parser(code);
            // dbg!(&expect_expr);
            assert_eq!(
                result,
                Ok(("", expect_expr.clone())),
                "\n=== code: {} ===\n",
                code
            );
        }
    }

    #[test]
    fn test_binop2() {
        let codes: Vec<&str> = vec!["1 + 2 + 3"];
        let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
            Expr::Const(Const::new(1)), // 1
            Op::Add,                    // +
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new(2)),
                    Op::Add,
                    Expr::Const(Const::new(3)),
                ), // 2 + 3
            )),
        )));

        for code in codes {
            let result = expr_parser(code);
            // dbg!(&expect_expr);
            assert_eq!(
                result,
                Ok(("", expect_expr.clone())),
                "\n=== code: {} ===\n",
                code
            );
        }
    }
    #[test]
    fn test_binop3() {
        let codes: Vec<&str> = vec!["1 * 2 + 3"];
        let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new(1)),
                    Op::Mul,
                    Expr::Const(Const::new(2)),
                ), // 1 * 2
            )),
            Op::Add,                    // +
            Expr::Const(Const::new(3)), // 3
        )));

        for code in codes {
            let result = expr_parser(code);
            // dbg!(&expect_expr);
            assert_eq!(
                result,
                Ok(("", expect_expr.clone())),
                "\n=== code: {} ===\n",
                code
            );
        }
    }
}
