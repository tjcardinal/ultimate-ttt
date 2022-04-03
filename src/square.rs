use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_x_and_o() {
        assert_eq!(Mark::X.flip(), Mark::O);
        assert_eq!(Mark::O.flip(), Mark::X);
    }

    #[test]
    fn match_three() {
        let x = Square(Some(Mark::X));
        let o = Square(Some(Mark::O));

        assert_eq!(Square::check_match(&x, &x, &x), Some(Mark::X));
        assert_eq!(Square::check_match(&o, &o, &o), Some(Mark::O));
    }

    #[test]
    fn mismatch_three() {
        let x = Square(Some(Mark::X));
        let o = Square(Some(Mark::O));
        let n = Square(None);

        assert_eq!(Square::check_match(&x, &x, &o), None);
        assert_eq!(Square::check_match(&x, &x, &n), None);

        assert_eq!(Square::check_match(&x, &o, &x), None);
        assert_eq!(Square::check_match(&x, &o, &o), None);
        assert_eq!(Square::check_match(&x, &o, &n), None);

        assert_eq!(Square::check_match(&x, &n, &x), None);
        assert_eq!(Square::check_match(&x, &n, &o), None);
        assert_eq!(Square::check_match(&x, &n, &n), None);

        assert_eq!(Square::check_match(&o, &x, &x), None);
        assert_eq!(Square::check_match(&o, &x, &o), None);
        assert_eq!(Square::check_match(&o, &x, &n), None);

        assert_eq!(Square::check_match(&o, &o, &x), None);
        assert_eq!(Square::check_match(&o, &o, &n), None);

        assert_eq!(Square::check_match(&o, &n, &x), None);
        assert_eq!(Square::check_match(&o, &n, &o), None);
        assert_eq!(Square::check_match(&o, &n, &n), None);

        assert_eq!(Square::check_match(&n, &x, &x), None);
        assert_eq!(Square::check_match(&n, &x, &o), None);
        assert_eq!(Square::check_match(&n, &x, &n), None);

        assert_eq!(Square::check_match(&n, &o, &x), None);
        assert_eq!(Square::check_match(&n, &o, &o), None);
        assert_eq!(Square::check_match(&n, &o, &n), None);

        assert_eq!(Square::check_match(&n, &n, &x), None);
        assert_eq!(Square::check_match(&n, &n, &o), None);
        assert_eq!(Square::check_match(&n, &n, &n), None);
    }
}
