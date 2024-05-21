mod guess;
mod words;

use guess::{Guess, GuessErr};
use rand::Rng;
use std::io::{stdin, stdout, Write};
use words::{WordErr, ANSWER_WORDS};

use ansi_term::Color::Fixed;
use ansi_term::Style;

fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            // ANSI escape codes were successfully enabled, or this is a non-Windows platform.
        }
        Err(_) => {
            // The operation was unsuccessful, typically because it's running on an older
            // version of Windows. The program may choose to disable ANSI color code output in
            // this case.
            panic!("Your OS doesn't support colored terminal output.");
        }
    }
    // Use your terminal color library of choice here.
    // let enabled = ansi_term::enable_ansi_support();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let rand_index = rand::thread_rng().gen_range(0..ANSWER_WORDS.len());
    let answer: &str = &ANSWER_WORDS[rand_index];

    println!(
        "{} in {} 🦀",
        Style::new()
            .bold()
            .fg(Fixed(0))
            .on(Fixed(15))
            .paint(" Wordle "),
        Style::new().bold().fg(Fixed(166)).paint("Rust")
    );
    println!("{}\n", Style::new().fg(Fixed(248)).paint("by @notken12"));

    let max_guesses = 6;
    let mut guesses = 0;

    while guesses < max_guesses {
        let mut guess = String::new();

        print!(
            "{}",
            Style::new()
                .fg(Fixed(246))
                .paint("💭 Enter your guess: ")
                .to_string()
        );
        let _ = stdout().flush();
        stdin().read_line(&mut guess).expect("Failed to read line");

        guess = guess.trim().to_string().to_lowercase();

        let guess_result = Guess::new(guess, &answer);

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
                guess.display();

                if guess.word.0.eq(answer) {
                    println!(
                        "\n🎉 Nice! You got it in {} {}.",
                        guesses,
                        if guesses == 1 { "try" } else { "tries" }
                    );
                    println!("\n🚥 Hit enter to play again or press Ctrl+C to quit.");
                    let mut s = String::new();
                    stdin().read_line(&mut s).expect("Failed to read line");
                    main();
                }
            }
        }
    }

    println!("\n❌ Game over, the answer was {}", answer);
    println!("\n🚥 Hit enter to play again or press Ctrl+C to quit.");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    main();
}
