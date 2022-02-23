mod guess;
mod words;

use guess::{Guess, GuessErr};
use words::{WordErr};
use rand::Rng;
use std::io::{stdin,stdout,Write};

use ansi_term::Style;
use ansi_term::Color::Fixed;

fn main() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // let enabled = ansi_term::enable_ansi_support();
  
    let word_lists = words::get_word_lists();
  
    let rand_index = rand::thread_rng().gen_range(0..word_lists.answer_words.len());
    let answer: &String = &word_lists.answer_words[rand_index];

  
    println!("⬛🟨🟩 Wordle in Rust\n");

    let max_guesses = 6;
    let mut guesses = 0;

    while guesses < max_guesses {
      
        let mut guess = String::new();

      print!("{}", Style::new().fg(Fixed(246)).paint("💭 Enter your guess: ").to_string());
      let _=stdout().flush();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
      
        guess = guess.trim().to_string();

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
              println!("{}", guess);

              if guess.word.0.eq(answer) {
                println!("\n🎉 Nice! You got it in {} {}.", guesses, if guesses == 1 {"try"} else {"tries"});
          println!("\n🚥 Hit enter to play again or press Ctrl+C to quit.");
                let mut s = String::new();
                        stdin()
            .read_line(&mut s)
            .expect("Failed to read line");
                main();
              }
            }
        }
    }

    println!("\n❌ Game over, the answer was {}", answer);
          println!("\n🚥 Hit enter to play again or press Ctrl+C to quit.");
        let mut s = String::new();
                stdin()
    .read_line(&mut s)
    .expect("Failed to read line");
  main();
}
