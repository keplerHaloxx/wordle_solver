use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

fn main() {
    // - Reads valid words and puts them into a vector
    let reader =
        BufReader::new(File::open("./validwords.txt").expect("Could not open validwords.txt"));
    let mut valid_words: Vec<String> = vec![];

    for line in reader.lines() {
        valid_words.push(line.unwrap());
    }

    loop {
        let green_letters = get_green_letters();
        let yellow_letters = get_yellow_letters();
        let grey_letters = get_grey_letters();

        let new_words: Vec<&String> = valid_words
            .iter()
            .filter(|&word| validate_word(word, &green_letters, &yellow_letters, &grey_letters))
            .collect();

        println!();
        for i in new_words.chunks(5) {
            for (index, j) in i.iter().enumerate() {
                if index == i.len() - 1 {
                    println!("{j}");
                    break;
                }
                print!("{j}, ");
            }
        }
        get_input("\nPress enter to restart app."); // pause execution till this is finished
        Command::new("clear").status().unwrap(); // clear terminal
    }
}

fn validate_word(word: &str, green: &[char], yellow: &[(char, usize)], grey: &[char]) -> bool {
    let word_chars: Vec<char> = word.chars().collect();

    // green letters must match their correct position
    for (i, &green_char) in green.iter().enumerate() {
        if green_char != '_' && word_chars[i] != green_char {
            return false;
        }
    }

    // must not contain grey letters
    for &grey_char in grey.iter() {
        if word_chars.contains(&grey_char) && !green.contains(&grey_char) {
            return false;
        }
    }

    // yellow letters must be contained but not in the specified position
    for &(yellow_char, pos) in yellow.iter() {
        if !word_chars.contains(&yellow_char) || word_chars[pos] == yellow_char {
            return false;
        }
    }

    true
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    // Print prompt and flush stdout so it actually shows
    println!("{}", prompt);
    print!("> ");
    io::stdout().flush().unwrap();

    // Get input, returning the trimmed version
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_green_letters() -> Vec<char> {
    println!("Enter exactly 5 letters");
    let input_chars: Vec<char> =
        get_input("Enter green letters in order. Use '_' to indicate a space. e.g. AB_D_.")
            .chars()
            .collect();

    if input_chars.len() != 5 // Input has only 5 letters and consists of letters and '_'
        || !input_chars
            .iter()
            .all(|&c| c.is_ascii_alphabetic() || c == '_')
    {
        println!("Invalid input!");
        get_green_letters();
    }

    input_chars.iter().map(|c| c.to_ascii_lowercase()).collect() // return letters but lowercase
}

fn get_yellow_letters() -> Vec<(char, usize)> {
    // Get yellow letters input
    let input_chars: Vec<char> =
        get_input("Enter yellow letters. Type only the letters and nothing else (e.g. abc).")
            .chars()
            .collect();

    // Now, ask for the positions of these yellow letters
    let mut yellow_letters = Vec::new();
    for &letter in input_chars.iter() {
        let position_input = get_input(&format!(
            "Enter the position (1-5) where '{}' is NOT allowed:",
            letter
        ));
        let position: usize = match position_input.parse() {
            Ok(pos) if pos <= 5 && pos > 0 => pos,
            _ => {
                println!("Invalid position! Must be between 1 and 5.");
                return get_yellow_letters(); // Retry if invalid input
            }
        };

        yellow_letters.push((letter.to_ascii_lowercase(), position - 1));
    }

    yellow_letters
}

fn get_grey_letters() -> Vec<char> {
    get_input("Enter grey letters. Type only the letters and nothing else. e.g. xyz")
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_ascii_lowercase())
        .collect() // return letters but lowercase
}
