#[allow(dead_code)]
use std::fmt::{self};
use crate::colors::{colored_text, RED, BLUE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "{}", colored_text("X", RED))?,
            Player::O => write!(f, "{}", colored_text("O", BLUE))?,
        }
        
        Ok(())
    }
} 