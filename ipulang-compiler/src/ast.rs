use std::str::FromStr;

use nom::{
    branch::alt,
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, multispace1, one_of},
    combinator::{cut, map, opt},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::nodes::{
    Assing, BinOp, Call, Const, Expr, FunctionDecl, Op, Program, Stmt, Stmts, Variable,
    VariableDecl,
};

/// "64hoge" -> hoge, <64>
pub fn const_parser(s: &str) -> IResult<&str, Expr> {
    // dbg!("const_parser");
    let (ss, lit) = digit1(s)?;
    let val = FromStr::from_str(lit).unwrap();
    Ok((ss, Expr::Const(Const::new(val))))
}

// 変数名
pub fn var_name_parser(s: &str) -> IResult<&str, String> {
    let (s, (a, b)) = tuple((alpha1, alphanumeric0))(s)?;
    Ok((s, a.to_owned() + b))
}

// 変数
pub fn var_parser(s: &str) -> IResult<&str, Variable> {
    let (s, name) = var_name_parser(s)?;
    Ok((s, Variable::new(name.to_owned())))
}

// 変数宣言
pub fn var_decl_parser(s: &str) -> IResult<&str, VariableDecl> {
    let (s, (_, _, name, opt_init, _)) = tuple((
        tag("var"),
        multispace1,
        var_name_parser,
        map(
            opt(tuple((
                delimited(multispace0, char('='), multispace0),
                expr_parser,
            ))),
            |opt| opt.map(|a| a.1),
        ),
        char(';'),
    ))(s)?;
    Ok((s, VariableDecl::new(name.to_owned(), opt_init)))
}

// 加減算
pub fn op1_parser(s: &str) -> IResult<&str, Op> {
    // dbg!("op_parser");
    let (ss, op) = delimited(multispace0, one_of("+-"), multispace0)(s)?;
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
    let (ss, op) = delimited(multispace0, one_of("*/"), multispace0)(s)?;
    let op = match op {
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => panic!("unknown operator: {:?}", op),
    };
    Ok((ss, op))
}

pub fn paren_expr_parser(s: &str) -> IResult<&str, Expr> {
    delimited(char('('), expr_parser, char(')'))(s)
}

pub fn call_parser(s: &str) -> IResult<&str, Call> {
    map(
        tuple((
            var_name_parser,
            delimited(
                terminated(char('('), multispace0),
                separated_list0(delimited(multispace0, char(','), multispace0), expr_parser),
                preceded(multispace0, char(')')),
            ),
        )),
        |(name, args)| Call::new(name, args),
    )(s)
}

pub fn factor_parser(s: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            const_parser,
            paren_expr_parser,
            map(call_parser, |call| Expr::Call(call)),
            map(var_parser, |var| Expr::Variable(var)),
        )),
        multispace0,
    )(s)
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

pub fn assign_parser(s: &str) -> IResult<&str, Assing> {
    let (s, (id, _, expr)) = tuple((
        var_name_parser,
        delimited(multispace0, char('='), multispace0),
        terminated(expr_parser, terminated(multispace0, char(';'))),
    ))(s)?;
    Ok((s, Assing::new(id, expr)))
}

pub fn return_parser(s: &str) -> IResult<&str, Expr> {
    let (s, (_, _, expr, _)) = tuple((tag("return"), multispace1, expr_parser, char(';')))(s)?;
    Ok((s, expr))
}

pub fn stmt_parser(s: &str) -> IResult<&str, Stmt> {
    delimited(
        multispace0,
        alt((
            map(var_decl_parser, |v| Stmt::VariableDecl(v)),
            map(return_parser, |r| Stmt::Return(r)),
            map(assign_parser, |a| Stmt::Assing(a)),
            map(
                tuple((expr_parser, multispace0, char(';'))),
                |(expr, _, _)| Stmt::Expr(expr),
            ),
        )),
        multispace0,
    )(s)
}

pub fn stmts_parser(s: &str) -> IResult<&str, Stmts> {
    let (s, stmts) = many0(stmt_parser)(s)?;
    Ok((s, Stmts::new(stmts)))
}

// pub fn trim_parser(s: &str, keyword: &str) -> IResult<&str, String> {
//     delimited(multispace0, tag(keyword), multispace0)(s)
// }

// 関数宣言
// TODO: みづらい
pub fn function_decl_parser(s: &str) -> IResult<&str, FunctionDecl> {
    let (s, (name, params, stmts)) = tuple((
        preceded(tuple((tag("fn"), multispace1)), var_name_parser),
        preceded(
            tuple((char('('), multispace0)),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                var_name_parser,
            ),
        ),
        delimited(
            tuple((multispace0, char(')'), multispace0, char('{'), multispace0)),
            stmts_parser,
            tuple((multispace0, char('}'))),
        ),
    ))(s)?;
    Ok((s, FunctionDecl::new(name.to_owned(), params, stmts)))
}

pub fn program_parser(s: &str) -> Program {
    let (ss, ast) = many1(delimited(multispace0, function_decl_parser, multispace0))(s).unwrap();
    assert_eq!(ss, "", "program parser must consume all string");
    Program::new(ast)
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

    #[test]
    fn test_vardecl1() {
        let codes: Vec<&str> = vec!["var a;", "var   a;"];
        let expect_expr: VariableDecl = VariableDecl::new("a".to_owned(), None);
        for code in codes {
            let result = var_decl_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }
    #[test]
    fn test_vardecl2() {
        let codes: Vec<&str> = vec!["var ababaAFAF;", "var   ababaAFAF;"];
        let expect_expr: VariableDecl = VariableDecl::new("ababaAFAF".to_owned(), None);
        for code in codes {
            let result = var_decl_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }

    #[test]
    fn test_var() {
        let codes: Vec<&str> = vec!["a", "A", "Ab", "a1", "A123"];

        for code in codes {
            let result = var_parser(code);
            assert_eq!(result, Ok(("", Variable::new(code.to_string()))));
        }
    }
    #[test]
    fn test_stmt1() {
        let codes: Vec<&str> = vec!["1 * 2 + 3;", "1 * 2 + 3   ;"];

        let expect_expr: Stmt = Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new(1)),
                    Op::Mul,
                    Expr::Const(Const::new(2)),
                ), // 1 * 2
            )),
            Op::Add,                    // +
            Expr::Const(Const::new(3)), // 3
        ))));

        for code in codes {
            let result = stmt_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }

    #[test]
    fn test_stmts1() {
        let codes: Vec<&str> = vec!["1; 2;", "var a; var b;", "var a; 1 * 2 + 3;", "1 + a;"];
        let exprs: Vec<Stmts> = vec![
            Stmts::new(vec![
                Stmt::Expr(Expr::Const(Const::new(1))),
                Stmt::Expr(Expr::Const(Const::new(2))),
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new("a".to_owned(), None)), // var a;
                Stmt::VariableDecl(VariableDecl::new("b".to_owned(), None)), // var b;
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new("a".to_owned(), None)), // var a;
                Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                    Expr::BinOp(Box::new(
                        BinOp::new(
                            Expr::Const(Const::new(1)),
                            Op::Mul,
                            Expr::Const(Const::new(2)),
                        ), // 1 * 2
                    )),
                    Op::Add,                    // +
                    Expr::Const(Const::new(3)), // 3
                )))),
            ]),
            Stmts::new(vec![Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                Expr::Const(Const::new(1)),
                Op::Add,
                Expr::Variable(Variable::new("a".to_owned())),
            ))))]),
        ];

        for (code, expr) in codes.into_iter().zip(exprs.into_iter()) {
            let result = stmts_parser(code);
            assert_eq!(result, Ok(("", expr)));
        }
    }

    #[test]
    fn stmt_test() {
        let codes: Vec<&str> = vec![
            "1;",
            "var a;",
            "var a = 1;",
            "var a = 1 + 2;",
            "a;",
            "f();",
            "f( ) + 1;",
        ];
        for code in codes {
            let (rest, _) = stmt_parser(code).unwrap();
            assert_eq!(rest, "", "\n=== code: {} ===", code);
        }
    }

    #[test]
    fn test_fn1() {
        let code = "fn main( ) { }";
        let expect = FunctionDecl::new("main".to_owned(), vec![], Stmts::new(vec![]));
        let result = function_decl_parser(code);
        assert_eq!(result, Ok(("", expect)));
    }

    #[test]
    fn test_fn2() {
        let code = "fn  h0Ge() { 1 ; }";
        let expect = FunctionDecl::new(
            "h0Ge".to_owned(),
            vec![],
            Stmts::new(vec![Stmt::Expr(Expr::Const(Const::new(1)))]),
        );
        let result = function_decl_parser(code);
        assert_eq!(result, Ok(("", expect)));
    }

    #[test]
    fn call_test() {
        let codes: Vec<&str> = vec![
            "a()",
            "a( )",
            "a(1)",
            "a( 1, 2 , 3 )",
            "a(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)",
        ];
        for code in codes {
            let (rest, _) = call_parser(code).unwrap();
            assert_eq!(rest, "");
        }
    }

    #[test]
    fn test_fn_decl() {
        let codes: Vec<&str> = vec!["fn a() { }", "fn a() { 1; }", "fn a(){}", "fn a( ) { }"];

        for code in codes {
            let (rest, _) = function_decl_parser(code).unwrap();
            assert_eq!(rest, "", "=== code: {} ===\n", code);
        }
    }

    #[test]
    fn test_fns() {
        let codes: Vec<&str> = vec!["fn a() { } fn main() {a();}"];

        let expect = Program::new(vec![
            FunctionDecl::new("a".to_owned(), vec![], Stmts::new(vec![])),
            FunctionDecl::new(
                "main".to_owned(),
                vec![],
                Stmts::new(vec![Stmt::Expr(Expr::Call(Call::new(
                    "a".to_string(),
                    vec![],
                )))]),
            ),
        ]);
        for code in codes {
            let rest = program_parser(code);
            assert_eq!(rest, expect);
        }
    }
}
