use aoc_2025::assets::read_to_string;
use std::error::Error;
use std::pin::pin;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Option<Self> {
        if x < 0 || y < 0 {
            return None;
        }
        Some(Self { x, y })
    }
}


fn main() {
    let data = read_to_string("day9.txt").unwrap();
    let points: Vec<Point> = data
        .lines()
        .filter_map(|line| {
            let (y_str, x_str) = line.split_once(',')?;
            let y = y_str.trim().parse().ok()?;
            let x = x_str.trim().parse().ok()?;

            Point::new(x, y)
        })
        .collect();
    println!("points: {:?}", run_problem_1(&points));
}

fn run_problem_1(points: &[Point]) -> i64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(idx, point_1)| {
            points[(idx + 1)..]
                .iter()
                .map(|point_2| ((point_1.x - point_2.x + 1) * (point_1.y - point_2.y + 1)).abs())
        })
        .max()
        .unwrap()
}
