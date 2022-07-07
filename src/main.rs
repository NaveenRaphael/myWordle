use my_wordle::*;
use std::io;

fn main() {
    println!("Welcome to Raph's wordle helper!\n I won't tell you which word to use, but I can tell you if a guess you are about to make does not contradict your previous guesses!\n-----\nEnter the number of letters");
    let inp = || {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        return String::from(input.trim());
    };
    let n: usize;

    loop {
        match inp().parse() {
            Ok(num) => {
                n = num;
                break;
            }
            Err(e) => {
                println!("Not a number! Repeat!, {}", e);
            }
        }
    }

    let mut game = WordleGame::init(n);

    loop {
        println!("What do you want to do?\na. Add new guess\nb. Check legality of guess\nc. Debug\nd. Exit");

        match inp().as_str() {
            "a" | "A" | "1" => {
                println!("Input new word");
                let new_word = inp();

                println!("Enter wordle result");
                let wordle_result = inp();
                game.update(new_word.as_str(), wordle_result.as_str())
            }

            "b" | "B" | "2" => {
                println!("Enter the word you want to guess");
                let guess = inp();
                match game.check(guess.as_str()) {
                    Ok(_) => {
                        println!("No issues!")
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
            "c" | "C" | "3" => {
                println!("{}", game);
            }
            "d" | "D" | "4" => {
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
    println!("Very cool!")
}
