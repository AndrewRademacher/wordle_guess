use std::io::stdin;

use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::dictionary::parse_words;
use crate::guess::{parse_guess, Guess, Response};

pub struct App {
    possible_words: Vec<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            possible_words: parse_words(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Options: {}", self.possible_words.len());
        println!();
        let mut line = String::new();
        loop {
            println!("Enter guess line:\n");
            line.clear();
            stdin().read_line(&mut line)?;
            line.remove(line.len() - 1);
            self.handle_guess(parse_guess(&line)?);
            self.show_next_options();
        }
    }

    pub fn handle_guess(&mut self, guess: Guess) {
        // self.filter_not_present(&guess);
        self.filter_wrong_place_contains(&guess);
        self.filter_wrong_place_position(&guess);
        self.filter_correct(&guess);
    }

    pub fn filter_not_present(&mut self, guess: &Guess) {
        let nps = guess
            .0
            .iter()
            .filter(|x| x.response == Response::NotPresent)
            .collect::<Vec<_>>();

        let new_words = self
            .possible_words
            .iter()
            .filter(|w| {
                !nps.iter()
                    .map(|l| w.contains(l.letter))
                    .fold(false, |a, v| a || v)
            })
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        self.possible_words = new_words;
    }

    pub fn filter_wrong_place_contains(&mut self, guess: &Guess) {
        let wp = guess
            .0
            .iter()
            .filter(|x| x.response == Response::WrongPlace)
            .collect::<Vec<_>>();

        let new_words = self
            .possible_words
            .iter()
            .filter(|w| {
                wp.iter()
                    .map(|l| w.contains(l.letter))
                    .fold(true, |a, v| a && v)
            })
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        self.possible_words = new_words;
    }

    pub fn filter_wrong_place_position(&mut self, guess: &Guess) {
        for (pos, letter) in guess.0.iter().enumerate() {
            if letter.response == Response::WrongPlace {
                let new_words = self
                    .possible_words
                    .iter()
                    .filter(|w| {
                        for (comp_pos, char) in w.chars().enumerate() {
                            if comp_pos != pos && char == letter.letter {
                                return true;
                            }
                        }
                        false
                    })
                    .map(|x| x.clone())
                    .collect();
                self.possible_words = new_words;
            }
        }
    }

    pub fn filter_correct(&mut self, guess: &Guess) {
        for (pos, letter) in guess.0.iter().enumerate() {
            if letter.response == Response::Correct {
                let new_words = self
                    .possible_words
                    .iter()
                    .filter(|w| {
                        for (comp_pos, char) in w.chars().enumerate() {
                            if comp_pos == pos && char == letter.letter {
                                return true;
                            }
                        }
                        false
                    })
                    .map(|x| x.clone())
                    .collect();
                self.possible_words = new_words;
            }
        }
    }

    pub fn show_next_options(&self) {
        println!("Remaining: {}", self.possible_words.len());
        println!("------------------");
        let mut display = self.possible_words.clone();
        display.shuffle(&mut thread_rng());
        for word in display.iter().take(5) {
            println!("{}", word)
        }
        println!();
    }
}
