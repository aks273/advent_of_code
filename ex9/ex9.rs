use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "input.txt";

fn get_file_input() -> Vec::<Vec::<u32>> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec::<Vec::<u32>> = Vec::new();

    const RADIX: u32 = 10;

    for line in reader.lines() {
        let heights: Vec::<u32> = line.unwrap()
            .chars()
            .map(|x| x.to_digit(RADIX).unwrap())
            .collect();

        // This parsing of the input file means we must index map as map[y][x],
        // because the first index gives us a horizontal line (at fixed y) and
        // then the second index gives us a point within that line.
        map.push(heights);
    }

    map
}

fn calculate_if_minima(
    map: &Vec::<Vec::<u32>>, x: usize, y: usize, map_width: usize, map_height: usize
) -> bool {
    // max height on the map is 9, so if we are on an edge we need the fourth
    // comparison point to definitely be higher than the third
    let mut comparison_points: [u32; 4] = [10; 4];

    // We must make sure we do not compare to the left if we are at the left
    // edge, etc.
    if x > 0                { comparison_points[0] = map[y][x - 1]; }
    if x < map_width - 1    { comparison_points[1] = map[y][x + 1]; }
    if y > 0                { comparison_points[2] = map[y - 1][x]; }
    if y < map_height - 1   { comparison_points[3] = map[y + 1][x]; }

    // Only return true if all four comparison points are greater than map[y][x]
    comparison_points.iter().all(|p| p > &map[y][x])
}

fn fill_basin_vec(
    basin_vec: &mut Vec::<(usize, usize)>, map: &Vec::<Vec::<u32>>, x: usize, y: usize, map_width: usize, map_height: usize
) {
    // We "explore" the area around a minimum. If we have discovered it is
    // already a minimum, stop as we definitely know it has already been
    // explored
    if basin_vec.contains(&(x, y)) { return }

    // We know that this point is now part of the basin, because it is either a
    // minimum, or satisfies one of the below conditions.
    basin_vec.push((x, y));

    // Explore the next point if we are not at the edge, the next point is
    // higher than the current point, and the next point is lower than 9.
    // We have to explore in four directions (left, down, up, right) for each
    // point within the basin.
    //
    // This is a bit annoying because if a point is not part of a basin it can
    // be explored multiple times.
    // For example the 9 in the centre of
    //      219
    //      39X
    //      9XX
    // will be explored twice. We could possibly mark points as "explored" in
    // a better way than checking if they are in the basin, but this is even
    // more overhead which doesn't seem too neccesary.
    if x > 0 && map[y][x - 1] > map[y][x] && map[y][x - 1] < 9 { 
        fill_basin_vec(basin_vec, map, x - 1, y, map_width, map_height);
    }
    if y > 0 && map[y - 1][x] > map[y][x] && map[y - 1][x] < 9 { 
        fill_basin_vec(basin_vec, map, x, y - 1, map_width, map_height);
    }
    if x < map_width - 1 && map[y][x + 1] > map[y][x] && map[y][x + 1] < 9 { 
        fill_basin_vec(basin_vec, map, x + 1, y, map_width, map_height);
    }
    if y < map_height - 1 && map[y + 1][x] > map[y][x] && map[y + 1][x] < 9 { 
        fill_basin_vec(basin_vec, map, x, y + 1, map_width, map_height);
    }
}

fn update_largest_basins(largest_basins: &mut [usize], basin_size: usize) {
    // Urgh. This can probably be nerdified but we get the gist.
    if basin_size > largest_basins[0] {
        largest_basins[2] = largest_basins[1];
        largest_basins[1] = largest_basins[0];
        largest_basins[0] = basin_size;
    } else if basin_size > largest_basins[1] {
        largest_basins[2] = largest_basins[1];
        largest_basins[1] = basin_size;
    } else if basin_size > largest_basins[2] {
        largest_basins[2] = basin_size;
    }
}

fn main() {
    let map = get_file_input();
    let map_height = map.len() as usize;
    let map_width = map[0].len() as usize;

    // The sum of all of the basin heights + the number of basins
    let mut risk_sum = 0;

    // The three largest basins
    let mut largest_basins: [usize; 3] = [0; 3];

    for y in 0..map_height {
        for x in 0..map_width {
            // Passing in the map width and map height here looks a bit funny,
            // but I don't want to calculate the lengths of the map vec each
            // time we lookup whether a minima / the basin size.
            if calculate_if_minima(&map, x, y, map_width, map_height) {
                risk_sum += map[y][x] + 1;
                
                let mut basin_vec = Vec::<(usize, usize)>::new();
                fill_basin_vec(&mut basin_vec, &map, x, y, map_width, map_height);

                let basin_size = basin_vec.len();
                update_largest_basins(&mut largest_basins, basin_size);
            }
        }
    }

    println!("Part 1");
    println!("Risk sum of map = {}", risk_sum);
    println!("");

    println!("Part 2");
    println!("Largest basins contain the following number of points: {:?}", largest_basins);
    println!("Multiplied, this gives {:?}", largest_basins[0] * largest_basins[1] * largest_basins[2]);
    println!("");
}