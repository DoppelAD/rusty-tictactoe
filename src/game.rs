use std::io;

use crate::board::Board;
use crate::colors::{GREEN, PURPLE, RED, colored_text};
use crate::player::Player;
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

    pub fn play_game(&mut self) -> Option<Player> {
        self.print_game_header();

        loop {
            let position = self.read_input();

            self.handle_player_move(&position);

            if let Some(player) = self.handle_winner() {
                break Some(player);
            }

            if self.handle_draw() {
                break None;
            }
        }
    }

    fn print_game_header(&self) {
        println!(
            "{}\n RuStY TiCTaCToE! \n{}\n{}",
            colored_text("==================", PURPLE),
            colored_text("==================", PURPLE),
            self.board
        );
    }

    fn read_input(&self) -> Position {
        let mut player_input = String::new();

        println!(
            "|> {} <| Enter your move (row, column): ",
            self.current_turn.unwrap()
        );

        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line!");

        self.format_player_input(&player_input)
    }

    fn format_player_input(&self, player_input: &str) -> Position {
        let mut nums = player_input
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or(0));
        let (row, column) = (nums.next().unwrap_or(0), nums.next().unwrap_or(0));

        return Position::new(row, column)
    }

    fn handle_player_move(&mut self, position: &Position) {
        match self.board.make_move(position, self.current_turn.unwrap()) {
            Ok(_) => {
                println!("\n{}", self.board);
                self.switch_players();
            }
            Err(e) => println!(
                "{} at ({},{}): {:?}",
                colored_text("Error", RED),
                position.row,
                position.column,
                e
            ),
        }
    }

    fn switch_players(&mut self) {
        match self.current_turn {
            Some(Player::X) => self.current_turn = Some(Player::O),
            Some(Player::O) => self.current_turn = Some(Player::X),
            None => {}
        }
    }

    fn handle_winner(&self) -> Option<Player> {
        match self.board.check_winner() {
            Some(player) => {
                println!(
                    "{}\n Winner: {}\n{}",
                    colored_text("===========", GREEN),
                    player,
                    colored_text("===========", GREEN)
                );
                return Some(player);
            }
            None => return None,
        }
    }

    fn handle_draw(&self) -> bool {
        if self.board.is_full() {
            println!(
                "{}\n Draw :(\n{}",
                colored_text("===========", PURPLE),
                colored_text("===========", PURPLE)
            );
            return true;
        }
        return false;
    }
}
