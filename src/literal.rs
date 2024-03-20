use std::{fmt, ops::{Add, Div, Mul, Sub}};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Literal {
    /// Returns `false` if self is `Nil` or `Bool(false)`.
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Nil => false,
            _ => true,
        }
    }

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

impl Add for Literal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => (l + r).into(),
            (Self::String(l), Self::String(r)) => (l + &r).into(),
            (l, r) => panic!("Cannot add `{}` with `{}`", l, r),
        }
    }
}

impl Sub for Literal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.expect_number() - rhs.expect_number()).into()
    }
}

impl Mul for Literal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.expect_number() * rhs.expect_number()).into()
    }
}

impl Div for Literal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self.expect_number() / rhs.expect_number()).into()
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.partial_cmp(r),
            _ => None,
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

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
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
