mod board;
mod colors;
mod game;
mod player;
mod position;

use std::io;

use colors::clear_console;
use game::Game;
use player::{Player, PlayerStats};

fn main() {
    let mut player_x_stats = PlayerStats::new(player::Player::X);
    let mut player_o_stats = PlayerStats::new(player::Player::O);

    loop {
        clear_console();
        play_game(&mut player_x_stats, &mut player_o_stats);
        print_score(&player_x_stats, &player_o_stats);

        if !ask_for_rematch() {
            break;
        }
    }
}

fn print_score(player_x_stats: &PlayerStats, player_o_stats: &PlayerStats) {
    println!(
        "\n Score: \n{}: {}\n{}: {}\n",
        player_x_stats.player, 
        player_x_stats.score, 
        player_o_stats.player, 
        player_o_stats.score
    );
}

fn play_game(player_x_stats: &mut PlayerStats, player_o_stats: &mut PlayerStats) {
    let mut game = Game::new();

    match game.play_game() {
        Some(Player::X) => player_x_stats.increment_score(),
        Some(Player::O) => player_o_stats.increment_score(),
        None => {}
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
