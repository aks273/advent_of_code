use std::fs;
use std::io::Error;

const FILEPATH: &str = "input.txt";

// string methods to determine which code is which
fn contains_str(long_str: &str, short_str: &str) -> bool {
    for char in short_str.chars() {
        if !long_str.to_string().contains(char) {
            return false;
        }
    }

    true
}

fn is_same_len(first_str: &str, second_str: &str) -> bool {
    first_str.len() == second_str.len()
}

fn is_same_str(first_str: &str, second_str: &str) -> bool {
    is_same_len(first_str, second_str) && contains_str(first_str, second_str)
}

struct SevenDisplayInput {
    string: String,
}

impl SevenDisplayInput {
    fn create(input_str: &str) -> SevenDisplayInput {
        SevenDisplayInput {
            string: input_str[..].to_string()
        }
    }

    // "public" interface
    pub fn get_unique_length_nums(&self) -> u32 {
        let mut number_unique_length_chars = 0;
        for num in self.numbers() {
            if is_same_len(num, self.code_1()) || 
                is_same_len(num, self.code_4()) ||
                is_same_len(num, self.code_7()) ||
                is_same_len(num, self.code_8()) {
                    number_unique_length_chars += 1;
                }
        }

        number_unique_length_chars
    }

    pub fn parse_numbers(&self) -> u32 {
        // Calculate the codes vec once as opposed to doing it for every iteration
        // of the nested for loop
        let codes_vec = self.codes_vec();

        let mut total_number: u32 = 0;

        for i in 0..=3 {
            for j in 0..=9 {
                if is_same_str(codes_vec[j], self.numbers()[3 - i]) {
                    total_number += u32::pow(10, i as u32) * (j as u32);
                }
            }
        }

        total_number
    }

    // These are all "private" methods that will be used by the public methods above
    fn codes(&self) -> Vec::<&str> {
        self.string[..58].split(" ").collect()
    }

    fn numbers(&self) -> Vec::<&str> {
        self.string[61..].split(" ").collect()
    }

    fn code_0(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 6 
                && !contains_str(x, self.code_4()) 
                && contains_str(x, self.code_1())
        }).unwrap()
    }

    fn code_1(&self) -> &str {
        self.codes().iter().find(|x| x.len() == 2).unwrap()
    }

    fn code_2(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 5 
                && !contains_str(self.code_6(), x) 
                && !is_same_str(x, self.code_3())
        }).unwrap()
    }

    fn code_3(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 5 && contains_str(x, self.code_1())
        }).unwrap()
    }

    fn code_4(&self) -> &str {
        self.codes().iter().find(|x| x.len() == 4).unwrap()
    }

    fn code_5(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 5 && contains_str(self.code_6(), x)
        }).unwrap()
    }

    fn code_6(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 6 && !contains_str(x, self.code_1())
        }).unwrap()
    }

    fn code_7(&self) -> &str {
        self.codes().iter().find(|x| x.len() == 3).unwrap()
    }

    fn code_8(&self) -> &str {
        self.codes().iter().find(|x| x.len() == 7).unwrap()
    }

    fn code_9(&self) -> &str {
        self.codes().iter().find(|x| {
            x.len() == 6 && contains_str(x, self.code_4())
        }).unwrap()
    }

    // We only call this function once per line segment, so it is okay.
    //
    // However we could make this faster by iterating through the codes just once
    // and filling in a vec. This is currently slow because we iterate through
    // self.codes() once for each number, as opposed to just once.
    fn codes_vec(&self) -> Vec::<&str> {
        return vec![
            self.code_0(),
            self.code_1(),
            self.code_2(),
            self.code_3(),
            self.code_4(),
            self.code_5(),
            self.code_6(),
            self.code_7(),
            self.code_8(),
            self.code_9(),
        ]
    }
}

fn parse_input() -> Result<Vec::<SevenDisplayInput>, Error> {
    let input_str = fs::read_to_string(FILEPATH)?;

    let input_vec: Vec::<SevenDisplayInput> = input_str
        .split("\n")
        .map(|x| SevenDisplayInput::create(x))
        .collect();

    Ok(input_vec)
}

fn main() -> Result<(), Error> {
    let input_vec = parse_input()?;
    let mut number_unique_length_chars = 0;
    let mut total_sum = 0;

    for string in input_vec {
        number_unique_length_chars += string.get_unique_length_nums();
        total_sum += string.parse_numbers();
    }

    println!("Part 1\nNumber of 1s, 4s, 7s, 8s = {}\n", number_unique_length_chars);
    println!("Part 2\nTotal sum of seven display inputs = {}\n", total_sum);
    
    Ok(())
}