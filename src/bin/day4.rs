use aoc_2025::assets::read_to_string;

#[derive(Debug)]
struct RollsGrip {
    grip: Vec<Vec<bool>>,
}

impl RollsGrip {
    fn new(raw_grip_data: &str) -> RollsGrip {
        let grip = raw_grip_data
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();

        RollsGrip { grip }
    }

    fn rows(&self) -> usize {
        self.grip.len()
    }

    fn cols(&self) -> usize {
        self.grip.first().unwrap().len()
    }

    fn is_roll_exist(&self, row_no: i32, col_no: i32) -> bool {
        if row_no < 0 || row_no >= self.rows() as i32 {
            return false;
        }

        if col_no < 0 || col_no >= self.cols() as i32 {
            return false;
        }

        self.grip[row_no as usize][col_no as usize]
    }

    fn remove(&mut self, row_idx: i32, col_idx: i32) {
        self.grip[row_idx as usize][col_idx as usize] = false
    }
}

fn is_roll_movable(grip: &RollsGrip, row: i32, col: i32) -> bool {
    let mut surrounding_count = 0;
    for r in row - 1..=row + 1 {
        for c in col - 1..=col + 1 {
            if grip.is_roll_exist(r, c) {
                surrounding_count += 1;
            }
        }
    }
    surrounding_count - 1 < 4
}

fn main() {
    let input_file = "day4.txt";
    let mut grip = RollsGrip::new(&read_to_string(input_file).unwrap());
    println!("problem 1: {}", run_problem_1(&grip));
    println!("problem 2: {}", run_problem_2(&mut grip));
}

fn find_movable_rolls(rolls_grip: &RollsGrip) -> Vec<(i32, i32)> {
    rolls_grip
        .grip
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, _)| (row_idx as i32, col_idx as i32))
        })
        .filter(|(row_idx, col_idx)| {
            rolls_grip.is_roll_exist(*row_idx, *col_idx)
                && is_roll_movable(rolls_grip, *row_idx, *col_idx)
        })
        .collect()
}

fn run_problem_1(rolls_grip: &RollsGrip) -> usize {
    find_movable_rolls(rolls_grip).len()
}

fn run_problem_2(rolls_grip: &mut RollsGrip) -> usize {
    let mut total_removed = 0;

    loop {
        let to_be_removed = find_movable_rolls(rolls_grip);
        let this_removed = to_be_removed.len();

        to_be_removed
            .iter()
            .for_each(|(row_idx, col_idx)| rolls_grip.remove(*row_idx, *col_idx));

        total_removed += this_removed;
        if this_removed == 0 {
            break;
        }
    }
    total_removed
}
