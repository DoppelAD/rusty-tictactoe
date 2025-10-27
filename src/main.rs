mod colors;
mod game;
mod board;
mod player;
mod position;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.start_game();
}

