use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "./input.txt";

const MAP_LENGTH: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

enum LineDir {
    Forwards,
    Backwards,
    Stationary
}

impl LineSegment {
    pub fn create(segment_vec: &[isize]) -> LineSegment {
        if segment_vec.len() != 4 {
            panic!("Incorrect use of LineSegment::create");
        }

        LineSegment {
            x1: segment_vec[0],
            y1: segment_vec[1],
            x2: segment_vec[2],
            y2: segment_vec[3],
        }
    }

    pub fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn is_diagonal(&self) -> bool {
        (self.x1 - self.x2).abs() == (self.y1 - self.y2).abs()
    }

    pub fn length(&self) -> isize {
        std::cmp::max(
            (self.y1 - self.y2).abs(),
            (self.x1 - self.x2).abs()
        )
    }

    pub fn x_dir(&self) -> LineDir {
        if self.x1 > self.x2 {
            LineDir::Backwards
        } else if self.x1 < self.x2 {
            LineDir::Forwards
        } else {
            LineDir::Stationary
        }
    }

    pub fn y_dir(&self) -> LineDir {
        if self.y1 > self.y2 {
            LineDir::Backwards
        } else if self.y1 < self.y2 {
            LineDir::Forwards
        } else {
            LineDir::Stationary
        }
    }
}

struct Map {
    points: [[u32; MAP_LENGTH]; MAP_LENGTH]
}

impl Map {
    fn create() -> Map {
        Map {
            points: [[0; MAP_LENGTH]; MAP_LENGTH]
        }
    }

    fn add_line(&mut self, segment: LineSegment) {
        let x_dir = segment.x_dir();
        let y_dir = segment.y_dir();
        let length = segment.length();

        let mut x = segment.x1 as usize;
        let mut y = segment.y1 as usize;

        self.points[x][y] = self.points[x][y] + 1;
        for _ in 0..length {
            self.increment_point(&x_dir, &mut x);
            self.increment_point(&y_dir, &mut y);
            self.points[x][y] = self.points[x][y] + 1;
        }
    }

    fn increment_point(&self, dir: &LineDir, point: &mut usize) {
        // There must be a better way of doing this
        // It would be nice is `for i in (100..1) { }` worked
        match dir {
            LineDir::Forwards => *point = *point + 1,
            LineDir::Backwards => *point = *point - 1,
            LineDir::Stationary => ()
        }
    }

    fn count_mulitple_line_points(&self) -> u32 {
        let mut total: u32 = 0;
        for y in 0..MAP_LENGTH {
            for x in 0..MAP_LENGTH {
                if self.points[x][y] > 1 {
                    total = total + 1;
                }
            }
        }

        total
    }
}

fn read_input_file() -> Vec::<LineSegment> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut line_segments: Vec::<LineSegment> = Vec::new();

    for line in reader.lines() {
        let formatted_line_str = &line.unwrap()[..].replace(" -> ", ",");
        let line_segment_vec: Vec::<isize> = formatted_line_str
            .split(",")
            .map(|x| x.parse().expect("Failed to cast &str to u32"))
            .collect();

        line_segments.push(LineSegment::create(&line_segment_vec));
    }

    line_segments
}

fn main() {
    let segments = read_input_file();

    let mut map = Map::create();

    println!("Part 1");

    for segment in &segments {
        if segment.is_straight() {
            map.add_line(*segment);
        }
    }

    println!(
        "Straight line multiple point count = {}\n",
        map.count_mulitple_line_points()
    );

    println!("Part 2");

    for segment in &segments {
        if segment.is_diagonal() {
            map.add_line(*segment);
        }
    }

    println!(
        "Straight and diagonal line multiple point count = {}\n",
        map.count_mulitple_line_points()
    );
}