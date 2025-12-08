use aoc_2025::assets::read_to_string;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Index2D {
    row_idx: usize,
    col_idx: usize,
}

impl Index2D {
    fn step_left(&self) -> Self {
        Index2D {
            row_idx: self.row_idx,
            col_idx: self.col_idx - 1,
        }
    }

    fn step_right(&self) -> Self {
        Index2D {
            row_idx: self.row_idx,
            col_idx: self.col_idx + 1,
        }
    }

    fn step_down(&self) -> Self {
        Index2D {
            row_idx: self.row_idx + 1,
            col_idx: self.col_idx,
        }
    }
}

struct TachyonManifold {
    starting_point: Index2D,
    splitter_map: Vec<bool>,
    height: usize,
    width: usize,
}

impl TachyonManifold {
    fn new(raw_data: &str) -> Self {
        let mut starting_point = None;
        let width = raw_data.lines().next().unwrap().chars().count();
        let height = raw_data.lines().count();

        let mut splitter_map = Vec::new();
        for (row_idx, row_data) in raw_data.lines().enumerate() {
            for (col_idx, ch) in row_data.chars().enumerate() {
                let is_splitter = match ch {
                    'S' => {
                        starting_point = Some(Index2D { row_idx, col_idx });
                        false
                    }
                    '.' => false,
                    '^' => true,
                    _ => panic!("unknown char @({row_idx},{width}): {}", ch),
                };
                splitter_map.push(is_splitter);
            }
        }

        if starting_point.is_none() {
            panic!("starting point not found");
        }

        TachyonManifold {
            splitter_map,
            starting_point: starting_point.unwrap(),
            height,
            width,
        }
    }

    fn is_splitter(&self, pos: &Index2D) -> bool {
        if self.is_out_of_map(pos) {
            return false;
        }

        self.splitter_map[pos.row_idx * self.width + pos.col_idx]
    }

    fn is_out_of_map(&self, pos: &Index2D) -> bool {
        pos.row_idx >= self.height || pos.col_idx >= self.width
    }
}

fn count_split_times(manifold: &TachyonManifold) -> i32 {
    count_split_times_recursive(
        manifold,
        &mut HashSet::new(),
        &mut HashSet::new(),
        manifold.starting_point,
    )
}

fn count_split_times_recursive(
    manifold: &TachyonManifold,
    visited_starting_pos: &mut HashSet<Index2D>,
    visited_splitter: &mut HashSet<Index2D>,
    begin_pos: Index2D,
) -> i32 {
    if visited_starting_pos.contains(&begin_pos) {
        return 0;
    }

    let mut split_times = 0;
    let mut curr_pos = begin_pos;
    loop {
        if manifold.is_out_of_map(&curr_pos) {
            break;
        }

        let next_pos = curr_pos.step_down();

        if manifold.is_splitter(&next_pos) {
            if visited_splitter.contains(&next_pos) {
                return 0;
            }
            visited_splitter.insert(next_pos);
            split_times += 1;
            split_times += count_split_times_recursive(
                manifold,
                visited_starting_pos,
                visited_splitter,
                curr_pos.step_left(),
            );
            split_times += count_split_times_recursive(
                manifold,
                visited_starting_pos,
                visited_splitter,
                curr_pos.step_right(),
            );
            break;
        } else {
            curr_pos = next_pos;
        }
    }

    visited_starting_pos.insert(curr_pos);
    split_times
}

fn count_timelines(manifold: &TachyonManifold) -> u64 {
    count_timelines_recursive(manifold, &mut HashMap::new(), manifold.starting_point)
}

fn count_timelines_recursive(
    manifold: &TachyonManifold,
    memo: &mut HashMap<Index2D, u64>,
    begin_pos: Index2D,
) -> u64 {
    if memo.contains_key(&begin_pos) {
        return memo[&begin_pos];
    }

    let mut timelines = 0;
    let mut curr_pos = begin_pos;
    let mut splitter_encountered = false;
    loop {
        if manifold.is_out_of_map(&curr_pos) {
            break;
        }

        let next_pos = curr_pos.step_down();
        if manifold.is_splitter(&next_pos) {
            splitter_encountered = true;
            timelines += count_timelines_recursive(manifold, memo, curr_pos.step_left());
            timelines += count_timelines_recursive(manifold, memo, curr_pos.step_right());
            break;
        } else {
            curr_pos = next_pos;
        }
    }
    if !splitter_encountered {
        timelines = 1;
    }

    memo.insert(begin_pos, timelines);

    timelines
}

fn main() {
    let manifold = TachyonManifold::new(&read_to_string("day7.txt").unwrap());
    println!("problem 1: {}", count_split_times(&manifold));
    println!("problem 2: {}", count_timelines(&manifold));
}
