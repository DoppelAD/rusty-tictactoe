#[allow(dead_code)]
use std::fmt::{self};
use crate::colors::{colored_text, RED, BLUE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

pub struct PlayerStats {
    pub player: Player,
    pub score: u32,
}

impl PlayerStats {
    pub fn new(player: Player) -> PlayerStats {
        Self { player, score: 0 }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
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