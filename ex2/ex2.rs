use std::fs;
use std::io::Error;

const FILEPATH: &str = "input.txt";

fn generate_input_vec() -> Result<Vec::<Move>, Error> {
    let file_contents = fs::read_to_string(FILEPATH)?;

    let moves_vec: Vec::<Move> = file_contents
        .split("\n")
        .map(|x| {
            let y: Vec::<&str> = x.split(" ").collect();
            
            let direction_str: &str = y[0];
            let distance: i32 = y[1].parse().expect("Failed to cast &str to i32");

            build_move(direction_str, distance)
        })
        .collect();

    Ok(moves_vec)
}

// We construct each line in input.txt to a Move
enum Direction {
    Forward,
    Up,
    Down,
}

struct Move {
    direction: Direction,
    distance: i32,
}

fn build_move(direction_str: &str, distance: i32) -> Move {
    let direction = match direction_str {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("Unknown direction in input.txt!")
    };

    Move { direction, distance }
}

// Used from the sum of all of the moves
struct Displacement {
    horizontal: i32,
    vertical: i32,
}

// Part 1 - simply adding all of the moves
fn get_displacement(moves_vec: &Vec::<Move>) -> Displacement {
    let mut horizontal_displacement: i32 = 0;
    let mut vertical_displacement: i32 = 0;

    for i in 0..moves_vec.len() {
        match moves_vec[i].direction {
            Direction::Forward => horizontal_displacement = horizontal_displacement + moves_vec[i].distance,
            Direction::Down => vertical_displacement = vertical_displacement + moves_vec[i].distance,
            Direction::Up => vertical_displacement = vertical_displacement - moves_vec[i].distance,
        }
    }

    Displacement {
        horizontal: horizontal_displacement,
        vertical: vertical_displacement,
    }
}

// Part 2: use a concept of Aim where up and down are effectively angling the submarine
fn get_aim_displacement(moves_vec: &[Move]) -> Displacement {
    let mut horizontal_displacement: i32 = 0;
    let mut vertical_displacement: i32 = 0;
    let mut aim: i32 = 0;

    for i in 0..moves_vec.len() {
        match moves_vec[i].direction {
            Direction::Forward => {
                horizontal_displacement = horizontal_displacement + moves_vec[i].distance;
                vertical_displacement = vertical_displacement + (aim * moves_vec[i].distance);
            },
            Direction::Down => aim = aim + moves_vec[i].distance,
            Direction::Up => aim = aim - moves_vec[i].distance,
        }
    }

    Displacement {
        horizontal: horizontal_displacement,
        vertical: vertical_displacement,
    }
}

fn print_displacement_vals(displacement: Displacement) {
    println!("Horizontal Displacement = {}", displacement.horizontal);
    println!("Vertical Displacement = {}", displacement.vertical);
    println!("Multipled, this gives {}", displacement.horizontal * displacement.vertical);
    println!("");
}

fn main() -> Result<(), Error> {
    let moves_vec = generate_input_vec()?;

    print_displacement_vals(get_displacement(&moves_vec));
    print_displacement_vals(get_aim_displacement(&moves_vec));
        
    Ok(())
}
