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
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Up,
    Down,
    Left,
    Right
}

impl LineSegment {
    pub fn create(segment_vec: Vec::<isize>) -> LineSegment {
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

    pub fn line_dir(&self) -> Option<LineDir> {
        // There must be a better way of doing this
        if !(self.is_diagonal() || self.is_straight()) {
            return None
        }

        if self.x2 > self.x1 && self.y2 > self.y1 {
            return Some(LineDir::TopRight)
        }
        else if self.x2 > self.x1 && self.y2 < self.y1 {
            return Some(LineDir::BottomRight)
        }
        else if self.x2 < self.x1 && self.y2 > self.y1 {
            return Some(LineDir::TopLeft)
        }
        else if self.x2 < self.x1 && self.y2 < self.y1 {
            return Some(LineDir::BottomLeft)
        }
        else if self.x1 == self.x2 && self.y2 > self.y1 {
            return Some(LineDir::Up)
        }
        else if self.x1 == self.x2 && self.y2 < self.y1 {
            return Some(LineDir::Down)
        }
        else if self.x1 > self.x2 && self.y2 == self.y1 {
            return Some(LineDir::Left)
        }
        else if self.x1 < self.x2 && self.y2 == self.y1 {
            return Some(LineDir::Right)
        }

        None
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
        let dir = segment.line_dir().unwrap();
        let length = std::cmp::max(
            (segment.y1 - segment.y2).abs(),
            (segment.x1 - segment.x2).abs()
        );

        let mut x = segment.x1 as usize;
        let mut y = segment.y1 as usize;

        self.points[x][y] = self.points[x][y] + 1;
        for _ in 0..length {
            self.increment_point(&dir, &mut x, &mut y);
            self.points[x][y] = self.points[x][y] + 1;
        }
    }

    fn increment_point(&self, dir: &LineDir, x: &mut usize, y: &mut usize) {
        // There must be a better way of doing this
        // It would be nice is `for i in (100..1) { }` worked
        match dir {
            LineDir::TopLeft => {
                *x = *x - 1;
                *y = *y + 1;
            },
            LineDir::TopRight => {
                *x = *x + 1;
                *y = *y + 1;
            },
            LineDir::BottomLeft => {
                *x = *x - 1;
                *y = *y - 1;
            },
            LineDir::BottomRight => {
                *x = *x + 1;
                *y = *y - 1;
            },
            LineDir::Left => {
                *x = *x - 1;
            },
            LineDir::Right => {
                *x = *x + 1;
            },
            LineDir::Up => {
                *y = *y + 1;
            },
            LineDir::Down => {
                *y = *y - 1;
            },
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

        line_segments.push(LineSegment::create(line_segment_vec));
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