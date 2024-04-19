use crate::{
    cursor::Cursor,
    error::parse_error::ParseError::{self, *},
    expr::Expr,
    stmt::Stmt,
    token::Token,
    token_kind::TokenKind as TK,
};

macro_rules! binary_expr {
    (fn $name:ident = $left:ident ($($op:ident),+) $right:ident $($rest:tt)*) => {
        fn $name(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
            let mut expr = $left(tokens)?;

            while tokens.current().is_some_and(|c| { $(c.kind == TK::$op)||+ }) {
                let op = tokens.eat().unwrap().clone();
                let right = $right(tokens)?;
                expr = Expr::Binary(Box::new(expr), op, Box::new(right));
            }

            Ok(expr)
        }

        binary_expr!($($rest)*);
    };
    () => {};
}

// Eats tokens until the next statement boundary.
// Used to discard tokens likely to cause cascaded errors after a parse error.
// https://en.wikipedia.org/wiki/Cascading_failure.
fn sync(tokens: &mut Cursor<Token>) {
    while let Some(prev_token) = tokens.eat() {
        let tk = tokens.current().map(|t| t.kind);

        if prev_token.kind == TK::Semicolon || (tk.is_some() && tk.unwrap().is_stmt()) {
            break;
        }
    }
}

/// Parses a vec of tokens that compose only a single expression.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    let mut tokens = Cursor::new(tokens);
    let mut stmts: Vec<Stmt> = vec![];

    while tokens.current().is_some() {
        stmts.push(statement(&mut tokens)?);
    }

    Ok(stmts)
}

fn statement(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    match tokens
        .current()
        .expect("Should not be called with empty cursor")
        .kind
    {
        TK::Print => print_stmt(tokens),
        _ => expr_stmt(tokens),
    }
}

fn print_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let print = tokens
        .eat()
        .expect("Should be called when print is the current token");
    let value = expression(tokens)?;

    match tokens.eat() {
        Some(t) if t.kind == TK::Semicolon => Ok(Stmt::Print(value)),
        Some(t) => Err(ExpectedSemicolon {
            got: Some(t.kind),
            line: t.line,
        }),
        None => Err(ExpectedSemicolon {
            got: None,
            line: tokens.prev().map(|t| t.line).unwrap_or(0),
        }),
    }
}

fn expr_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let expr = expression(tokens)?;

    match tokens.eat() {
        Some(t) if t.kind == TK::Semicolon => Ok(Stmt::Expr(expr)),
        Some(t) => Err(ExpectedSemicolon {
            got: Some(t.kind),
            line: t.line,
        }),
        None => Err(ExpectedSemicolon {
            got: None,
            line: tokens.prev().map(|t| t.line).unwrap_or(0),
        }),
    }
}

fn expression(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    ternary(tokens)
}

fn ternary(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    let mut expr = equality(tokens)?;

    if tokens.current().is_some_and(|t| t.kind == TK::Question) {
        let question = tokens.eat().unwrap();
        let if_ = expression(tokens)?;

        // The colon.
        match tokens.eat() {
            Some(t) if t.kind != TK::Colon => {
                return Err(ExpectedToken {
                    expected: TK::Colon,
                    got: Some(t.kind),
                    line: t.line,
                })
            }
            None => {
                return Err(ExpectedToken {
                    expected: TK::Colon,
                    got: None,
                    line: question.line,
                })
            }
            _ => (),
        }

        let else_ = expression(tokens)?;
        expr = Expr::Ternary(Box::new(expr), Box::new(if_), Box::new(else_));
    }

    Ok(expr)
}

binary_expr!(
    fn equality = comparison (BangEqual, EqualEqual) comparison
    fn comparison = term (Greater, GreaterEqual, Less, LessEqual) term
    fn term = factor (Plus, Minus) factor
    fn factor = unary (Star, Slash) unary
);

fn unary(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    if tokens
        .current()
        .is_some_and(|t| matches!(t.kind, TK::Bang | TK::Minus))
    {
        let op = tokens.eat().unwrap();
        let right = unary(tokens)?;
        return Ok(Expr::Unary(op, Box::new(right)));
    }

    literal(tokens)
}

fn literal(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    // Lazy evaluation is needed, otherwise `tokens.prev` will error. This is why `ok_or` is not used.
    let t = if let Some(t) = tokens.current() {
        t
    } else {
        return Err(ExpectedAnyToken {
            line: tokens.prev().expect("`tokens` should not be empty").line,
        });
    };

    if t.kind.is_lit() {
        let tok = tokens.eat().unwrap();
        return Ok(Expr::Literal(tok.literal.unwrap_or_else(|| {
            panic!("Expected token `{:?}` to have a literal", tok.kind)
        })));
    }

    group(tokens)
}

fn group(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    // The opening parenthesis.
    match tokens.current() {
        Some(t) if t.kind == TK::LeftParenthesis => (),
        Some(t) => return Err(last_parse_error(tokens)),
        None => {
            return Err(ExpectedToken {
                expected: TK::LeftParenthesis,
                got: None,
                line: tokens.prev().expect("`tokens` should not be empty").line,
            });
        }
    };

    let opening = tokens.eat().unwrap();
    let expr = expression(tokens)?;

    match tokens.eat() {
        Some(r) if r.kind == TK::RightParenthesis => Ok(Expr::Group(Box::new(expr))),
        Some(r) => Err(ExpectedToken {
            expected: TK::RightParenthesis,
            got: Some(r.kind),
            line: r.line,
        }),
        None => Err(ExpectedToken {
            expected: TK::RightParenthesis,
            got: None,
            line: opening.line,
        }),
    }
}

// Should be ran by the last expression function when there is no more parseable expressions.
fn last_parse_error(tokens: &mut Cursor<Token>) -> ParseError {
    if let Some(t) = tokens.eat() {
        return if matches!(
            t.kind,
            TK::BangEqual
                | TK::EqualEqual
                | TK::Greater
                | TK::GreaterEqual
                | TK::Less
                | TK::LessEqual
                | TK::Plus
                | TK::Slash
                | TK::Star
        ) {
            ExpectedAnyLeftOperand {
                operator: t.kind,
                line: t.line,
            }
        } else {
            NotParseable {
                token: t.kind,
                line: t.line,
            }
        };
    }

    // TODO! Add actual line number to the error, which will require the last line to be known.
    ExpectedAnyToken {
        line: tokens.prev().expect("`tokens` should not be empty").line,
    }
}

// TODO! Update tests based on recent parser changes.
// mod tests {
//     use crate::{expr::Expr, parser::parse, scanner::tokenize};

//     fn make_expr(s: &str) -> Expr {
//         let tokens = tokenize(s.into()).expect("Should tokenize successfuly");
//          parse(tokens).expect("Should give a correct expression")
//     }

//     #[test]
//     fn test_arithmetic() {
//         assert_eq!(
//             make_expr("2 * (4 + -6)").to_string(),
//             "(* 2 (group (+ 4 (- 6))))"
//         );
//     }

//     #[test]
//     fn test_eq() {
//         assert_eq!(
//             make_expr("true == !!!false").to_string(),
//             "(== true (! (! (! false))))"
//         );
//     }

//     #[test]
//     fn test_comp() {
//         assert_eq!(
//             make_expr("1 + 1 < 2 * 2").to_string(),
//             "(< (+ 1 1) (* 2 2))"
//         );
//     }

//     #[test]
//     fn test_ternary() {
//         assert_eq!(
//             make_expr("0 ? 1 ? 2 : 3 : 4").to_string(),
//             "(0 ? (1 ? 2 : 3) : 4)"
//         );
//     }
// }
