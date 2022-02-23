mod answer_words;
mod guess_words;

pub use answer_words::ANSWER_WORDS;
pub use guess_words::GUESS_WORDS;

// #[derive(Debug, Deserialize)]
// pub struct WordLists {
//     pub guess_words: [&str; 12972],
//     pub answer_words: [&str; 2309],
// }

#[derive(Debug)]
pub struct Word(pub String);

impl Word {
    pub const LENGTH: usize = 5;
    pub fn from_str(str: String) -> Result<Word, WordErr> {
        let len = str.len();
        if len > Word::LENGTH {
            Err(WordErr::TooLongErr)
        } else if len < Word::LENGTH {
            Err(WordErr::TooShortErr)
        } else {
            Ok(Word(str))
        }
    }
}

pub enum WordErr {
    TooLongErr,
    TooShortErr,
}

// pub fn get_word_lists() -> WordLists {
//     let file_text = fs::read_to_string("src/words.json").expect("Unable to read file");

//     let words: WordLists = serde_json::from_str(&file_text).expect("JSON was not well-formatted");

//     return words;
// }