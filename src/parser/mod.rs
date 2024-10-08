pub mod expr;
pub mod parse_error;
pub mod stmt;

use crate::{
    cursor::Cursor,
    scanner::{literal::Literal, token::Token, token_kind::TokenKind as TK},
};

use expr::Expr;
use parse_error::ParseError::{self, *};
use stmt::Stmt;

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

/// Maps tokens into statements.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    let mut tokens = Cursor::new(tokens);
    let mut stmts: Vec<Stmt> = vec![];

    while tokens.current().is_some() {
        // Although on paper a program is a bunch of statements, declaration statements
        // must be separated because of this specific design choice:
        // Allowed:     if (foo) print "bar";     (is a statement, all good)
        // Not allowed: if (foo) var bar = "baz"; (is a declaration, not good)
        stmts.push(declaration(&mut tokens)?);
    }

    Ok(stmts)
}

// Below are the parsing functions, where each correspond to a specific rule / production in the grammar.
// They are organized in such a way that the deeper the function is, the higher its precedence,
// meaning it is evaluated first.

fn declaration(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    match tokens
        .current()
        .expect("Should not be called with empty cursor")
        .kind
    {
        TK::Var => var_declaration(tokens),
        _ => statement(tokens),
    }
}

fn var_declaration(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let var = tokens
        .eat_kind(TK::Var)
        .expect("Should be called when Var is the current token");

    let name = tokens.eat_kind(TK::Identifier)?;

    let init = match tokens.eat_kind(TK::Equal) {
        Ok(_) => expression(tokens)?,
        // Unitiliazed variables are defaulted to Nil.
        Err(_) => Expr::Literal(Literal::Nil),
    };

    tokens.eat_kind(TK::Semicolon)?;

    Ok(Stmt::Var { name, init })
}

fn statement(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    match tokens
        .current()
        .expect("Should not be called with empty cursor")
        .kind
    {
        TK::Print => print_stmt(tokens),
        TK::LeftBrace => block(tokens),
        TK::If => if_stmt(tokens),
        TK::While => while_stmt(tokens),
        _ => expr_stmt(tokens),
    }
}

fn print_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let print = tokens
        .eat_kind(TK::Print)
        .expect("Should be called when print is the current token");

    let value = expression(tokens)?;

    tokens.eat_kind(TK::Semicolon)?;

    Ok(Stmt::Print(value))
}

fn expr_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let expr = expression(tokens)?;

    tokens.eat_kind(TK::Semicolon)?;

    Ok(Stmt::Expr(expr))
}

fn block(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let left_brace = tokens
        .eat_kind(TK::LeftBrace)
        .expect("Should be called when LeftBrace is the current token");

    let mut stmts: Vec<Stmt> = Vec::new();
    while tokens.current().is_some_and(|t| t.kind != TK::RightBrace) {
        stmts.push(declaration(tokens)?);
    }

    tokens.eat_kind(TK::RightBrace)?;

    Ok(Stmt::Block(stmts))
}

fn if_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let if_ = tokens
        .eat_kind(TK::If)
        .expect("Should be called when If is the current token");

    tokens.eat_kind(TK::LeftParenthesis)?;

    let condition = expression(tokens)?;

    tokens.eat_kind(TK::RightParenthesis)?;

    let then_branch = Box::new(statement(tokens)?);

    let else_branch = if tokens.eat_kind(TK::Else).is_ok() {
        Some(Box::new(statement(tokens)?))
    } else {
        None
    };

    Ok(Stmt::If {
        condition,
        then_branch,
        else_branch,
    })
}

fn while_stmt(tokens: &mut Cursor<Token>) -> Result<Stmt, ParseError> {
    let while_ = tokens
        .eat_kind(TK::While)
        .expect("Should be called when While is the current token");

    tokens.eat_kind(TK::LeftParenthesis)?;

    let condition = expression(tokens)?;

    tokens.eat_kind(TK::RightParenthesis)?;

    let body = Box::new(statement(tokens)?);

    Ok(Stmt::While { condition, body })
}

fn expression(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    assignment(tokens)
}

fn assignment(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    // Even though we error if expr is not a variable later on, we still search for
    // anything ternary below, as long as it resolves to a variable.
    // This allows things such as: a ? b : c = d and: a.b = c
    let expr = ternary(tokens)?;

    if tokens.current().is_some_and(|t| t.kind == TK::Equal) {
        let equal = tokens.eat().unwrap();
        // We recursively go for another assignment to allow operator chaining (--1).
        let value = assignment(tokens)?;

        if let Expr::Variable { name } = expr {
            return Ok(Expr::Assign {
                name,
                value: Box::new(value),
            });
        } else {
            // a + b = c errors because a + c does not resolve to a variable.
            return Err(BadAssignmentTarget { line: equal.line });
        }
    }

    Ok(expr)
}

fn ternary(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    let mut expr = or(tokens)?;

    if tokens.current().is_some_and(|t| t.kind == TK::Question) {
        let question = tokens.eat().unwrap();
        let if_ = expression(tokens)?;

        tokens.eat_kind(TK::Colon)?;

        let else_ = expression(tokens)?;

        expr = Expr::Ternary(Box::new(expr), Box::new(if_), Box::new(else_));
    }

    Ok(expr)
}

fn or(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    let mut expr = and(tokens)?;

    while let Ok(op) = tokens.eat_kind(TK::Or) {
        let right = and(tokens)?;
        expr = Expr::Logical(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn and(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    let mut expr = equality(tokens)?;

    while let Ok(op) = tokens.eat_kind(TK::And) {
        let right = equality(tokens)?;
        expr = Expr::Logical(Box::new(expr), op, Box::new(right));
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

    variable(tokens)
}

fn variable(tokens: &mut Cursor<Token>) -> Result<Expr, ParseError> {
    if tokens.current().is_some_and(|t| t.kind == TK::Identifier) {
        let name = tokens.eat().unwrap();
        return Ok(Expr::Variable { name });
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

    tokens.eat_kind(TK::RightParenthesis)?;

    Ok(Expr::Group(Box::new(expr)))
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

mod tests {
    use crate::{parser::parse, scanner::tokenize};

    #[test]
    fn test() {
        let tokens = tokenize("a ? b : c = 555;".to_string()).unwrap();
        let ast = parse(tokens).unwrap();
        println!("{:#?}", ast);
    }
}
