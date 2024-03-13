use std::collections::VecDeque;

use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

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

macro_rules! unary_expr {
    ($name:ident -> ($($op:ident),+) $right:ident | $else:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
            if tokens.front().filter(|c| { $(c.kind == TK::$op)||+ }).is_some() {
                let op = tokens.pop_front().unwrap();
                let right = $right(tokens)?;
                return Ok(Expr::Unary(op, Box::new(right)));
            }

            $else(tokens)
        }
    };
}

macro_rules! binary_expr {
    ($name:ident -> $left:ident ($($op:ident),+) $right:ident) => {
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
    };
}

macro_rules! group_expr {
    ($name:ident -> ($left:ident) $expr:ident ($right:ident) | $else:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
            match tokens.front() {
                Some(t) => {
                    if t.kind != TK::$left {
                        return $else(tokens);
                    }
                }
                None => return Err("Expected token".into()),
            }

            let l = tokens.pop_front().unwrap();
            let expr = $expr(tokens)?;

            match tokens.pop_front() {
                Some(r) if r.kind == TK::$right => Ok(Expr::Group(l, Box::new(expr), r)),
                Some(r) => Err(format!("Expected {:?}, got {:?}", TK::$right, r)),
                None => return Err(format!("Expected {:?}", TK::$right)),
            }
        }
    };
}

macro_rules! lit_expr {
    ($name:ident -> ($($lit:ident),+)) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
            let tok = tokens.pop_front().ok_or("Expected token")?;

            if $(tok.kind == TK::$lit)||+ {
                return Ok(Expr::Literal(tok.literal.expect("Should have a literal")));
            }

            Err("Expected expression".into())
        }
    };
}

macro_rules! identity_expr {
    ($name:ident -> $fn:ident) => {
        fn $name(tokens: &mut VecDeque<Token>) -> Result<Expr, String> {
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

// Each function in the grammar will attempt to generate a kind of expression of its name or
// any kind of expression below it. The later the function is defined, the higher its priority.
// Take factor for example: if the current token can be parsed as a factor, a factor expression
// is generated. If not, it will try, in order, unary, group and then literal. Since factor is
// below term, factor has the higher priority.
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

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = VecDeque::from(tokens);
    expression(&mut tokens)
}

mod tests {
    use crate::{parser::parse, scanner::tokenize};

    #[test]
    fn test() {
        let tokens = tokenize("2 * (4 + -6)".into()).expect("Should tokenize successfuly");
        let expr = parse(tokens).expect("Should give a correct expression");
        assert_eq!(expr.to_string(), "(* 2 (((+ 4 (- 6)))))");
    }
}
