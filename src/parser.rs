use std::collections::VecDeque;

use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

macro_rules! unary_expr {
    ($name:ident -> ($($op:ident),+) $right:ident | $else:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Option<Expr> {
            if tokens.front().filter(|c| { $(c.kind == TK::$op)||+ }).is_some() {
                let op = tokens.pop_front().expect("Should have a token");
                let right = $right(tokens)?;
                return Some(Expr::Unary(op, Box::new(right)));
            }

            $else(tokens)
        }
    };
}

macro_rules! binary_expr {
    ($name:ident -> $left:ident ($($op:ident),+) $right:ident) => {
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
    };
}

macro_rules! group_expr {
    ($name:ident -> ($left:ident) $expr:ident ($right:ident) | $else:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Option<Expr> {
            if tokens.front()?.kind == TK::$left {
                let l = tokens.pop_front().expect("Should have a token");
                let expr = $expr(tokens)?;

                let r = tokens.pop_front().expect("Missing closing parenthesis");
                assert_eq!(
                    r.kind,
                    TK::$right,
                    "Expected closing parenthesis, got {:#?}",
                    r
                );

                return Some(Expr::Group(l, Box::new(expr), r));
            }

            $else(tokens)
        }
    };
}

macro_rules! lit_expr {
    ($name:ident -> ($($lit:ident),+)) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Option<Expr> {
            let tok = tokens.pop_front()?;

            if $(tok.kind == TK::$lit)||+ {
                return Some(Expr::Literal(tok.literal.expect("Should have a literal")));
            }

            None
        }
    };
}

macro_rules! identity_expr {
    ($name:ident -> $fn:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Option<Expr> {
            $fn(tokens)
        }
    };
}

macro_rules! expr {
    ($name:ident -> ($($op:ident),+) $right:ident | $else:ident $($rest:tt)*) => {
        unary_expr!( $name -> ($($op),+) $right | $else );
        expr!( $($rest)* );
    };
    ($name:ident -> $left:ident ($($op:ident),+) $right:ident $($rest:tt)*) => {
        binary_expr!( $name -> $left ($($op),+) $right );
        expr!( $($rest)* );
    };
    ($name:ident -> ($left:ident) $expr:ident ($right:ident) | $else:ident $($rest:tt)*) => {
        group_expr!( $name -> ($left) $expr ($right) | $else );
        expr!( $($rest)* );
    };
    ($name:ident -> ($($lit:ident),+) $($rest:tt)*) => {
        lit_expr!( $name -> ($($lit),+) );
        expr!( $($rest)* );
    };
    ($name:ident -> $fn:ident $($rest:tt)*) => {
        identity_expr!( $name -> $fn );
        expr!( $($rest)* );
    };
    () => {};
}

expr! {
    expression -> equality
    equality   -> comparison (BangEqual, EqualEqual) comparison
    comparison -> term (Greater, GreaterEqual, Less, LessEqual) term
    term       -> factor (Plus, Minus) factor
    factor     -> unary (Star, Slash) unary
    unary      -> (Bang, Minus) unary | group
    group      -> (LeftParenthesis) equality (RightParenthesis) | literal
    literal    -> (True, False, Nil, Number, String)
}

pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
    let mut tokens = VecDeque::from(tokens);
    expression(&mut tokens)
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
        assert_eq!(expr.to_string(), "(* 2 (((+ 4 (- 6)))))");
    }
}
