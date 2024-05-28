use crate::words::{Word, WordErr};
// Import (via `use`) the `fmt` module to make it available.
use ansi_term::Color::{Fixed, White};
use ansi_term::Style;
use std::fmt;
use std::io::{stdout, Write};
use std::{thread, time::Duration};

use crate::words::GUESS_WORDS;

pub struct Guess {
    pub word: Word,
    pub hints: [Hint; Word::LENGTH],
}

impl Guess {
    pub fn new(str: String, answer: &str) -> Result<Guess, GuessErr> {
        let word = Word::from_str(str);
        match word {
            Ok(word) => {
                if GUESS_WORDS.contains(&word.0.as_str()) {
                    let hints = get_hints(&word, answer);
                    let guess = Guess { word, hints };
                    Ok(guess)
                } else {
                    Err(GuessErr::NotInWordListErr)
                }
            }
            Err(err) => Err(GuessErr::WordErr(err)),
        }
    }
    pub fn display_hint(&self, i: usize) {
        let word = self.word.0.to_uppercase();
        let mut result = String::new();
        let hint = &self.hints[i];
        let bytes = vec![32, word.as_bytes()[i], 32];
        let letter = std::str::from_utf8(&bytes).expect("");
        let formatted = hint.style(letter);
        result += &formatted;
        result += &" ";
        print!("{}", result);
        let _ = stdout().flush();
    }

    pub fn display(&self) {
        for i in 0..self.word.0.len() {
            self.display_hint(i);
            thread::sleep(Duration::from_millis(400));
        }
        println!();
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut result = String::new();
        let word = self.word.0.to_uppercase();
        for i in 0..word.len() {
            let hint = &self.hints[i];
            let bytes = vec![32, word.as_bytes()[i], 32];
            let letter = std::str::from_utf8(&bytes).expect("");
            let formatted = hint.style(letter);
            result += &formatted;
            result += &" ";
        }

        write!(f, "{}", result)
    }
}

fn get_hints(word: &Word, answer: &str) -> [Hint; Word::LENGTH] {
    let mut hints: [Hint; Word::LENGTH] = [Hint::Wrong; Word::LENGTH];
    let word = &word.0;

    let mut used = [false; Word::LENGTH];

    // Check for all letters that are in the right places
    for i in 0..answer.len() {
        let at_same_index = word.as_bytes()[i] as char;
        if at_same_index == answer.as_bytes()[i] as char {
            hints[i] = Hint::Correct;
            used[i] = true;
        }
    }

    // Then check the same letter at other places
    for i in 0..answer.len() {
        if used[i] {
            // skip over the letters that are in the right place
            continue;
        }
        for j in 0..word.len() {
            if used[j] {
                // println!("continue because {} is used", i);
                // skip over letters that already matched with answer
                continue;
            }

            let at_index = word.as_bytes()[j] as char;
            let letter = answer.as_bytes()[i] as char;
            // println!("a:{} g:{}", at_index, letter);
            if at_index == letter {
                hints[j] = Hint::Elsewhere;
                used[j] = true;
                break;
            }
        }
    }

    hints
}

pub enum GuessErr {
    WordErr(WordErr),
    NotInWordListErr,
}

#[derive(Debug, Copy, Clone)]
pub enum Hint {
    Wrong,
    Elsewhere,
    Correct,
}

impl Hint {
    pub fn style(&self, str: &str) -> String {
        match &self {
            Hint::Wrong => Style::new()
                .fg(White)
                .on(Fixed(8))
                .bold()
                .paint(str)
                .to_string(),
            Hint::Elsewhere => Style::new()
                .fg(White)
                .on(Fixed(178))
                .bold()
                .paint(str)
                .to_string(),
            Hint::Correct => Style::new()
                .fg(White)
                .on(Fixed(70))
                .bold()
                .paint(str)
                .to_string(),
        }
    }
}
