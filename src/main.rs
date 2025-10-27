mod colors;
mod game;
mod board;
mod player;
mod position;

use std::io;

use game::Game;
use colors::clear_console;
use player::{Player, PlayerStats};

fn main() {
    let mut player_x_stats = PlayerStats::new(player::Player::X);
    let mut player_o_stats = PlayerStats::new(player::Player::O);
    
    loop {
        let mut game = Game::new();
        
        clear_console();
        match game.start_game() {
            Some(Player::X) => player_x_stats.increment_score(), 
            Some(Player::O) => player_o_stats.increment_score(), 
            None => {},
        }

        println!("\n Score: \n{}: {}\n{}: {}\n", player_x_stats.player, player_x_stats.score, player_o_stats.player, player_o_stats.score);

        if !ask_for_rematch() {
            break
        }
    }
}

fn ask_for_rematch() -> bool {
    let mut player_input = String::new();

    println!("\nDo you want to play another round? (y/n):");
    
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line!");

    match player_input.trim().to_lowercase().as_str() {
        "y" => return true,
        _ => return false,
    }
}