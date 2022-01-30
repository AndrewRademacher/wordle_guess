const DICTIONARY: &'static str = include_str!("../vendor/english-words/words_alpha.txt");

pub fn parse_words() -> Vec<String> {
    DICTIONARY
        .split("\r\n")
        .filter(|x| x.len() == 5)
        .map(|x| x.to_string())
        .collect()
}
