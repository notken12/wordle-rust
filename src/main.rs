mod guess;
mod words;

use guess::{Guess, GuessErr};
use words::{WordErr};
use rand::Rng;
use std::io;

fn main() {
    let word_lists = words::get_word_lists();

    let rand_index = rand::thread_rng().gen_range(0..word_lists.answer_words.len());
    let answer: &String = &word_lists.answer_words[rand_index];

    println!("The answer is: {}", answer);

    let max_guesses = 6;
    let mut guesses = 0;

    while guesses < max_guesses {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.pop();
        guess.pop();

        let guess_result = Guess::new(guess, &word_lists, &answer);

        match guess_result {
            Err(err) => match err {
                GuessErr::WordErr(err) => match err {
                    WordErr::TooLongErr => println!("Too long!"),
                    WordErr::TooShortErr => println!("Too short!"),
                },
                GuessErr::NotInWordListErr => println!("Not in word list"),
            },
            Ok(guess) => {
                guesses += 1;
                println!("{:?}", guess.word);
                println!("{:?}", guess.hints);
            }
        }
    }
}
