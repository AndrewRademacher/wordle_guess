use anyhow::Result;
use rand::prelude::*;
use std::io::stdin;

use wordle_guess::game_state::GameState;
use wordle_guess::guess::parse_guess;

fn main() -> Result<()> {
    Game::new().run()
}

struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.show_next_options();
        let mut line = String::new();
        loop {
            println!("Enter guess line:\n");
            line.clear();
            stdin().read_line(&mut line)?;
            line.remove(line.len() - 1);
            self.state.handle_guess(parse_guess(&line)?);
            self.show_next_options();
        }
    }

    pub fn show_next_options(&self) {
        println!("Remaining: {}", self.state.possible_words().len());
        println!("------------------");
        let mut display = self.state.possible_words().clone();
        display.shuffle(&mut thread_rng());
        for word in display.iter().take(5) {
            println!("{}", word)
        }
        println!();
    }
}
