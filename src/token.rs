use crate::literal::Literal;

#[derive(Clone, Debug)]
pub enum TokenKind {
	LeftParenthesis,
	RightParenthesis,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	Identifier,
	String,
	Number,
	And,
	Class,
	Else,
	False,
	True,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	Var,
	While,
	Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub lexeme: String,
	pub literal: Option<Literal>,
	pub line: usize,
}

impl Token {
	pub fn new(kind: TokenKind, lexeme: String, literal: Literal, line: usize) -> Token {
		Token { kind, lexeme, literal: Some(literal), line }
	}

	/// A symbol (or symbolic token) is a token that does not have a literal value.
	pub fn symbol(kind: TokenKind, lexeme: String, line: usize) -> Token {
		Token { kind, lexeme, literal: None, line }
	}
}
