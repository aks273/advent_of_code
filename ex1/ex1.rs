use std::io::Error;
use std::fs;

fn number_of_increases(input_vec: &Vec::<u32>) -> u32 {
    let mut number_of_increases = 0;

    for i in 1..input_vec.len() {
        if input_vec[i] > input_vec[i - 1] {
            number_of_increases = number_of_increases + 1;
        }
    }

    number_of_increases
}

fn sliding_window(input_vec: &Vec::<u32>) -> u32 {
    // the final sliding window is centered on the penultimate element
    let max_index = input_vec.len() - 1;

    // the second sliding window (i.e. the first that we compare) 
    // is centered on the 3rd element
    let min_index = 2;

    let mut number_of_increases = 0;

    for i in min_index..max_index {
        // this works because:
        // previous_height = input_vec[i - 2] + input_vec[i - 1] + input_vec[i];
        // current_height = input_vec[i - 1] + input_vec[i] + input_vec[i + 1];
        
        if input_vec[i + 1] > input_vec[i - 2] {
            number_of_increases = number_of_increases + 1;
        }
    }
    
    number_of_increases
}

fn main() -> Result<(), Error> {
    let filepath = "./input.txt";

    let file_contents = fs::read_to_string(filepath)?;

    let file_contents_vec: Vec::<u32> = file_contents
        .split("\n")
        .map(|s| s.parse().expect("Failed to convert &str to u32"))
        .collect();

    // we do immutable borrows of our file_contents_vec 
    // so we can use it multiple times
    println!("Number of increases = {}", number_of_increases(&file_contents_vec));
    println!("Sliding window increases = {}", sliding_window(&file_contents_vec));

    Ok(())
}
