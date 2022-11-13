use std::fmt;

#[derive(Clone)]
pub enum Object {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string),
            Self::Nil => write!(f, "nil"),
        }
    }
}
