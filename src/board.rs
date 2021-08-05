use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mark {
    X,
    O,
}

impl Mark {
    fn flip(&self) -> Self {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
        }
    }

    fn option_to_string(mark: &Option<Self>) -> String {
        match mark {
            Some(Mark::X) => "X".to_string(),
            Some(Mark::O) => "O".to_string(),
            None => " ".to_string(),
        }
    }
}

enum Row {
    First,
    Second,
    Third,
}

#[derive(Clone, Copy)]
pub struct SquareIndex(usize);

impl SquareIndex {
    pub fn new(square: &usize) -> Result<SquareIndex, String> {
        if *square < 9 {
            Ok(SquareIndex(*square))
        } else {
            Err(format!("Square {} is not in range (0-8)", square))
        }
    }
}

struct InnerBoard {
    squares: [Option<Mark>; 9],
    winner: Option<Mark>,
}

impl InnerBoard {
    fn new() -> Self {
        Self {
            squares: [None; 9],
            winner: None,
        }
    }

    fn row_to_string(&self, row: &Row) -> String {
        let index_range = match row {
            Row::First => (0..3),
            Row::Second => (3..6),
            Row::Third => (6..9),
        };
        index_range
            .map(|s| format!("[{}]", Mark::option_to_string(&self.squares[s])))
            .collect::<String>()
    }

    fn do_move(&mut self, index: &SquareIndex, mark: &Mark) -> Result<(), String> {
        match self.squares[index.0] {
            Some(_) => Err(format!("Square {} is not empty", index.0)),
            None => {
                self.squares[index.0] = Some(*mark);
                self.set_winner();
                Ok(())
            }
        }
    }

    fn set_winner(&mut self) {
        if self.winner.is_some() {
        } else if (self.squares[0] == self.squares[1] && self.squares[1] == self.squares[2]
            || self.squares[0] == self.squares[3] && self.squares[3] == self.squares[6]
            || self.squares[0] == self.squares[4] && self.squares[4] == self.squares[8])
            && self.squares[0].is_some()
        {
            self.winner = self.squares[0];
        } else if (self.squares[4] == self.squares[2] && self.squares[2] == self.squares[6]
            || self.squares[4] == self.squares[3] && self.squares[3] == self.squares[5]
            || self.squares[4] == self.squares[1] && self.squares[1] == self.squares[7])
            && self.squares[4].is_some()
        {
            self.winner = self.squares[4];
        } else if (self.squares[2] == self.squares[5] && self.squares[5] == self.squares[8])
            && self.squares[2].is_some()
        {
            self.winner = self.squares[2];
        } else if (self.squares[6] == self.squares[7] && self.squares[7] == self.squares[8])
            && self.squares[6].is_some()
        {
            self.winner = self.squares[6];
        }
    }
}

pub struct Board {
    squares: [InnerBoard; 9],
    required_square: Option<SquareIndex>,
    active_mark: Mark,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.inner_row_to_string(&Row::First))?;
        writeln!(f, "---------------------------------")?;
        writeln!(f, "{}", self.inner_row_to_string(&Row::Second))?;
        writeln!(f, "---------------------------------")?;
        writeln!(f, "{}", self.inner_row_to_string(&Row::Third))?;

        writeln!(f, "---------")?;
        writeln!(f, "{}", self.row_to_string(&Row::First))?;
        writeln!(f, "{}", self.row_to_string(&Row::Second))?;
        writeln!(f, "{}", self.row_to_string(&Row::Third))?;

        let required = match self.required_square {
            Some(s) => s.0.to_string(),
            None => "Free".to_string(),
        };
        writeln!(f, "Required Board: {}", required)?;
        writeln!(
            f,
            "Active player: {}",
            Mark::option_to_string(&Some(self.active_mark))
        )
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
                InnerBoard::new(),
            ],
            required_square: None,
            active_mark: Mark::X,
        }
    }

    pub fn do_move(
        &mut self,
        outer_index: &SquareIndex,
        inner_index: &SquareIndex,
    ) -> Result<(), String> {
        match self.required_square {
            Some(SquareIndex(req_outer)) if req_outer != outer_index.0 => {
                Err(format!("Move must be in board {}", req_outer))
            }
            _ => {
                self.squares[outer_index.0].do_move(inner_index, &self.active_mark)?;
                self.required_square = match self.squares[inner_index.0].winner {
                    Some(_) => None,
                    None => Some(*inner_index),
                };
                self.active_mark = self.active_mark.flip();
                Ok(())
            }
        }
    }

    fn row_to_string(&self, row: &Row) -> String {
        let index_range = match row {
            Row::First => (0..3),
            Row::Second => (3..6),
            Row::Third => (6..9),
        };

        index_range
            .map(|s| format!("[{}]", Mark::option_to_string(&self.squares[s].winner)))
            .collect::<String>()
    }

    fn inner_row_to_string(&self, row: &Row) -> String {
        let inner_rows = vec![Row::First, Row::Second, Row::Third];
        let index_range = match row {
            Row::First => (0..3),
            Row::Second => (3..6),
            Row::Third => (6..9),
        };
        inner_rows
            .into_iter()
            .map(|r| {
                index_range
                    .clone()
                    .map(|s| self.squares[s].row_to_string(&r))
                    .collect::<Vec<_>>()
                    .join(" | ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
