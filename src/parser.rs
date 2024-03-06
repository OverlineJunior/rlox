use crate::{
    cursor::Cursor, expr::Expr, literal::Literal, token::Token, token_kind::TokenKind as TK,
};

fn expression(tokens: &mut Cursor<Token>) -> Option<Expr> {
    equality(tokens)
}

fn equality(tokens: &mut Cursor<Token>) -> Option<Expr> {
    let mut expr = comparison(tokens)?;

    while let Some(op) = tokens
        .current()
        .filter(|c| c.kind == TK::EqualEqual || c.kind == TK::BangEqual)
    {
        tokens.eat().expect("Should have a token");
        let right = comparison(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }

    Some(expr)
}

fn comparison(tokens: &mut Cursor<Token>) -> Option<Expr> {
    let mut expr = term(tokens)?;

    while let Some(op) = tokens.current().filter(|c| {
        c.kind == TK::Greater
            || c.kind == TK::GreaterEqual
            || c.kind == TK::Less
            || c.kind == TK::LessEqual
            || c.kind == TK::EqualEqual
            || c.kind == TK::BangEqual
    }) {
        tokens.eat().expect("Should have a token");
        let right = term(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }

    Some(expr)
}

fn term(tokens: &mut Cursor<Token>) -> Option<Expr> {
    let mut expr = factor(tokens)?;

    while let Some(op) = tokens
        .current()
        .filter(|c| c.kind == TK::Plus || c.kind == TK::Minus)
    {
        tokens.eat().expect("Should have a token");
        let right = factor(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }

    Some(expr)
}

fn factor(tokens: &mut Cursor<Token>) -> Option<Expr> {
    let mut expr = unary(tokens)?;

    while let Some(op) = tokens
        .current()
        .filter(|c| c.kind == TK::Star || c.kind == TK::Slash)
    {
        tokens.eat().expect("Should have a token");
        let right = unary(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }

    Some(expr)
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
