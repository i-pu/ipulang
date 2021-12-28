use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, opt},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::nodes::{
    Assign, BinOp, Call, Const, Expr, For, FunctionDecl, IfElse, Op, Program, Stmt, Stmts,
    Variable, VariableDecl,
};

use crate::types::Type;

pub fn type_parser(s: &str) -> IResult<&str, Type> {
    alt((
        map(tag("i32"), |_| Type::Int32),
        map(tag("i64"), |_| Type::Int64),
        map(tag("bool"), |_| Type::Bool),
        map(tag("string"), |_| Type::String),
        map(tag("unit"), |_| Type::Unit),
    ))(s)
}

/// "64hoge" -> hoge, <64>
pub fn const_parser(s: &str) -> IResult<&str, Const> {
    alt((
        map(
            tuple((digit1, char('_'), type_parser)),
            |(n, _, t)| match t {
                Type::Int32 => Const::I32Const(n.parse::<i32>().unwrap()),
                Type::Int64 => Const::I64Const(n.parse::<i64>().unwrap()),
                Type::Bool => Const::BoolConst(n.parse::<i32>().unwrap() != 0),
                _ => panic!("unsupported type"),
            },
        ),
        // リテラルを書かない時はi32
        map(digit1, |s: &str| Const::I32Const(s.parse::<i32>().unwrap())),
    ))(s)
}

// 変数名
pub fn var_name_parser(s: &str) -> IResult<&str, String> {
    let (s, (a, b)) = tuple((alpha1, alphanumeric0))(s)?;
    Ok((s, a.to_owned() + b))
}

// 変数
pub fn var_parser(s: &str) -> IResult<&str, Variable> {
    let (s, name) = var_name_parser(s)?;
    Ok((s, Variable::new(name.to_owned(), Type::Unknown)))
}

// 変数宣言
pub fn var_decl_parser(s: &str) -> IResult<&str, VariableDecl> {
    let (s, (_, _, name, _, _, _, typ, _, opt_init, _)) = tuple((
        tag("var"),
        multispace1,
        var_name_parser,
        // type annotation
        multispace0,
        char(':'),
        multispace0,
        type_parser,
        multispace0,
        map(
            opt(tuple((
                delimited(multispace0, char('='), multispace0),
                or_expr_parser,
            ))),
            |opt| opt.map(|a| a.1),
        ),
        char(';'),
    ))(s)?;
    Ok((s, VariableDecl::new(name.to_owned(), typ, opt_init)))
}

pub fn paren_expr_parser(s: &str) -> IResult<&str, Expr> {
    delimited(char('('), or_expr_parser, char(')'))(s)
}

pub fn call_parser(s: &str) -> IResult<&str, Call> {
    map(
        tuple((
            var_name_parser,
            delimited(
                terminated(char('('), multispace0),
                separated_list0(
                    delimited(multispace0, char(','), multispace0),
                    or_expr_parser,
                ),
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
            map(const_parser, |c| Expr::Const(c)),
            paren_expr_parser,
            map(call_parser, |call| Expr::Call(call)),
            map(var_parser, |var| Expr::Variable(var)),
        )),
        multispace0,
    )(s)
}

pub fn multiplicative_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            factor_parser,
            opt(tuple((
                map(
                    delimited(multispace0, one_of("*/%"), multispace0),
                    |c| match c {
                        '*' => Op::Mul,
                        '/' => Op::Div,
                        '%' => Op::Mod,
                        _ => panic!("unknown operator: {:?}", c),
                    },
                ),
                multiplicative_expr_parser,
            ))),
        )),
        |(factor, option)| {
            if let Some((op, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(factor, op, expr, Type::Unknown)))
            } else {
                factor
            }
        },
    )(s)
}

pub fn additive_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            multiplicative_expr_parser,
            opt(tuple((
                map(
                    delimited(multispace0, one_of("+-"), multispace0),
                    |c| match c {
                        '+' => Op::Add,
                        '-' => Op::Sub,
                        _ => panic!("unknown operator: {:?}", c),
                    },
                ),
                additive_expr_parser,
            ))),
        )),
        |(multi, option)| {
            if let Some((op, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(multi, op, expr, Type::Unknown)))
            } else {
                multi
            }
        },
    )(s)
}

pub fn releational_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            additive_expr_parser,
            opt(tuple((
                map(
                    delimited(
                        multispace0,
                        alt((tag(">="), tag("<="), tag(">"), tag("<"))),
                        multispace0,
                    ),
                    |c| match c {
                        ">=" => Op::Geq,
                        "<=" => Op::Leq,
                        ">" => Op::Gt,
                        "<" => Op::Lt,
                        _ => panic!("unknown operator: {:?}", c),
                    },
                ),
                releational_expr_parser,
            ))),
        )),
        |(add, option)| {
            if let Some((op, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(add, op, expr, Type::Unknown)))
            } else {
                add
            }
        },
    )(s)
}
pub fn equality_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            releational_expr_parser,
            opt(tuple((
                map(
                    delimited(multispace0, alt((tag("=="), tag("!="))), multispace0),
                    |c| match c {
                        "==" => Op::Eq,
                        "!=" => Op::Neq,
                        _ => panic!("unknown operator: {:?}", c),
                    },
                ),
                equality_expr_parser,
            ))),
        )),
        |(rel, option)| {
            if let Some((op, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(rel, op, expr, Type::Unknown)))
            } else {
                rel
            }
        },
    )(s)
}

pub fn and_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            equality_expr_parser,
            opt(tuple((
                delimited(multispace0, tag("&&"), multispace0),
                and_expr_parser,
            ))),
        )),
        |(rel, option)| {
            if let Some((_, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(rel, Op::And, expr, Type::Unknown)))
            } else {
                rel
            }
        },
    )(s)
}

pub fn or_expr_parser(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            and_expr_parser,
            opt(tuple((
                delimited(multispace0, tag("||"), multispace0),
                or_expr_parser,
            ))),
        )),
        |(rel, option)| {
            if let Some((_, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(rel, Op::Or, expr, Type::Unknown)))
            } else {
                rel
            }
        },
    )(s)
}

pub fn assign_parser(s: &str) -> IResult<&str, Assign> {
    let (s, (id, _, expr)) = tuple((
        var_name_parser,
        delimited(multispace0, char('='), multispace0),
        terminated(or_expr_parser, terminated(multispace0, char(';'))),
    ))(s)?;
    Ok((s, Assign::new(id, expr)))
}

pub fn return_parser(s: &str) -> IResult<&str, Expr> {
    let (s, (_, _, expr, _)) = tuple((tag("return"), multispace1, or_expr_parser, char(';')))(s)?;
    Ok((s, expr))
}

pub fn if_else_parser(s: &str) -> IResult<&str, IfElse> {
    map(
        tuple((
            tag("if"),
            multispace0,
            // cond
            delimited(
                char('('),
                delimited(multispace0, or_expr_parser, multispace0),
                char(')'),
            ),
            // success
            delimited(
                multispace0,
                delimited(char('{'), stmts_parser, char('}')),
                multispace0,
            ),
            opt(tag("else")),
            // failure
            opt(delimited(
                multispace0,
                delimited(char('{'), stmts_parser, char('}')),
                multispace0,
            )),
        )),
        |(_, _, cond, sucess, _, failure)| IfElse::new(cond, sucess, failure),
    )(s)
}
pub fn for_parser(s: &str) -> IResult<&str, For> {
    map(
        tuple((
            tag("for"),
            multispace0,
            // var_decl, cond, assign
            delimited(
                char('('),
                tuple((
                    delimited(multispace0, var_decl_parser, multispace0),
                    delimited(multispace0, or_expr_parser, multispace0),
                    delimited(multispace0, char(';'), multispace0),
                    delimited(multispace0, assign_parser, multispace0),
                )),
                char(')'),
            ),
            delimited(
                multispace0,
                delimited(char('{'), stmts_parser, char('}')),
                multispace0,
            ),
        )),
        |(_, _, (var_decl, cond, _, assign), stmts)| For::new(var_decl, cond, assign, stmts),
    )(s)
}

pub fn stmt_parser(s: &str) -> IResult<&str, Stmt> {
    delimited(
        multispace0,
        alt((
            map(var_decl_parser, |v| Stmt::VariableDecl(v)),
            map(return_parser, |r| Stmt::Return(r)),
            map(assign_parser, |a| Stmt::Assign(a)),
            map(if_else_parser, |i| Stmt::IfElse(i)),
            map(for_parser, |i| Stmt::For(i)),
            map(
                tuple((or_expr_parser, multispace0, char(';'))),
                |(expr, _, _)| Stmt::Expr(expr),
            ),
        )),
        multispace0,
    )(s)
}

pub fn stmts_parser(s: &str) -> IResult<&str, Stmts> {
    let (s, stmts) = delimited(multispace0, many0(stmt_parser), multispace0)(s)?;
    Ok((s, Stmts::new(stmts)))
}

// pub fn trim_parser(s: &str, keyword: &str) -> IResult<&str, String> {
//     delimited(multispace0, tag(keyword), multispace0)(s)
// }

pub fn function_parameters_parser(s: &str) -> IResult<&str, Vec<Variable>> {
    delimited(
        char('('),
        delimited(
            multispace0,
            many0(map(tuple((var_name_parser, type_parser)), |(id, typ)| {
                Variable::new(id, typ)
            })),
            multispace0,
        ),
        char(')'),
    )(s)
}

// 関数宣言
// TODO: みづらい
pub fn function_decl_parser(s: &str) -> IResult<&str, FunctionDecl> {
    let (s, (_, _, name, params, _, _, _, typ, _, stmts)) = tuple((
        tag("fn"),
        multispace1,
        var_name_parser,
        function_parameters_parser,
        multispace0,
        char(':'),
        multispace0,
        type_parser,
        multispace0,
        delimited(
            multispace0,
            delimited(char('{'), stmts_parser, char('}')),
            multispace0,
        ),
    ))(s)?;
    Ok((s, FunctionDecl::new(name.to_owned(), params, typ, stmts)))
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
        let codes: Vec<&str> = vec!["4", "20", "0_i32", "10_i64", "0_bool", "1_bool"];
        for code in codes {
            let (res, _) = const_parser(code).unwrap();
            assert_eq!(res, "", "\n=== code: {} ===\n", code);
        }
    }

    #[test]
    fn test_binop_add() {
        let codes: Vec<&str> = vec!["1+2", "1   +2", "1 + 2"];
        let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
            Expr::Const(Const::new_i32(1)),
            Op::Add,
            Expr::Const(Const::new_i32(2)),
            Type::Unknown,
        )));

        for code in codes {
            let result = or_expr_parser(code);
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
            Expr::Const(Const::new_i32(1)), // 1
            Op::Add,                        // +
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new_i32(2)),
                    Op::Add,
                    Expr::Const(Const::new_i32(3)),
                    Type::Unknown,
                ), // 2 + 3
            )),
            Type::Unknown,
        )));

        for code in codes {
            let result = or_expr_parser(code);
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
                    Expr::Const(Const::new_i32(1)),
                    Op::Mul,
                    Expr::Const(Const::new_i32(2)),
                    Type::Unknown,
                ), // 1 * 2
            )),
            Op::Add,                        // +
            Expr::Const(Const::new_i32(3)), // 3
            Type::Unknown,
        )));

        for code in codes {
            let result = or_expr_parser(code);
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
    fn test_binop4() {
        let codes: Vec<&str> = vec!["1 >= 2 == 3"];
        let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new_i32(1)),
                    Op::Geq,
                    Expr::Const(Const::new_i32(2)),
                    Type::Unknown,
                ), // 1 >= 2
            )),
            Op::Eq,                         // ==
            Expr::Const(Const::new_i32(3)), // 3
            Type::Unknown,
        )));

        for code in codes {
            let result = or_expr_parser(code);
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
    fn test_binops() {
        let codes: Vec<&str> = vec![
            "1 >= 2 == 3",
            "1 <= 3 + 2 * 4",
            "(1 < 3 * 4 + 2) || 3",
            "3 == 4 + 5",
            "3 > 4",
            "1 || 3 && 4",
        ];

        for code in codes {
            let (s, expr) = or_expr_parser(code).unwrap();
            assert_eq!(
                s, "",
                "\n=== code: {} ===\n=== expr: {:?} ===\n",
                code, expr,
            );
        }
    }

    #[test]
    fn test_vardecl1() {
        let codes: Vec<&str> = vec!["var a: i32;", "var   a : i32;"];
        let expect_expr: VariableDecl = VariableDecl::new("a".to_owned(), Type::Int32, None);
        for code in codes {
            let result = var_decl_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }
    #[test]
    fn test_vardecl2() {
        let codes: Vec<&str> = vec!["var ababaAFAF: i32 ;", "var   ababaAFAF: i32;"];
        let expect_expr: VariableDecl =
            VariableDecl::new("ababaAFAF".to_owned(), Type::Int32, None);
        for code in codes {
            let result = var_decl_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }

    #[test]
    fn test_var() {
        let codes: Vec<&str> = vec!["a", "A", "Ab", "a1", "A123"];
        for code in codes {
            let (rest, _) = var_parser(code).unwrap();
            assert_eq!(rest, "", "\n=== code: {} ===", code);
        }
    }

    #[test]
    fn test_stmt1() {
        let codes: Vec<&str> = vec!["1 * 2 + 3;", "1 * 2 + 3   ;"];

        let expect_expr: Stmt = Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
            Expr::BinOp(Box::new(
                BinOp::new(
                    Expr::Const(Const::new_i32(1)),
                    Op::Mul,
                    Expr::Const(Const::new_i32(2)),
                    Type::Unknown,
                ), // 1 * 2
            )),
            Op::Add,                        // +
            Expr::Const(Const::new_i32(3)), // 3
            Type::Unknown,
        ))));

        for code in codes {
            let result = stmt_parser(code);
            assert_eq!(result, Ok(("", expect_expr.clone())));
        }
    }

    #[test]
    fn test_stmts1() {
        let codes: Vec<&str> = vec![
            "1; 2;",
            "var a: i32; var b: i32;",
            "var a: i32; 1 * 2 + 3;",
            "1 + a;",
        ];
        let exprs: Vec<Stmts> = vec![
            Stmts::new(vec![
                Stmt::Expr(Expr::Const(Const::new_i32(1))),
                Stmt::Expr(Expr::Const(Const::new_i32(2))),
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new("a".to_owned(), Type::Int32, None)), // var a;
                Stmt::VariableDecl(VariableDecl::new("b".to_owned(), Type::Int32, None)), // var b;
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new("a".to_owned(), Type::Int32, None)), // var a;
                Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                    Expr::BinOp(Box::new(
                        BinOp::new(
                            Expr::Const(Const::new_i32(1)),
                            Op::Mul,
                            Expr::Const(Const::new_i32(2)),
                            Type::Unknown,
                        ), // 1 * 2
                    )),
                    Op::Add,                        // +
                    Expr::Const(Const::new_i32(3)), // 3
                    Type::Unknown,
                )))),
            ]),
            Stmts::new(vec![Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                Expr::Const(Const::new_i32(1)),
                Op::Add,
                Expr::Variable(Variable::new("a".to_owned(), Type::Unknown)),
                Type::Unknown,
            ))))]),
        ];

        for (code, expr) in codes.into_iter().zip(exprs.into_iter()) {
            let result = stmts_parser(code);
            assert_eq!(result, Ok(("", expr)));
        }
    }

    #[test]
    fn test_stmt() {
        let codes: Vec<&str> = vec![
            "1;",
            "var a: i32;",
            "var a: i32 = 1;",
            "var a: i32 = 1 + 2;",
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
        let code = "fn main( ): unit { }";
        let expect = FunctionDecl::new("main".to_owned(), vec![], Type::Unit, Stmts::new(vec![]));
        let result = function_decl_parser(code);
        assert_eq!(result, Ok(("", expect)));
    }

    #[test]
    fn test_fn2() {
        let code = "fn  h0Ge() : unit { 1 ; }";
        let expect = FunctionDecl::new(
            "h0Ge".to_owned(),
            vec![],
            Type::Unit,
            Stmts::new(vec![Stmt::Expr(Expr::Const(Const::new_i32(1)))]),
        );
        let result = function_decl_parser(code);
        assert_eq!(result, Ok(("", expect)));
    }

    #[test]
    fn test_call() {
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
        let codes: Vec<&str> = vec![
            "fn a() : unit { }",
            "fn a() : unit { 1; }",
            "fn a():unit {}",
            "fn a( ) : unit { }",
        ];

        for code in codes {
            let (rest, _) = function_decl_parser(code).unwrap();
            assert_eq!(rest, "", "=== code: {} ===\n", code);
        }
    }

    #[test]
    fn test_fns() {
        let codes: Vec<&str> = vec!["fn a() : unit { } fn main(): i32 {a();}"];

        let expect = Program::new(vec![
            FunctionDecl::new("a".to_owned(), vec![], Type::Unit, Stmts::new(vec![])),
            FunctionDecl::new(
                "main".to_owned(),
                vec![],
                Type::Int32,
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

    #[test]
    fn test_if_else() {
        let codes: Vec<&str> = vec![
            "if(1) { }",
            "if(1) { 0; } else { 0; }",
            "if(1){ 1; } else { if(2){} else {} }",
            "if (1) { if (2) { 0; } } else { if(0) { 0; }}",
            "if (1 > 1 && 3 || 3 + 4 > 4 * 5 || 3) { if (2) {} } else { if(0) {}}",
        ];
        for code in codes {
            let (rest, _) = if_else_parser(code).unwrap();
            assert_eq!(rest, "", "\n=== code: {} ===\n", code);
        }
    }
    #[test]
    fn test_for() {
        let codes: Vec<&str> = vec![
            "for(var i: i32 ;i < 10; i = i + 1;){}",
            "for (var i : i32 ;i < 10; i = i + 1;){ return 0;}",
            r#"for (var i  : i32 ; i < 10; i = i + 1;) {
                a = a + i;
            }"#,
        ];
        for code in codes {
            let (rest, _) = for_parser(code).unwrap();
            assert_eq!(rest, "", "\n=== code: {} ===\n", code);
        }
    }
}
