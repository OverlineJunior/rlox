use std::collections::VecDeque;

use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

/// Creates a function for a binary expression.
/// Pattern: `fn_name -> expr_fn (token, ...) expr_fn`.
macro_rules! binary_expr {
    ($($name:ident -> $left:ident ($($op:ident),+) $right:ident)*) => {
        $(
            fn $name(tokens: &mut VecDeque<Token>) -> Option<Expr> {
                let mut expr = $left(tokens)?;

                while tokens.front().filter(|c| { $(c.kind == TK::$op)||+ }).is_some() {
                    let op = tokens.front().expect("Should have a token").clone();
                    tokens.pop_front().expect("Should have a token");
                    let right = $right(tokens)?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }

                Some(expr)
            }
        )*
    };
}

fn expression(tokens: &mut VecDeque<Token>) -> Option<Expr> {
    equality(tokens)
}

binary_expr! {
    equality -> comparison (BangEqual, EqualEqual) comparison
    comparison -> term (Greater, GreaterEqual, Less, LessEqual) term
    term -> factor (Plus, Minus) factor
    factor -> unary (Star, Slash) unary
}

fn unary(tokens: &mut VecDeque<Token>) -> Option<Expr> {
    let tk = tokens.front()?.kind;

    if tk == TK::Bang || tk == TK::Minus {
        let op = tokens.pop_front().expect("Should have a token");
        let right = unary(tokens)?;
        return Some(Expr::Unary(op, Box::new(right)));
    }

    primary(tokens)
}

fn primary(tokens: &mut VecDeque<Token>) -> Option<Expr> {
    if tokens.front()?.kind == TK::LeftParenthesis {
        tokens.pop_front().expect("Should have a token");
        let expr = expression(tokens)?;

        let t = tokens.pop_front().expect("Missing closing parenthesis");
        assert_eq!(
            t.kind,
            TK::RightParenthesis,
            "Expected closing parenthesis, got {:#?}",
            t
        );

        return Some(Expr::Group(Box::new(expr)));
    }

    let tok = tokens.pop_front()?;

    let lit = match tok.kind {
        TK::True => Some(true.into()),
        TK::False => Some(false.into()),
        TK::Nil => Some(Literal::Nil),

        TK::Number | TK::String => Some(
            tok.literal
                .expect("Should have a literal, since kind is either Number or String"),
        ),

        _ => None,
    }?;

    Some(Expr::Literal(lit))
}

mod tests {
    use crate::{cursor::Cursor, scanner::tokenize, token::Token, token_kind::TokenKind as TK};
    use std::{collections::VecDeque, fs, path::Path};

    use super::expression;

    #[test]
    fn test() {
        // 2 * (4 + -6)
        let mut tokens = VecDeque::from([
            Token::new(TK::Number, "2".into(), 2.0.into(), 1),
            Token::symbol(TK::Star, "*".into(), 1),
            Token::symbol(TK::LeftParenthesis, "(".into(), 1),
            Token::new(TK::Number, "4".into(), 4.0.into(), 1),
            Token::symbol(TK::Plus, "+".into(), 1),
            Token::symbol(TK::Minus, "-".into(), 1),
            Token::new(TK::Number, "6".into(), 6.0.into(), 1),
            Token::symbol(TK::RightParenthesis, ")".into(), 1),
        ]);
        let expr = expression(&mut tokens).expect("Should have an expression");
        assert_eq!(expr.to_string(), "(* 2 (group (+ 4 (- 6))))");
    }
}
