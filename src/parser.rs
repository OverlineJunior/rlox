use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

/// Creates a function for a binary expression.
/// Pattern: `fn_name -> expr_fn (token, ...) expr_fn`.
macro_rules! binary_expr {
    ($($name:ident -> $left:ident ($($op:ident),+) $right:ident)*) => {
        $(
            fn $name(tokens: &mut Cursor<Token>) -> Option<Expr> {
                let mut expr = $left(tokens)?;

                while let Some(op) = tokens.current().filter(|c| {
                    $(c.kind == TK::$op)||+
                }) {
                    tokens.eat().expect("Should have a token");
                    let right = $right(tokens)?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }

                Some(expr)
            }
        )*
    };
}

fn expression(tokens: &mut Cursor<Token>) -> Option<Expr> {
    equality(tokens)
}

binary_expr! {
    equality -> comparison (BangEqual, EqualEqual) comparison
    comparison -> term (Greater, GreaterEqual, Less, LessEqual) term
    term -> factor (Plus, Minus) factor
    factor -> unary (Star, Slash) unary
}

fn unary(tokens: &mut Cursor<Token>) -> Option<Expr> {
    let tk = tokens.current()?.kind;

    if tk == TK::Bang || tk == TK::Minus {
        let op = tokens.eat().expect("Should have a token");
        let right = unary(tokens)?;
        return Some(Expr::Unary(op, Box::new(right)));
    }

    primary(tokens)
}

fn primary(tokens: &mut Cursor<Token>) -> Option<Expr> {
    if tokens.current()?.kind == TK::LeftParenthesis {
        tokens.eat().expect("Should have a token");
        let expr = expression(tokens)?;
        assert_eq!(
            tokens.eat().expect("Missing closing parenthesis").kind,
            TK::RightParenthesis,
            "Expected closing parenthesis, got {:#?}",
            tokens.prev()
        );
        return Some(Expr::Group(Box::new(expr)));
    }

    let tok = tokens.eat()?;

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
    use std::{fs, path::Path};
    use crate::{cursor::Cursor, scanner::tokenize, token::Token, token_kind::TokenKind as TK};

    use super::expression;

    #[test]
    fn test() {
        // 2 * (4 + -6)
        let tokens = vec![
            Token::new(TK::Number, "2".into(), 2.0.into(), 1),
            Token::symbol(TK::Star, "*".into(), 1),
            Token::symbol(TK::LeftParenthesis, "(".into(), 1),
            Token::new(TK::Number, "4".into(), 4.0.into(), 1),
            Token::symbol(TK::Plus, "+".into(), 1),
            Token::symbol(TK::Minus, "-".into(), 1),
            Token::new(TK::Number, "6".into(), 6.0.into(), 1),
            Token::symbol(TK::RightParenthesis, ")".into(), 1),
        ];
        let mut cursor = Cursor::new(tokens);
        let expr = expression(&mut cursor).expect("Should have an expression");
        assert_eq!(expr.to_string(), "(* 2 (group (+ 4 (- 6))))");
    }
}
