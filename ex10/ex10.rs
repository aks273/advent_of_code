use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "input.txt";

const OPEN_BRACKETS: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_BRACKETS: [char; 4] = [')', ']', '}', '>'];

const ILLEGAL_SCORES: [u32; 4] = [3, 57, 1197, 25137];
const CLOSING_CHAR_ADDITIONS: [u64; 4] = [1, 2, 3, 4];

fn get_file_input() -> Vec::<String> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut input_file = Vec::<String>::new();

    for line in reader.lines() {
        input_file.push(line.unwrap());
    }

    input_file
}

fn update_illegal_score(char: char) -> u32 {
    if let Some(index) = CLOSE_BRACKETS.iter().position(|c| *c == char) {
        return ILLEGAL_SCORES[index]
    }
    
    panic!("Trying to update illegal score for an invalid character!")
}

fn generate_closing_char_score(string: String) -> u64 {
    let mut closing_char_score: u64 = 0;

    for char in string.chars() {
        closing_char_score *= 5;

        if let Some(index) = CLOSE_BRACKETS.iter().position(|c| *c == char) {
            closing_char_score += CLOSING_CHAR_ADDITIONS[index];
        } else {
            panic!("Trying to update the closing pattern score for invalid character!")
        }
    }

    closing_char_score
}

fn main() {
    let chunks = get_file_input();

    let mut illegal_score: u32 = 0;

    let mut closing_char_scores = Vec::<u64>::new();

    'line_loop: for chunk in chunks {
        let mut close_pattern = String::from("");

        for char in chunk.chars() {
            if let Some(index) = OPEN_BRACKETS.iter().position(|c| *c == char) {
                // we've found another opening bracket, the close_pattern needs to be updated
                close_pattern.insert(0, CLOSE_BRACKETS[index]);
            } else if close_pattern.chars().nth(0).unwrap() == char {
                // we've found the first element of the close pattern, remove it
                close_pattern.remove(0);
            } else  {
                // Syntax error!
                // println!("Expected {}, but found {} instead", &close_pattern[..1], char);
                illegal_score += update_illegal_score(char);
                continue 'line_loop;
            }
        }

        closing_char_scores.push(generate_closing_char_score(close_pattern));
    }

    println!("Part 1: Illegal score = {}", illegal_score);

    closing_char_scores.sort();
    let median_closing_score = closing_char_scores[closing_char_scores.len() / 2];
    println!("Part 2: Median closing char score = {}", median_closing_score);
}