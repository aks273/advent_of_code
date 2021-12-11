use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "./input.txt";

// Diagonals don't count!
const LINES: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
];

#[derive(Copy, Clone, Debug)]
struct CardEntry {
    number: u32,
    marked: bool,
}

struct Card {
    index: usize,
    entries: [CardEntry; 25],
    is_finished: bool,
}

fn setup_card(numbers: Vec::<u32>, card_number: usize) -> Card {
    if numbers.len() != 25 {
        panic!("Parsed input.txt incorrectly and have an incorrectly sized card. Aborting!");
    }

    let mut entries: [CardEntry; 25] = [CardEntry {number: 0, marked: false}; 25];

    for (i, num) in numbers.into_iter().enumerate() {
        entries[i].number = num;
    }

    Card {
        index: card_number - 1,
        entries: entries,
        is_finished: false
    }
}

impl Card {
    pub fn update(&mut self, number: u32) {
        if let Some(index) = self.contains(number) {
            self.entries[index].marked = true;
            if self.check_is_finished() && !self.is_finished {
                self.is_finished = true;
            }
        }
    }

    // returns the index of the number being asked to find, None if not present
    fn contains(&self, number: u32) -> Option<usize> {
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.number == number {
                return Some(i)
            }
        }

        None
    }

    fn check_is_finished(&self) -> bool {
        for line in LINES {
            if self.entries[line[0]].marked &&
                self.entries[line[1]].marked &&
                self.entries[line[2]].marked &&
                self.entries[line[3]].marked &&
                self.entries[line[4]].marked {
                return true
            }
        }

        false
    }
}

fn read_input_file() -> (Vec::<u32>, Vec::<Card>){
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut card_numbers: Vec::<u32> = Vec::new();
    let mut number_of_cards: usize = 0;

    let mut bingo_numbers: Vec::<u32> = Vec::new();
    let mut bingo_cards = Vec::<Card>::new();

    for (line_number, line) in reader.lines().enumerate() {
        // line 0 of the file, collect the bingo numbers
        if line_number == 0 {
            bingo_numbers = line
                .unwrap()
                .split(",")
                .map(|x| x.parse().expect("Failed to parse bingo numbers to u32"))
                .collect();
        }
        // line one of the file, we haven't parsed a bingo card just yet
        else if line_number == 1 {
            // do nothing
        }
        // every sixth line is a blank line where should have finished parsing
        // a new card
        else if (line_number - 1) % 6 == 0 {
            number_of_cards = number_of_cards + 1;
            bingo_cards.push(setup_card(card_numbers.clone(), number_of_cards));
            card_numbers = vec![];
        }
        // parse the next line of a bingo card
        else {
            let mut new_line: Vec::<u32> = line
                .unwrap()
                .split(" ")
                .filter(|x| x != &"")
                .map(|x| x.parse().expect("Failed to parse line of bingo card to u32"))
                .collect();

            card_numbers.append(&mut new_line);
        }
    }

    // add a final card if we have a valid bingo card
    if card_numbers.len() == 25 {
        number_of_cards = number_of_cards + 1;
        bingo_cards.push(setup_card(card_numbers.clone(), number_of_cards));
    }

    (bingo_numbers, bingo_cards)
}

fn calculate_unmarked_sum(card: &Card) -> u32 {
    let mut unmarked_sum = 0;
    for entry in card.entries {
        if !entry.marked {
            unmarked_sum = unmarked_sum + entry.number;
        }
    }

    unmarked_sum
}

fn play_bingo_game(bingo_numbers: Vec::<u32>, bingo_cards: &mut Vec::<Card>) {
    let mut unfinished_indices: Vec::<u32> = (0..100).collect();

    for (i, number) in bingo_numbers.iter().enumerate() {
        for card in &mut *bingo_cards {
            if card.is_finished {
                continue; // we don't want to update the card again
            }

            card.update(*number);
            if card.is_finished {
                // first card has won
                if unfinished_indices.len() == 100 {
                    print_output(&card, i, *number, true);
                }

                // last card has won
                if unfinished_indices.len() == 1 {
                    print_output(&card, i, *number, false);
                }

                // remove the finished index from the unfinished indices list
                unfinished_indices.retain(|x| *x != card.index as u32);
            }
        }
    }
}

fn print_output(card: &Card, turn: usize, final_number: u32, is_first: bool) {
    if is_first {
        println!("Card {} has finished first after turn {}!", card.index, turn);
    } else {
        println!("Card {} has finished last after turn {}!", card.index, turn);
    }

    let unmarked_sum = calculate_unmarked_sum(&card);
    println!("Final number = {}", final_number);
    println!("Unmarked sum = {}", unmarked_sum);
    println!("Multiplied, gives {}\n", final_number * unmarked_sum);
}

fn main() {
    let (bingo_numbers, mut bingo_cards) = read_input_file();
    play_bingo_game(bingo_numbers, &mut bingo_cards);
}