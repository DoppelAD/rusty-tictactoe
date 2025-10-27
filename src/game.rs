use crate::board::Board;
use crate::player::Player;
use crate::colors::{colored_text, GREEN, PURPLE};

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
        for column in 0..3 {
            match self.board.make_move(0, column, self.current_turn.unwrap()) {
                Ok(_) => {
                    println!("\nRound {} \n{}", column + 1,  self.board);
                    match self.current_turn {
                        Some(Player::X) => self.current_turn = Some(Player::O),
                        Some(Player::O) => self.current_turn = Some(Player::X),
                        None => {},
                    }
                },
                Err(e) => println!("Error at row {}: {:?}", column, e),
            }

            match self.board.check_winner() {
                Some(player) => println!("{}\n Winner: {}\n{}", colored_text("===========", GREEN), player, colored_text("===========", GREEN)),
                None => {},
            }
        }
    }
}