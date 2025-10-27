mod colors;
mod game;
mod board;
mod player;
mod position;

use std::io;

use game::Game;
use colors::clear_console;

fn main() {
    loop {
        let mut game = Game::new();
        
        clear_console();
        game.start_game();

        if !ask_for_rematch() {
            break
        }
    }
}

fn ask_for_rematch() -> bool {
        let mut player_input = String::new();

        println!("Do you want to play another round? (y/n):");
        
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line!");

        match player_input.trim().to_lowercase().as_str() {
            "y" => return true,
            _ => return false,
        }
    }