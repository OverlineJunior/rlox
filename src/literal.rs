#[derive(Clone, Debug)]
pub enum Literal {
	Number(f64),
	String(String),
}

impl From<f64> for Literal {
	fn from(n: f64) -> Self {
		Literal::Number(n)
	}
}

impl From<&str> for Literal {
	fn from(s: &str) -> Self {
		Literal::String(s.into())
	}
}
