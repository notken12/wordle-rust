use crate::words::{Word, WordErr, WordLists};

pub struct Guess {
    pub word: Word,
    pub hints: [Hint; Word::LENGTH],
}

impl Guess {
    pub fn new(str: String, word_lists: &WordLists, answer: &String) -> Result<Guess, GuessErr> {
        let word = Word::from_str(str);
        match word {
            Ok(word) => {
                if word_lists.guess_words.contains(&word.0) {
                    let hints = get_hints(&word, &answer);
                    let guess = Guess { word, hints };
                    return Ok(guess);
                } else {
                    return Err(GuessErr::NotInWordListErr);
                }
            }
            Err(err) => Err(GuessErr::WordErr(err)),
        }
    }
}

fn get_hints(word: &Word, answer: &String) -> [Hint; Word::LENGTH] {
    let mut hints: [Hint; Word::LENGTH] = [Hint::Wrong; Word::LENGTH];
    let word = &word.0;

    let answer = answer.clone();
    let mut used = [false; Word::LENGTH];
    for i in 0..word.len() {
        // println!("{}", i);
        // First check the same index
        if !used[i] {
            let at_same_index = answer.as_bytes()[i];
            if at_same_index == word.as_bytes()[i] {
                hints[i] = Hint::Correct;
                used[i] = true;
                continue;
            }
        }

        let mut letter_found = false;

        // Then check the same letter at other places
        // println!("{}", answer.len());
        for j in 0..answer.len() {
            if used[j] {
                // println!("continue because {} is used", i);
                continue;
            }

            let at_index = answer.as_bytes()[j] as char;
            let letter = word.as_bytes()[i] as char;
            // println!("a:{} g:{}", at_index, letter);
            if at_index == letter {
                hints[i] = Hint::Elsewhere;
                used[j] = true;
                letter_found = true;
                break;
            }
        }

        // if !letter_found {
        //     // No match
        //     hints[i] = Hint::Wrong;
        // }
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
