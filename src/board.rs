use crate::square;
use std::fmt;

const BOARD_SIZE: usize = 9;

#[derive(Clone, Copy, PartialEq)]
pub enum BoardState {
    Winner(square::Mark),
    Draw,
    InProgress,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Index(usize);

impl Index {
    pub fn new(i: &usize) -> Result<Self, String> {
        if *i < BOARD_SIZE {
            Ok(Self(*i))
        } else {
            Err(format!("Index {} is not in range (0-{})", i, BOARD_SIZE))
        }
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct InnerBoard {
    squares: [square::Square; BOARD_SIZE],
    state: BoardState,
}

impl InnerBoard {
    pub fn new() -> Self {
        Self {
            squares: [square::Square(None); BOARD_SIZE],
            state: BoardState::InProgress,
        }
    }

    pub fn get_state(&self) -> BoardState {
        self.state
    }

    pub fn do_move(&mut self, mark: &square::Mark, index: &Index) -> Result<(), String> {
        match self.squares[index.get()].0 {
            Some(_) => Err(format!("Square {} is not empty", index.get())),
            None => {
                self.squares[index.get()] = square::Square(Some(*mark));
                self.update_state();
                Ok(())
            }
        }
    }

    pub fn rows_as_strings(&self) -> Vec<String> {
        vec![(0..3), (3..6), (6..9)]
            .into_iter()
            .map(|r| r.map(|s| format!("[{}]", self.squares[s])).collect())
            .collect()
    }

    pub fn check_match(a: &Self, b: &Self, c: &Self) -> Option<square::Mark> {
        match a.get_state() {
            BoardState::Winner(m)
                if a.get_state() == b.get_state() && b.get_state() == c.get_state() =>
            {
                Some(m)
            }
            _ => None,
        }
    }

    fn update_state(&mut self) {
        let wins = vec![
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        if self.state == BoardState::InProgress {
            for (a, b, c) in wins {
                if let Some(winner) = square::Square::check_match(
                    &self.squares[a],
                    &self.squares[b],
                    &self.squares[c],
                ) {
                    self.state = BoardState::Winner(winner);
                }
            }
            if self.state == BoardState::InProgress && self.is_full() {
                self.state = BoardState::Draw;
            }
        }
    }

    fn is_full(&self) -> bool {
        for s in 0..BOARD_SIZE {
            if None == self.squares[s].0 {
                return false;
            }
        }
        true
    }
}

pub struct OuterBoard {
    squares: [InnerBoard; BOARD_SIZE],
    state: BoardState,
    required_index: Option<Index>,
}

impl OuterBoard {
    pub fn new() -> Self {
        Self {
            squares: [InnerBoard::new(); BOARD_SIZE],
            state: BoardState::InProgress,
            required_index: None,
        }
    }

    pub fn get_state(&self) -> BoardState {
        self.state
    }

    pub fn do_move(
        &mut self,
        mark: &square::Mark,
        outer: &Index,
        inner: &Index,
    ) -> Result<(), String> {
        match self.required_index {
            Some(i) if i != *outer => Err(format!("Move must be in board {}", i.get())),
            _ => match self.squares[outer.get()].do_move(mark, inner) {
                Ok(_) => {
                    self.update_state();
                    self.required_index = match self.squares[inner.get()].get_state() {
                        BoardState::InProgress => Some(*inner),
                        _ => None,
                    };
                    Ok(())
                }
                Err(e) => Err(format!("Board {}: {}", outer.get(), e)),
            },
        }
    }

    fn update_state(&mut self) {
        let wins = vec![
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        if self.state == BoardState::InProgress {
            for (a, b, c) in wins {
                if let Some(winner) =
                    InnerBoard::check_match(&self.squares[a], &self.squares[b], &self.squares[c])
                {
                    self.state = BoardState::Winner(winner);
                }
            }
            if self.state == BoardState::InProgress && self.is_full() {
                self.state = BoardState::Draw;
            }
        }
    }

    fn is_full(&self) -> bool {
        for s in 0..BOARD_SIZE {
            if BoardState::InProgress == self.squares[s].get_state() {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for OuterBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in vec![(0..3), (3..6), (6..9)] {
            let s = r
                .map(|i| self.squares[i].rows_as_strings())
                .collect::<Vec<_>>();

            let first = &s[0];
            let second = &s[1];
            let third = &s[2];

            for (a, (b, c)) in first.iter().zip(second.iter().zip(third.iter())) {
                writeln!(f, "{} | {} | {}", a, b, c)?;
            }
            writeln!(f, "---------------------------------")?;
        }
        Ok(())
    }
}
