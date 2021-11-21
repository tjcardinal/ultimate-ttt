use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    pub fn flip(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::X => "X",
            Self::O => "O",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Square(pub Option<Mark>);

impl Square {
    pub fn check_match(a: &Self, b: &Self, c: &Self) -> Option<Mark> {
        match a.0 {
            Some(m) if a == b && b == c => Some(m),
            _ => None,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.0 {
            Some(m) => m.to_string(),
            None => " ".to_string(),
        };
        write!(f, "{}", symbol)
    }
}
