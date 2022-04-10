use crate::dictionary::parse_words;
use crate::guess::{Guess, Response};

pub struct GameState {
    possible_words: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            possible_words: parse_words(),
        }
    }

    pub fn iter_remaining(&self) -> impl Iterator<Item = &String> {
        self.possible_words.iter()
    }

    pub fn possible_words(&self) -> &Vec<String> {
        &self.possible_words
    }

    pub fn handle_guess(&mut self, guess: Guess) {
        self.filter_not_present(&guess);
        self.filter_wrong_place_contains(&guess);
        self.filter_wrong_place_position(&guess);
        self.filter_correct(&guess);
    }

    pub fn filter_not_present(&mut self, guess: &Guess) {
        let correct = guess
            .0
            .iter()
            .filter(|x| x.response == Response::Correct)
            .map(|x| x.letter)
            .collect::<Vec<_>>();
        let wrong_place = guess
            .0
            .iter()
            .filter(|x| x.response == Response::WrongPlace)
            .map(|x| x.letter)
            .collect::<Vec<_>>();
        let nps = guess
            .0
            .iter()
            .filter(|x| x.response == Response::NotPresent)
            .filter(|x| !correct.contains(&x.letter))
            .filter(|x| !wrong_place.contains(&x.letter))
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
}
