use std::io;

use crate::board::Board;
use crate::player::Player;
use crate::colors::{colored_text, RED, GREEN, PURPLE};
use crate::position::Position;

pub struct Game {
    current_turn: Option<Player>,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Self {
            current_turn: Some(Player::X),
            board: Board::new(3),
        }
    }

    pub fn start_game(&mut self) {
        println!("{}\n RuStY TiCTaCToE! \n{}\n{}", colored_text("==================", PURPLE), colored_text("==================", PURPLE), self.board);
        self.game_loop()
    }

    fn game_loop(&mut self) {
        loop{
            let position = self.read_input();

            match self.board.make_move(&position, self.current_turn.unwrap()) {
                Ok(_) => {
                    println!("\n{}", self.board);
                    match self.current_turn {
                        Some(Player::X) => self.current_turn = Some(Player::O),
                        Some(Player::O) => self.current_turn = Some(Player::X),
                        None => {},
                    }
                },
                Err(e) => println!("{} at ({},{}): {:?}", colored_text("Error", RED), position.row, position.column, e),
            }

            match self.board.check_winner() {
                Some(player) => {
                    println!("{}\n Winner: {}\n{}", colored_text("===========", GREEN), player, colored_text("===========", GREEN));
                    break;
                },
                None => {},
            }
        }
    }

    fn read_input(&self) -> Position {
        let mut player_input = String::new();

        println!("|> {} <| Enter your move (row, column): ", self.current_turn.unwrap());
        
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line!");

        let mut nums = player_input.trim().split(',').map(|s| s.parse::<usize>().unwrap_or(0));
        let (row, column) = (nums.next().unwrap_or(0), nums.next().unwrap_or(0));
        
        Position::new(row, column)
    }

}