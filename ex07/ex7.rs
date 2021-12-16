use std::fs;
use std::io::Error;

const FILEPATH: &str = "input.txt";

fn parse_input() -> Result<Vec::<i32>, Error> {
    let input_str = fs::read_to_string(FILEPATH)?;

    let input_vec: Vec::<i32> = input_str
        .split(",")
        .map(|x| x.parse().expect("Failed to cast to i32"))
        .collect();

    Ok(input_vec)
}

fn calculate_fuel(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

fn main() -> Result<(), Error> {
    let input_vec = parse_input()?;

    let max_value = *input_vec.iter().max().unwrap();

    let mut fuel_vec_1 = Vec::<i32>::new();
    let mut fuel_vec_2 = Vec::<i32>::new();
    
    for i in 0..=max_value {
        let mut fuel_1 = 0;
        let mut fuel_2 = 0;

        for input in &input_vec {
            fuel_1 += (i - input).abs();
            fuel_2 += calculate_fuel((i - input).abs());
        }

        fuel_vec_1.push(fuel_1);
        fuel_vec_2.push(fuel_2);
    }

    let min_1: i32 = *fuel_vec_1.iter().min().unwrap();
    println!("Part 1 - minimum fuel needed = {}", min_1);

    let min_2: i32 = *fuel_vec_2.iter().min().unwrap();
    println!("Part 2 - minimum fuel needed = {}", min_2);

    Ok(())
}