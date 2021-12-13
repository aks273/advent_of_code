use std::fs;
use std::io::Error;

const FILEPATH: &str = "input.txt";
const BIT_LENGTH: usize = 12;
const FILE_LENGTH: u32 = 1000;

fn generate_input_vec() -> Result<Vec::<u32>, Error> {
    let file_contents = fs::read_to_string(FILEPATH)?;

    let input_vec: Vec::<u32> = file_contents
        .split("\n")
        .map(|x| isize::from_str_radix(x, 2).unwrap() as u32)
        .collect();

    Ok(input_vec)
}

fn count_bit_columns(vec: &[u32]) -> [u32; 12] {
    let mut final_array: [u32; BIT_LENGTH] = [0; BIT_LENGTH];
    
    // there are probably nicer ways to do this character by character, but i guess this
    // is O(N) because we just scan through once which is okay
    for line in vec {
        for i in 0..BIT_LENGTH {
            if line & (1 << (BIT_LENGTH - i - 1)) != 0 {
                final_array[i] += 1;
            }
        }
    }

    final_array
}

// Building a new bit string seems a bit shit here I imagine there are nicer ways to do
// both steps to part 1 than this
fn generate_gamma_epsilon_vals(final_array: [u32; 12]) -> (isize, isize) {
    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();

    for val in final_array.iter() {
        if *val > FILE_LENGTH / 2 {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }

    let gamma_val = isize::from_str_radix(&gamma_binary[..], 2).unwrap();
    let epsilon_val = isize::from_str_radix(&epsilon_binary[..], 2).unwrap();

    println!("Gamma value = {}", gamma_val);
    println!("Epsilon value = {}", epsilon_val);

    (gamma_val, epsilon_val)
}

// helper methods which return a vec of the values with zero/one at a given
// bit index
fn get_zero_bit_vec(vec: &Vec::<u32>, bit_index: usize) -> Vec::<u32> {
    vec.clone()
        .into_iter()
        .filter(|x| *x & 1 << (BIT_LENGTH - bit_index - 1) == 0)
        .collect()
}

fn get_one_bit_vec(vec: &Vec::<u32>, bit_index: usize) -> Vec::<u32> {
    vec.clone()
        .into_iter()
        .filter(|x| *x & 1 << (BIT_LENGTH - bit_index - 1) != 0)
        .collect()
}


fn get_oxygen_generator_rating(input_vec: &mut Vec::<u32>) -> Option<u32> {
    for i in 0..BIT_LENGTH {
        let zeros_vec: Vec::<u32> = get_zero_bit_vec(input_vec, i);
        let ones_vec = get_one_bit_vec(input_vec, i);

        if ones_vec.len() >= zeros_vec.len() {
            *input_vec = ones_vec;
        } else {
            *input_vec = zeros_vec;
        }

        if input_vec.len() == 1 {
            println!("Oxygen Generator Rating = {}", input_vec[0]);
            return Some(input_vec[0])
        }
    }

    println!("We haven't taken enough measurements to determine the Oxygen Generator Rating");
    None
}

// This is basically a copy paste of the function above.
// I could probably split these two out into a "most" and "least" common bit search
fn get_co2_scubber_rating(input_vec: &mut Vec::<u32>) -> Option<u32> {
    for i in 0..BIT_LENGTH {
        let zeros_vec: Vec::<u32> = get_zero_bit_vec(input_vec, i);
        let ones_vec = get_one_bit_vec(input_vec, i);

        if zeros_vec.len() <= ones_vec.len() {
            *input_vec = zeros_vec;
        } else {
            *input_vec = ones_vec;
        }

        if input_vec.len() == 1 {
            println!("CO2 Scrubber Rating {}", input_vec[0]);
            return Some(input_vec[0])
        }
    }

    println!("We haven't taken enough measurements to determine the CO2 Scrubber Rating");
    None
}

fn main() -> Result<(), Error> {
    let input_vec = generate_input_vec()?;

    println!("Part 1!");

    let bit_column_count = count_bit_columns(&input_vec);
    let (gamma_val, epsilon_val) = generate_gamma_epsilon_vals(bit_column_count);
    println!("Power consumption of submarine = {}", gamma_val * epsilon_val);

    println!("\nPart 2!");

    if let (Some(oxygen_rating), Some(co2_rating)) = (
        get_oxygen_generator_rating(&mut input_vec.clone()),
        get_co2_scubber_rating(&mut input_vec.clone())
    ) {
        println!("The life support rating is {}", oxygen_rating * co2_rating);
    }
    
    Ok(())
}