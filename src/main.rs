mod colors;
mod board;
mod player;

use board::Board;
use player::Player;

use crate::colors::{colored_text, GREEN, PURPLE};

fn main() {
    let mut board = Board::new(3);
    println!("{}\n RuStY TiCTaCToE! \n{}\n{}", colored_text("==================", PURPLE), colored_text("==================", PURPLE), board);

    for column in 0..3 {
        match board.make_move(0, column, Player::X) {
            Ok(_) => println!("\nRound {} \n{}", column + 1,  board),
            Err(e) => println!("Error at row {}: {:?}", column, e),
        }

        match board.check_winner() {
            Some(player) => println!("{}\n Winner: {}\n{}", colored_text("===========", GREEN), player, colored_text("===========", GREEN)),
            None => {},
        }
    }
}

