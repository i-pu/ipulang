use crate::nodes::{
    Assign, BinOp, Call, Const, Expr, For, FunctionDecl, IfElse, Op, Program, Span, Stmt, Stmts,
    Variable, VariableDecl,
};
use crate::types::Type;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, opt},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use nom_locate::position;

pub fn type_parser(s: Span) -> IResult<Span, Type> {
    alt((
        map(tag("i32"), |_| Type::Int32),
        map(tag("i64"), |_| Type::Int64),
        map(tag("bool"), |_| Type::Bool),
        map(tag("string"), |_| Type::String),
        map(tag("unit"), |_| Type::Unit),
    ))(s)
}

/// "64hoge" -> hoge, <64>
pub fn const_parser(s: Span) -> IResult<Span, Const> {
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
        map(digit1, |s: Span| Const::I32Const(s.parse::<i32>().unwrap())),
    ))(s)
}

/// 変数名
pub fn var_name_parser<'a>(s: Span<'a>) -> IResult<Span, (Span, String)> {
    let (s, (a, b)) = tuple((alpha1, alphanumeric0))(s)?;
    let var_name = a.fragment().to_string() + b.fragment();
    Ok((s, (s, var_name)))
}

/// 変数
pub fn var_parser(s: Span) -> IResult<Span, Variable> {
    let (s, name) = var_name_parser(s)?;
    let (_, pos) = position(s)?;
    Ok((s, Variable::new(pos, name.1, Type::Unknown)))
}

/// 変数宣言
pub fn var_decl_parser(s: Span) -> IResult<Span, VariableDecl> {
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
    let (s, pos) = position(s)?;
    Ok((s, VariableDecl::new(pos, name.1, typ, opt_init)))
}

pub fn paren_expr_parser(s: Span) -> IResult<Span, Expr> {
    delimited(char('('), or_expr_parser, char(')'))(s)
}

pub fn call_parser(s: Span) -> IResult<Span, Call> {
    let (s, ((_, name), args)) = tuple((
        var_name_parser,
        delimited(
            terminated(char('('), multispace0),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                or_expr_parser,
            ),
            preceded(multispace0, char(')')),
        ),
    ))(s)?;
    let (s, pos) = position(s)?;
    Ok((s, Call::new(pos, name, args)))
}

pub fn factor_parser(s: Span) -> IResult<Span, Expr> {
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

/// `factor` (*/% `multiplicative_expr`)
pub fn multiplicative_expr_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (factor, option)) = tuple((
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
    ))(s)?;
    let (s, pos) = position(s)?;
    if let Some((op, expr)) = option {
        Ok((
            s,
            Expr::BinOp(Box::new(BinOp::new(pos, factor, op, expr, Type::Unknown))),
        ))
    } else {
        Ok((s, factor))
    }
}

/// `multiplicative_expr` (+ `additive_expr`)
pub fn additive_expr_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (multi, option)) = tuple((
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
    ))(s)?;
    let (s, pos) = position(s)?;
    if let Some((op, expr)) = option {
        Ok((
            s,
            Expr::BinOp(Box::new(BinOp::new(pos, multi, op, expr, Type::Unknown))),
        ))
    } else {
        Ok((s, multi))
    }
}

/// `>=`, `<=`, `>`, `<`
pub fn relational_op_parser(s: Span) -> IResult<Span, Op> {
    let (s, c) = delimited(
        multispace0,
        alt((tag(">="), tag("<="), tag(">"), tag("<"))),
        multispace0,
    )(s)?;
    let (s, pos) = position(s)?;
    let op = match *c.fragment() {
        ">=" => Op::Geq,
        "<=" => Op::Leq,
        ">" => Op::Gt,
        "<" => Op::Lt,
        _ => panic!("unknown operator: {:?}", c),
    };
    Ok((s, op))
}

/// `additive_expr` (== `relational_expr`)
pub fn releational_expr_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (add, option)) = tuple((
        additive_expr_parser,
        opt(tuple((relational_op_parser, releational_expr_parser))),
    ))(s)?;
    let (s, pos) = position(s)?;
    if let Some((op, expr)) = option {
        Ok((
            s,
            Expr::BinOp(Box::new(BinOp::new(pos, add, op, expr, Type::Unknown))),
        ))
    } else {
        Ok((s, add))
    }
}

/// `==`, `!=`
pub fn equality_op_parser(s: Span) -> IResult<Span, Op> {
    let (s, c) = delimited(multispace0, alt((tag("=="), tag("!="))), multispace0)(s)?;
    let (s, pos) = position(s)?;
    let op = match *c.fragment() {
        "==" => Op::Eq,
        "!=" => Op::Neq,
        _ => panic!("unknown operator: {:?}", c),
    };
    Ok((s, op))
}

/// `relational_expr` (== `equality_expr`)
pub fn equality_expr_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (rel, option)) = tuple((
        releational_expr_parser,
        opt(tuple((equality_op_parser, equality_expr_parser))),
    ))(s)?;
    let (s, pos) = position(s)?;
    if let Some((op, expr)) = option {
        Ok((
            s,
            Expr::BinOp(Box::new(BinOp::new(pos, rel, op, expr, Type::Unknown))),
        ))
    } else {
        Ok((s, rel))
    }
}

/// `equality` (&& `and_expr`)
pub fn and_expr_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (rel, option)) = tuple((
        equality_expr_parser,
        opt(tuple((
            delimited(multispace0, tag("&&"), multispace0),
            and_expr_parser,
        ))),
    ))(s)?;
    let (s, pos) = position(s)?;
    if let Some((_, expr)) = option {
        Ok((
            s,
            Expr::BinOp(Box::new(BinOp::new(pos, rel, Op::And, expr, Type::Unknown))),
        ))
    } else {
        Ok((s, rel))
    }
}

/// `and_expr` (|| `or_expr`)
pub fn or_expr_parser(s: Span) -> IResult<Span, Expr> {
    map(
        tuple((
            and_expr_parser,
            opt(tuple((
                delimited(multispace0, tag("||"), multispace0),
                or_expr_parser,
            ))),
        )),
        |(rel, option)| {
            if let Some((pos, expr)) = option {
                Expr::BinOp(Box::new(BinOp::new(pos, rel, Op::Or, expr, Type::Unknown)))
            } else {
                rel
            }
        },
    )(s)
}

pub fn assign_parser(s: Span) -> IResult<Span, Assign> {
    map(
        tuple((
            var_name_parser,
            delimited(multispace0, char('='), multispace0),
            terminated(or_expr_parser, terminated(multispace0, char(';'))),
        )),
        |(id, _, expr)| Assign::new(id.0, id.1, expr),
    )(s)
}

pub fn return_parser(s: Span) -> IResult<Span, Expr> {
    let (s, (_, _, expr, _)) = tuple((tag("return"), multispace1, or_expr_parser, char(';')))(s)?;
    Ok((s, expr))
}

pub fn if_else_parser(s: Span) -> IResult<Span, IfElse> {
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
        |(tag, _, cond, sucess, _, failure)| IfElse::new(tag, cond, sucess, failure),
    )(s)
}
pub fn for_parser(s: Span) -> IResult<Span, For> {
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
        |(tag, _, (var_decl, cond, _, assign), stmts)| For::new(tag, var_decl, cond, assign, stmts),
    )(s)
}

pub fn stmt_parser(s: Span) -> IResult<Span, Stmt> {
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

pub fn stmts_parser(s: Span) -> IResult<Span, Stmts> {
    let (s, stmts) = delimited(multispace0, many0(stmt_parser), multispace0)(s)?;
    Ok((s, Stmts::new(stmts)))
}

// pub fn trim_parser(s: Span, keyword: Span) -> IResult<Span, String> {
//     delimited(multispace0, tag(keyword), multispace0)(s)
// }

pub fn function_parameters_parser(s: Span) -> IResult<Span, Vec<Variable>> {
    delimited(
        char('('),
        delimited(
            multispace0,
            separated_list0(
                tuple((multispace0, char(','), multispace0)),
                map(
                    tuple((
                        var_name_parser,
                        multispace0,
                        char(':'),
                        multispace0,
                        type_parser,
                    )),
                    |(id, _, _, _, typ)| Variable::new(id.0, id.1, typ),
                ),
            ),
            multispace0,
        ),
        char(')'),
    )(s)
}

// 関数宣言
pub fn function_decl_parser(s: Span) -> IResult<Span, FunctionDecl> {
    map(
        tuple((
            tag("fn"),
            multispace1,
            var_name_parser,
            multispace0,
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
        )),
        |(tag, _, name, _, params, _, _, _, typ, _, stmts)| {
            FunctionDecl::new(tag, name.1, params, typ, stmts)
        },
    )(s)
}

pub fn program_parser(s: Span) -> Program {
    let (ss, ast) = many1(delimited(multispace0, function_decl_parser, multispace0))(s).unwrap();
    assert_eq!(
        ss.fragment().to_string(),
        "",
        "program parser must consume all string"
    );
    Program::new(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_consumed(code: Span, res: Span) {
        assert_eq!(
            res.fragment().to_string(),
            "",
            "\n=== code: {} ===\n",
            code.fragment().to_string()
        );
    }

    #[test]
    fn test_const() {
        let codes: Vec<Span> = vec!["4", "20", "0_i32", "10_i64", "0_bool", "1_bool"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let (res, _) = const_parser(code).unwrap();
            check_consumed(code, res);
        }
    }

    #[test]
    fn test_binop_add() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["1+2", "1   +2", "1 + 2"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let (res, expr) = or_expr_parser(code).unwrap();
            let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
                IDK,
                Expr::Const(Const::new_i32(1)),
                Op::Add,
                Expr::Const(Const::new_i32(2)),
                Type::Unknown,
            )));
            // dbg!(&expect_expr);
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }

    #[test]
    fn test_binop2() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["1 + 2 + 3"]
            .iter()
            .map(|code| Span::new(code))
            .collect();
        for code in codes {
            let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
                IDK,
                Expr::Const(Const::new_i32(1)), // 1
                Op::Add,                        // +
                Expr::BinOp(Box::new(
                    BinOp::new(
                        IDK,
                        Expr::Const(Const::new_i32(2)),
                        Op::Add,
                        Expr::Const(Const::new_i32(3)),
                        Type::Unknown,
                    ), // 2 + 3
                )),
                Type::Unknown,
            )));
            let (res, expr) = or_expr_parser(code).unwrap();
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }
    #[test]
    fn test_binop3() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["1 * 2 + 3"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
                IDK,
                Expr::BinOp(Box::new(
                    BinOp::new(
                        IDK,
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
            let (res, expr) = or_expr_parser(code).unwrap();
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }

    #[test]
    fn test_binop4() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["1 >= 2 == 3"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let expect_expr: Expr = Expr::BinOp(Box::new(BinOp::new(
                IDK,
                Expr::BinOp(Box::new(
                    BinOp::new(
                        IDK,
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
            let (res, expr) = or_expr_parser(code).unwrap();
            // dbg!(&expect_expr);
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }

    #[test]
    fn test_binops() {
        let codes: Vec<Span> = vec![
            "1 >= 2 == 3",
            "1 <= 3 + 2 * 4",
            "(1 < 3 * 4 + 2) || 3",
            "3 == 4 + 5",
            "3 > 4",
            "1 || 3 && 4",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (s, _) = or_expr_parser(code).unwrap();
            check_consumed(code, s);
        }
    }

    #[test]
    fn test_vardecl1() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["var a: i32;", "var   a : i32;"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let expect_expr: VariableDecl =
                VariableDecl::new(IDK, "a".to_owned(), Type::Int32, None);
            let (res, expr) = var_decl_parser(code).unwrap();
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }
    #[test]
    fn test_vardecl2() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["var ababaAFAF: i32 ;", "var   ababaAFAF: i32;"]
            .iter()
            .map(|code| Span::new(code))
            .collect();
        for code in codes {
            let expect_expr: VariableDecl =
                VariableDecl::new(IDK, "ababaAFAF".to_owned(), Type::Int32, None);
            let (res, expr) = var_decl_parser(code).unwrap();
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }

    #[test]
    fn test_var() {
        let codes: Vec<Span> = vec!["a", "A", "Ab", "a1", "A123"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let (res, _) = var_parser(code).unwrap();
            check_consumed(code, res);
        }
    }

    #[test]
    fn test_stmt1() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["1 * 2 + 3;", "1 * 2 + 3   ;"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        for code in codes {
            let expect_expr: Stmt = Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                IDK,
                Expr::BinOp(Box::new(
                    BinOp::new(
                        IDK,
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

            let (res, expr) = stmt_parser(code).unwrap();
            check_consumed(code, res);
            assert_eq!(expect_expr, expr);
        }
    }

    #[test]
    fn test_stmts1() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec![
            "1; 2;",
            "var a: i32; var b: i32;",
            "var a: i32; 1 * 2 + 3;",
            "1 + a;",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();
        let exprs: Vec<Stmts> = vec![
            Stmts::new(vec![
                Stmt::Expr(Expr::Const(Const::new_i32(1))),
                Stmt::Expr(Expr::Const(Const::new_i32(2))),
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new(IDK, "a".to_owned(), Type::Int32, None)), // var a;
                Stmt::VariableDecl(VariableDecl::new(IDK, "b".to_owned(), Type::Int32, None)), // var b;
            ]),
            Stmts::new(vec![
                Stmt::VariableDecl(VariableDecl::new(IDK, "a".to_owned(), Type::Int32, None)), // var a;
                Stmt::Expr(Expr::BinOp(Box::new(BinOp::new(
                    IDK,
                    Expr::BinOp(Box::new(
                        BinOp::new(
                            IDK,
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
                IDK,
                Expr::Const(Const::new_i32(1)),
                Op::Add,
                Expr::Variable(Variable::new(Span::new("a"), "a".to_owned(), Type::Unknown)),
                Type::Unknown,
            ))))]),
        ];

        for (code, expr) in codes.into_iter().zip(exprs.into_iter()) {
            let (res, _) = stmts_parser(code).unwrap();
            check_consumed(code, res);
        }
    }

    #[test]
    fn test_stmt() {
        let codes: Vec<Span> = vec![
            "1;",
            "var a: i32;",
            "var a: i32 = 1;",
            "var a: i32 = 1 + 2;",
            "a;",
            "f();",
            "f( ) + 1;",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (rest, _) = stmt_parser(code).unwrap();
            check_consumed(code, rest);
        }
    }

    #[test]
    fn test_fn1() {
        let IDK = Span::new("");
        let code = Span::new("fn main( ): unit { }");
        let expect = FunctionDecl::new(
            IDK,
            "main".to_owned(),
            vec![],
            Type::Unit,
            Stmts::new(vec![]),
        );
        let (res, _) = function_decl_parser(code).unwrap();
        check_consumed(code, res);
    }

    #[test]
    fn test_fn2() {
        let IDK = Span::new("");
        let code = Span::new("fn  h0Ge() : unit { 1 ; }");
        let expect = FunctionDecl::new(
            IDK,
            "h0Ge".to_owned(),
            vec![],
            Type::Unit,
            Stmts::new(vec![Stmt::Expr(Expr::Const(Const::new_i32(1)))]),
        );
        let (res, _) = function_decl_parser(code).unwrap();
        check_consumed(code, res);
    }

    #[test]
    fn test_call() {
        let codes: Vec<Span> = vec![
            "a()",
            "a( )",
            "a(1)",
            "a( 1, 2 , 3 )",
            "a(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (rest, _) = call_parser(code).unwrap();
            check_consumed(code, rest);
        }
    }

    #[test]
    fn test_fn_decl() {
        let codes: Vec<Span> = vec![
            "fn a() : unit { }",
            "fn a() : unit { 1; }",
            "fn a():unit {}",
            "fn a( ) : unit { }",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (rest, _) = function_decl_parser(code).unwrap();
            check_consumed(code, rest);
        }
    }

    #[test]
    fn test_fns() {
        let IDK = Span::new("");
        let codes: Vec<Span> = vec!["fn a() : unit { } fn main(): i32 {a();}"]
            .iter()
            .map(|code| Span::new(code))
            .collect();

        let expect = Program::new(vec![
            FunctionDecl::new(IDK, "a".to_owned(), vec![], Type::Unit, Stmts::new(vec![])),
            FunctionDecl::new(
                IDK,
                "main".to_owned(),
                vec![],
                Type::Int32,
                Stmts::new(vec![Stmt::Expr(Expr::Call(Call::new(
                    IDK,
                    "a".to_string(),
                    vec![],
                )))]),
            ),
        ]);
        for code in codes {
            let program = program_parser(code);
            dbg!(&program);
            dbg!(&expect);
            assert_eq!(program, expect);
        }
    }

    #[test]
    fn test_if_else() {
        let codes: Vec<Span> = vec![
            "if(1) { }",
            "if(1) { 0; } else { 0; }",
            "if(1){ 1; } else { if(2){} else {} }",
            "if (1) { if (2) { 0; } } else { if(0) { 0; }}",
            "if (1 > 1 && 3 || 3 + 4 > 4 * 5 || 3) { if (2) {} } else { if(0) {}}",
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (rest, _) = if_else_parser(code).unwrap();
            check_consumed(code, rest);
        }
    }
    #[test]
    fn test_for() {
        let codes: Vec<Span> = vec![
            "for(var i: i32 ;i < 10; i = i + 1;){}",
            "for (var i : i32 ;i < 10; i = i + 1;){ return 0;}",
            r#"for (var i  : i32 ; i < 10; i = i + 1;) {
                a = a + i;
            }"#,
        ]
        .iter()
        .map(|code| Span::new(code))
        .collect();

        for code in codes {
            let (rest, _) = for_parser(code).unwrap();
            check_consumed(code, rest);
        }
    }
}
