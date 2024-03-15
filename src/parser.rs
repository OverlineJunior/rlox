use std::collections::VecDeque;

use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

macro_rules! binary_expr {
    (fn $name:ident = $left:ident ($($op:ident),+) $right:ident $($rest:tt)*) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
            let mut expr = $left(tokens)?;

            while tokens.front().is_some_and(|c| { $(c.kind == TK::$op)||+ }) {
                let op = tokens.pop_front().unwrap().clone();
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
    ternary(tokens)
}

fn ternary(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    let mut expr = equality(tokens)?;

    if tokens.front().is_some_and(|t| t.kind == TK::Question) {
        tokens.pop_front().unwrap();
        let if_ = expression(tokens)?;
        if tokens.pop_front().is_none() {
            return Err("Expected `:`".into());
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

fn unary(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    if tokens
        .front()
        .is_some_and(|t| matches!(t.kind, TK::Bang | TK::Minus))
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
        return Ok(Expr::Literal(tok.literal.unwrap_or_else(|| panic!("Expected token `{:?}` to have a literal", tok.kind))));
    }

    group(tokens)
}

fn group(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
    match tokens.front() {
        Some(t) if t.kind != TK::LeftParenthesis => {
            return Err(format!("Expected `(`, got {:?}", t.kind));
        },
        Some(_) => (),
        None => return Err("Expected token".into()),
    }

    let l = tokens.pop_front().unwrap();
    let expr = expression(tokens)?;

    match tokens.pop_front() {
        Some(r) if r.kind == TK::RightParenthesis => Ok(Expr::Group(Box::new(expr))),
        Some(r) => Err(format!("Expected closing `)`, got {:?}", r.kind)),
        None => Err("Expected closing `)`".into()),
    }
}

mod tests {
    use crate::{expr::Expr, parser::parse, scanner::tokenize};

    fn make_expr(s: &str) -> Expr {
        let tokens = tokenize(s.into()).expect("Should tokenize successfuly");
        parse(tokens).expect("Should give a correct expression")
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(make_expr("2 * (4 + -6)").to_string(), "(* 2 (group (+ 4 (- 6))))");
    }

    #[test]
    fn test_eq() {
        assert_eq!(make_expr("true == !!!false").to_string(), "(== true (! (! (! false))))");
    }

    #[test]
    fn test_comp() {
        assert_eq!(make_expr("1 + 1 < 2 * 2").to_string(), "(< (+ 1 1) (* 2 2))");
    }

    #[test]
    fn test_ternary() {
        assert_eq!(make_expr("0 ? 1 ? 2 : 3 : 4").to_string(), "(0 ? (1 ? 2 : 3) : 4)");
    }
}
