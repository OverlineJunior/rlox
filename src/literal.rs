use std::fmt;

#[derive(Clone, Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Literal {
    pub fn expect_number(&self) -> f64 {
        match self {
            Self::Number(n) => *n,
            v => panic!(
                "Expected `Literal` variant to be `Number`, but is `{:?}`",
                v
            ),
        }
    }

    pub fn expect_string(&self) -> &str {
        match self {
            Self::String(s) => s,
            v => panic!(
                "Expected `Literal` variant to be `String`, but is `{:?}`",
                v
            ),
        }
    }

    pub fn expect_bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            v => panic!("Expected `Literal` variant to be `Bool`, but is `{:?}`", v),
        }
    }

    pub fn expect_nil(&self) {
        match self {
            Self::Nil => (),
            _ => panic!("Expected `Literal` variant to be `Nil`"),
        }
    }
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

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Bool(value)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}
