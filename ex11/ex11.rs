use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "input.txt";

const MAP_LEN: usize = 10;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn x_position(&self) -> Position {
        if self.x == 0                  { return Position::Start }
        else if self.x == MAP_LEN - 1   { return Position::End }
        else                            { return Position::Middle}
    }

    fn y_position(&self) -> Position {
        if self.y == 0                  { return Position::Start }
        else if self.y == MAP_LEN - 1   { return Position::End }
        else                            { return Position::Middle }
    }
}

enum Position {
    Start,
    Middle,
    End,
}

fn get_file_input() -> Vec::<Vec::<u32>> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut input_file = Vec::<Vec::<u32>>::new();

    const RADIX: u32 = 10;

    for line in reader.lines() {
        let parsed_line: Vec::<u32> = line.unwrap()
            .chars()
            .map(|x| x.to_digit(RADIX).unwrap())
            .collect();

        input_file.push(parsed_line);
    }

    input_file
}

// Helper method used while debugging to print energy levels in a readable form
fn print_energy_levels(levels: &Vec::<Vec::<u32>>) {
    for y in 0..levels.len() {
        for x in 0..levels[0].len() {
            print!("{}", levels[y][x]);
        }
        println!("");
    }

    println!("\n");
}

fn update_adjacent_levels(map: &mut Vec::<Vec::<u32>>, point: Point, number_of_flashes: &mut u32) {
    if map[point.y][point.x] == 0 {
        return
    }

    // An initial point has flashed
    if map[point.y][point.x] == 10 {
        map[point.y][point.x] = 0;
    } else {
        map[point.y][point.x] += 1;
        
        // An adjacent point has flashed
        if map[point.y][point.x] == 10 {
            map[point.y][point.x] = 0;
        }
    }

    if map[point.y][point.x] > 0 {
        return
    }

    // A flash triggers adjacent levels to be updated
    *number_of_flashes += 1;

    let x_update_points: Vec::<usize> = match point.x_position() {
        Position::Start     => vec![point.x, point.x + 1],
        Position::Middle    => vec![point.x - 1, point.x, point.x + 1],
        Position::End       => vec![point.x - 1, point.x],
    };

    let y_update_points: Vec::<usize> = match point.y_position() {
        Position::Start     => vec![point.y, point.y + 1],
        Position::Middle    => vec![point.y - 1, point.y, point.y + 1],
        Position::End       => vec![point.y - 1, point.y],
    };

    for y in &y_update_points {
        for x in &x_update_points {
            let next_point = Point { x: *x, y: *y };

            // we don't want to update the current point because it only trigger
            // adjacent updates.
            if (next_point.x, next_point.y) != (point.x, point.y) {
                update_adjacent_levels(map, next_point, number_of_flashes);
            }
        }
    }
}

fn main() {
    let mut energy_levels = get_file_input();

    let mut number_of_flashes = 0;
    let mut update_count = 0;

    loop {
        // We update each of the energy levels by one, and then check the state
        // of them and see if any of the octopuses need to update their neighbors
        // and then count the number of flashes per step.
        //
        // This involves looping through the energy levels array three times for
        // per step which is not the best
        energy_levels = energy_levels
            .into_iter()
            .map(|x| x.into_iter().map(|y| y + 1).collect())
            .collect();

        for y in 0..energy_levels.len() {
            for x in 0..energy_levels[0].len() {
                if energy_levels[y][x] == 10 {
                    update_adjacent_levels(&mut energy_levels, Point { x, y }, &mut number_of_flashes);
                }
            }
        }

        update_count += 1;

        if update_count == 100 {
            println!("Part 1: Number of flashes after 100 updates = {}", number_of_flashes);
        }

        if energy_levels.iter().all(|x| x.iter().all(|y| *y == 0)) {
            println!("Part 2: Number of updates for a total flash = {}", update_count);
            break;
        }
    }
}