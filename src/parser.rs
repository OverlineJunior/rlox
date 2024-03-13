use std::collections::VecDeque;

use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

macro_rules! binary_expr {
    (fn $name:ident = $left:ident ($($op:ident),+) $right:ident $($rest:tt)*) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
            let mut expr = $left(tokens)?;

            while tokens.front().filter(|c| { $(c.kind == TK::$op)||+ }).is_some() {
                let op = tokens.front().unwrap().clone();
                tokens.pop_front().unwrap();
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
fn sync(tokens: &mut VecDeque<Token>) {
    while let Some(prev_token) = tokens.pop_front() {
        let tk = tokens.front().map(|t| t.kind);

        if prev_token.kind == TK::Semicolon || (tk.is_some() && tk.unwrap().is_stmt()) {
            break;
        }
    }
}

/// Parses a vec of tokens that compose only a single expression.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = VecDeque::from(tokens);
    expression(&mut tokens)
}

fn expression(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    equality(tokens)
}

binary_expr!(
    fn equality = comparison (BangEqual, EqualEqual) comparison
    fn comparison = term (Greater, GreaterEqual, Less, LessEqual) term
    fn term = factor (Plus, Minus) factor
    fn factor = unary (Star, Slash) unary
);

fn unary(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    if tokens
        .front()
        .filter(|c| c.kind == TK::Bang || c.kind == TK::Minus)
        .is_some()
    {
        let op = tokens.pop_front().unwrap();
        let right = unary(tokens)?;
        return Ok(Expr::Unary(op, Box::new(right)));
    }

    literal(tokens)
}

fn literal(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    let t = tokens.front().ok_or("Expected token")?;

    if t.kind.is_lit() {
        let tok = tokens.pop_front().unwrap();
        return Ok(Expr::Literal(tok.literal.expect("Should have a literal")));
    }

    group(tokens)
}

fn group(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    match tokens.front() {
        Some(t) => {
            if t.kind != TK::LeftParenthesis {
                return Err(format!("Expected (, got {:?}", t.kind));
            }
        }
        None => return Err("Expected token".into()),
    }

    let l = tokens.pop_front().unwrap();
    let expr = expression(tokens)?;

    match tokens.pop_front() {
        Some(r) if r.kind == TK::RightParenthesis => Ok(Expr::Group(Box::new(expr))),
        Some(r) => Err(format!("Expected ), got {:?}", r)),
        None => Err("Expected ), got nothing".into()),
    }
}

mod tests {
    use crate::{parser::parse, scanner::tokenize};

    #[test]
    fn test() {
        let tokens = tokenize("2 * (4 + -6)".into()).expect("Should tokenize successfuly");
        let expr = parse(tokens).expect("Should give a correct expression");
        assert_eq!(expr.to_string(), "(* 2 (group (+ 4 (- 6))))");
    }
}
