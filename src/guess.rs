use anyhow::{anyhow, bail, Result};

pub struct Guess(pub [Letter; 5]);

impl Default for Guess {
    fn default() -> Self {
        Guess([
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ])
    }
}

pub struct Letter {
    pub letter: char,
    pub response: Response,
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            letter: 'a',
            response: Response::NotPresent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Response {
    Correct,
    WrongPlace,
    NotPresent,
}

// Guess Lines:
//      limit-nwnnn
//      quilt-nwncn
pub fn parse_guess(line: &str) -> Result<Guess> {
    let mut split = line.split('-').map(|x| x.to_string());
    let letters = split.next().ok_or(anyhow!("No '-' split."))?;
    let responses = split.next().ok_or(anyhow!("No following reponse."))?;

    if letters.len() != 5 {
        bail!("Must have 5 letters.");
    }

    if responses.len() != 5 {
        bail!("Must have 5 responses.");
    }

    let letters = letters.chars().collect::<Vec<_>>();
    let responses = responses.chars().collect::<Vec<_>>();

    let mut guess = Guess::default();
    for i in 0..5 {
        guess.0[i] = Letter {
            letter: letters[i],
            response: parse_response(responses[i])?,
        }
    }

    Ok(guess)
}

fn parse_response(response: char) -> Result<Response> {
    match response {
        'c' => Ok(Response::Correct),
        'w' => Ok(Response::WrongPlace),
        'n' => Ok(Response::NotPresent),
        _ => Err(anyhow!("{} is not a valid response.", response)),
    }
}

#[cfg(test)]
mod test {
    use crate::guess::{parse_guess, Response};

    #[test]
    fn test_basic() {
        let guess = parse_guess("quilt-nwncn").unwrap();
        assert_eq!(guess.0[0].letter, 'q');
        assert_eq!(guess.0[1].letter, 'u');
        assert_eq!(guess.0[2].letter, 'i');
        assert_eq!(guess.0[3].letter, 'l');
        assert_eq!(guess.0[4].letter, 't');

        assert_eq!(guess.0[0].response, Response::NotPresent);
        assert_eq!(guess.0[1].response, Response::WrongPlace);
        assert_eq!(guess.0[2].response, Response::NotPresent);
        assert_eq!(guess.0[3].response, Response::Correct);
        assert_eq!(guess.0[4].response, Response::NotPresent);
    }
}
