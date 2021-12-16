use std::fs;
use std::io::Error;

const FILEPATH: &str = "input.txt";

// This returns an array of the number of fish with 0 -> 8 days left until they
// birth a new fish.
fn get_file_input() -> Result<[u128; 9], Error> {
    let input_str = fs::read_to_string(FILEPATH)?;

    let input_vec: Vec::<usize> = input_str
        .split(",")
        .map(|s| s.parse().expect("Failed to convert &str to u32"))
        .collect();

    let mut bucket_array: [u128; 9] = [0; 9];

    for i in input_vec {
        bucket_array[i] += 1
    }

    Ok(bucket_array)
}

fn update_fish_number(fish_array: &mut [u128; 9]) {
    let number_of_new_fish = fish_array[0];

    for i in 1..9 {
        fish_array[i - 1] = fish_array[i];
    }
    fish_array[6] += number_of_new_fish;
    fish_array[8] = number_of_new_fish;
}

fn main() -> Result<(), Error> {
    let mut fish_array = get_file_input()?;

    for i in 0..256 {
        if i == 80 {
            let sum: u128 = fish_array.iter().sum();
            println!("Number of fish after 80 days = {}", sum);
        }
        
        update_fish_number(&mut fish_array);
    }

    let sum: u128 = fish_array.iter().sum();
    println!("Number of fish after 256 days {}", sum);

    Ok(())
}