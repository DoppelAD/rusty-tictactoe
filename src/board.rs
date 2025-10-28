use std::fmt::{self};

use crate::player::Player;
use crate::position::Position;

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    AlreadyUsed,
}

#[derive(Debug)]
pub struct Board {
    pub cells: Vec<Vec<Option<Player>>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![vec![None; size]; size],
        }
    }

    pub fn make_move(&mut self, position: &Position, player: Player) -> Result<(), BoardError> {
        if position.row > self.cells.len() || position.column > self.cells[0].len() {
            return Err(BoardError::OutOfBounds);
        }

        if self.cells[position.row][position.column].is_some() {
            return Err(BoardError::AlreadyUsed);
        }

        self.cells[position.row][position.column] = Some(player);

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.is_some())
    }

    pub fn check_winner(&self) -> Option<Player> {
        let size = self.cells.len();

        if let Some(winner) = self
            .check_diagonals(size)
            .or(self.check_rows_and_columns(size))
        {
            return Some(winner);
        }

        None
    }

    fn check_rows_and_columns(&self, size: usize) -> Option<Player> {
        for row in &self.cells {
            if let Some(winner) = self.check_line(row) {
                return Some(winner);
            }
        }

        for col in 0..size {
            let column: Vec<Option<Player>> = self.cells.iter().map(|r| r[col]).collect();
            if let Some(winner) = self.check_line(&column) {
                return Some(winner);
            }
        }

        None
    }

    fn check_diagonals(&self, size: usize) -> Option<Player> {
        let main_diag: Vec<Option<Player>> = (0..size).map(|i| self.cells[i][i]).collect();
        if let Some(winner) = self.check_line(&main_diag) {
            return Some(winner);
        }

        let anty_diag: Vec<Option<Player>> =
            (0..size).map(|i| self.cells[i][size - 1 - i]).collect();
        if let Some(winner) = self.check_line(&anty_diag) {
            return Some(winner);
        }

        None
    }

    fn check_line(&self, line: &[Option<Player>]) -> Option<Player> {
        if let Some(first) = line[0] {
            if line.iter().all(|cell| *cell == Some(first)) {
                return Some(first);
            }
        }

        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;

        for column in &self.cells {
            for cell in column {
                match cell {
                    Some(player) => write!(f, "|{}|", player)?,
                    None => write!(f, "| |")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
