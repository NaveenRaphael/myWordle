use std::collections::HashMap;

///PresentTypes is an enumeration of the type of letter if present in a wordle game.
///
/// ## Cases
/// 1. No: The letter is not at this position
/// 2. Maybe: There is no information yet about the letter at this position
/// 3. Yes: This letter does appear here, tho, this is not indicative that the letter cannot appear elsewhere
#[derive(Clone)]
enum PresentTypes {
    No,
    Maybe,
    Yes,
}

impl std::fmt::Display for PresentTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PresentTypes::No => "n",
                PresentTypes::Maybe => "m",
                PresentTypes::Yes => "y",
            }
        )
    }
}

///LetterInfo is the enumeration storing all the information of a specific letter.
///
/// A letter is either absent, or present; if present, we store the locational data also in a vector
///
/// ## Cases:
/// 1. Absent
/// 2. Present: Also has a vector of positional information, See PresentTypes
enum LetterInfo {
    Absent,
    Present(Vec<PresentTypes>),
}

impl std::fmt::Display for LetterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LetterInfo::Absent => String::new(),
                LetterInfo::Present(p) => String::from_iter(p.iter().map(|x| x.to_string())),
            }
        )
    }
}

///GuessResult is an enum of the possible information wordle gives us after a guess
///
/// ## Cases:
/// 1. No: the letter is not present-> Black square
/// 2. Yes: the letter is at that location -> Green Square
/// 3. Somewhere: the letter is elsewhere -> Yellow Square
enum GuessResult {
    No,
    Yes,
    Somewhere,
}

impl GuessResult {
    fn from_char(c: char) -> GuessResult {
        match c {
            'y' => GuessResult::Yes,
            'm' => GuessResult::Somewhere,
            _ => GuessResult::No,
        }
    }
}

/// Generates a new vector for LetterType, with starting information
///
/// ## Arguments:
/// n: usize; Length of vector
/// p: usize; Position of first data
/// result: GuessResult; for starting information
///
/// ##Examples:
/// generate_new_vec(n: 4, p: 2, result: GuessResult::Somewhere) -> [Maybe, Maybe, No, Maybe]
fn generate_new_vec(n: usize, p: usize, result: &GuessResult) -> Vec<PresentTypes> {
    let mut r = vec![PresentTypes::Maybe; n];
    r[p] = match result {
        GuessResult::No => {
            //This should never be called
            PresentTypes::No
        }
        GuessResult::Somewhere => PresentTypes::No,
        GuessResult::Yes => PresentTypes::Yes,
    };
    return r;
}

///
///  Converts a string to a vector of Guess Result, that is, you know, the black yellow and green of wordle to my enum implementation
///
/// ## Example:
/// word_to_result("ymnn")-> vec![Yes, Somewhere, No, No]
fn word_to_result(word: &str) -> Vec<GuessResult> {
    word.chars().map(|x| GuessResult::from_char(x)).collect()
}

pub struct WordleGame {
    perfect_guess_so_far: Vec<char>,
    information: HashMap<char, LetterInfo>,
    num: usize,
}

impl WordleGame {
    ///To make it cleaner, I guess
    pub fn init(num: usize) -> WordleGame {
        WordleGame {
            perfect_guess_so_far: vec!['*'; num],
            information: HashMap::new(),
            num,
        }
    }

    ///To check if a new word I am guessing is a wise guess
    pub fn check(&self, word: &str) -> Result<(), String> {
        if self.information.is_empty() {
            return Err(String::from("Errors: \nNo Information yet..."));
        };
        let mut e = String::from("Error:\n");
        let mut flag = false;

        if word.len() != self.num {
            e.push_str(
                format!("Number of letters are not right! (should be {}) \n", {
                    self.num
                })
                .as_str(),
            );
            return Err(e);
        }

        //? Check with the known values
        for (correct_letter, (pos, guess_letter)) in
            self.perfect_guess_so_far.iter().zip(word.char_indices())
        {
            if *correct_letter != '*' {
                if *correct_letter != guess_letter {
                    flag = true;
                    e.push_str(
                        format!(
                            "Error at position: {}; Expected letter `{}`, instead found `{}`\n",
                            pos, correct_letter, guess_letter
                        )
                        .as_str(),
                    );
                }
            }
        }
        //? Check with the Present Somewhere Values
        for (pos, letter) in word.char_indices() {
            match self.information.get(&letter) {
                Some(p) => match p {
                    LetterInfo::Absent => {
                        e.push_str(
                            format!(
                                "Error at position: {}; This letter should be absent: letter {}\n",
                                pos, letter
                            )
                            .as_str(),
                        );
                        flag = true
                    }
                    LetterInfo::Present(v) => {
                        if matches!(v[pos], PresentTypes::No) {
                            e.push_str(format!(
                                    "Error at position: {pos}; This letter should not be here: letter:{letter}\n"
                                ).as_str());
                            flag = true;
                        }
                    }
                },
                _ => {}
            }
        }

        if flag {
            Err(e)
        } else {
            Ok(())
        }
    }

    ///Used to set the other letters' positional data at No, when a Yes pops up
    fn set_other_keys(&mut self, pos: usize, letter: char) {
        self.perfect_guess_so_far[pos] = letter;
        for (key, p) in self.information.iter_mut() {
            if *key != letter {
                match p {
                    LetterInfo::Present(v) => {
                        v[pos] = PresentTypes::No;
                    }
                    _ => {}
                }
            }
        }
    }
    ///Updating the information given a new word and details
    ///
    /// ## Arguments:
    /// information: Old info
    /// guess_word: New word provided
    /// guess_result: What wordle provides
    pub fn update(&mut self, new_word: &str, wordle_result: &str) {
        let mut flag: (bool, Vec<char>, Vec<usize>) = (false, Vec::new(), Vec::new());
        let mut change_flag = |letter, pos| {
            flag.0 = true;
            flag.1.push(letter);
            flag.2.push(pos);
        };

        let wordle_result = word_to_result(wordle_result);

        for ((position, letter), result) in new_word.char_indices().zip(wordle_result.iter()) {
            match self.information.get_mut(&letter) {
                None => match result {
                    GuessResult::No => {
                        self.information.insert(letter, LetterInfo::Absent);
                    }
                    GuessResult::Somewhere | GuessResult::Yes => {
                        self.information.insert(
                            letter,
                            LetterInfo::Present(generate_new_vec(self.num, position, result)),
                        );
                        if matches!(result, GuessResult::Yes) {
                            change_flag(letter, position);
                        }
                    }
                },
                Some(p) => match (result, p) {
                    (GuessResult::No, LetterInfo::Absent) => {
                        //?
                        //? Guess Result No happens when this letter is either not present at all, or when it is not present as a repeated letter
                        //? Like Buttter :mmnnymm -> the first T is no
                        //? So, it inserts "T" absent into the hashmap
                        //? the second T is no, which triggers this condition
                        //?
                        //? In both cases, this is either a useless guess, or it will be fixed shortly
                        println!("Useless Guess?");
                    }
                    (GuessResult::No, LetterInfo::Present(v)) => {
                        //? Guess Result No and guess type present means that we do not have an additional one of this letter, in the word
                        for i in 0..self.num {
                            match v[i] {
                                PresentTypes::Yes | PresentTypes::No => {}
                                PresentTypes::Maybe => {
                                    v[i] = PresentTypes::No;
                                }
                            }
                        }
                    }
                    (GuessResult::Somewhere, LetterInfo::Absent) => {
                        //?This absolutely cannot happen i think?
                    }
                    (GuessResult::Somewhere, LetterInfo::Present(v)) => {
                        v[position] = PresentTypes::No;
                    }
                    (GuessResult::Yes, LetterInfo::Absent) => {
                        //?This happens in the case elaborated in the first case
                        let mut v = vec![PresentTypes::No; new_word.len()];
                        v[position] = PresentTypes::Yes;
                        change_flag(letter, position);
                        self.information.insert(letter, LetterInfo::Present(v));
                    }
                    (GuessResult::Yes, LetterInfo::Present(p)) => {
                        change_flag(letter, position);
                        p[position] = PresentTypes::Yes;
                    }
                },
            }
        }
        if flag.0 {
            for (letter, pos) in flag.1.iter().zip(flag.2.iter()) {
                self.set_other_keys(*pos, *letter);
            }
        }
    }
}

impl std::fmt::Display for WordleGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_absent = self
            .information
            .iter()
            .filter(|(_letter, info)| matches!(info, LetterInfo::Absent))
            .map(|(letter, _info)| format!("{}", letter))
            .fold(String::from("\nAbsentees"), |mut acc, x| {
                acc.push_str(", ");
                acc.push_str(x.as_str());
                acc
            });
        let all_present = self
            .information
            .iter()
            .filter(|(_letter, info)| !matches!(info, LetterInfo::Absent))
            .map(|(letter, info)| format!("{letter}->{}", info.to_string()))
            .fold(String::from(""), |mut acc, x| {
                acc.push_str("\n");
                acc.push_str(x.as_str());
                acc
            });

        write!(
            f,
            "Necessary letters: {:?}\nAdditional Info:{}\n{}",
            self.perfect_guess_so_far, all_absent, all_present
        )
    }
}
