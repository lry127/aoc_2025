use aoc_2025::assets::read_to_string;

struct RangesContainer {
    ranges: Vec<(u64, u64)>,
}

impl RangesContainer {
    fn new(raw_ranges: &str) -> RangesContainer {
        let ranges: Vec<(u64, u64)> = raw_ranges
            .lines()
            .map(|r| r.split_once("-").unwrap())
            .map(|(begin, end)| (begin.parse().unwrap(), end.parse().unwrap()))
            .collect();
        RangesContainer { ranges }
    }
    fn is_within_range(&self, id: u64) -> bool {
        self.ranges
            .iter()
            .any(|(begin, end)| id >= *begin && id <= *end)
    }
}

fn run_problem_1(ranges: &RangesContainer, ids: &[u64]) -> usize {
    ids.iter().filter(|id| ranges.is_within_range(**id)).count()
}

fn find_overlap(counted: &[(u64, u64)], begin: u64, end: u64) -> Option<usize> {
    counted
        .iter()
        .enumerate()
        .find(|(_, counted)| {
            let counted_begin = (**counted).0;
            let counted_end = (**counted).1;
            if begin >= counted_begin && begin <= counted_end {
                return true;
            }
            if end >= counted_begin && end <= counted_end {
                return true;
            }
            if begin < counted_begin && end > counted_end {
                return true;
            }
            return false;
        })
        .map(|v| v.0)
}

fn merge_range_inner(ranges: &[(u64, u64)]) -> (Vec<(u64, u64)>, bool) {
    let mut final_range = Vec::new();
    let mut merged = false;

    ranges.iter().for_each(|(begin, end)| {
        if let Some(overlap_idx) = find_overlap(&final_range, *begin, *end) {
            let (old_begin, old_end) = final_range[overlap_idx];
            let new_begin = old_begin.min(*begin);
            let new_end = old_end.max(*end);
            final_range[overlap_idx] = (new_begin, new_end);
            merged = true;
        } else {
            final_range.push((*begin, *end));
        }
    });

    (final_range, merged)
}


// this is my initial impl, there's a better one using a clever Merge Interval algorithm
fn run_problem_2_brute_force(ranges: &RangesContainer) -> u64 {
    let mut ranges = ranges.ranges.clone();

    loop {
        let (new_ranges, is_merged) = merge_range_inner(&ranges);
        ranges = new_ranges;

        if !is_merged {
            break;
        }
    }
    ranges.iter().map(|(begin, end)| end - begin + 1).sum()
}


fn run_problem_2(ranges: &RangesContainer) -> u64 {
    let mut ranges = ranges.ranges.clone();
    ranges.sort_by_key(|r| r.0);

    let mut merged_interval: Vec<(u64, u64)> = Vec::new();
    ranges.iter().for_each(|(begin, end)| {
        if let Some((_, prev_end)) = merged_interval.last_mut() {
            if begin <= prev_end {
                // overlap
                *prev_end = *end.max(prev_end);
            } else {
                merged_interval.push((*begin, *end));
            }
        } else {
            merged_interval.push((*begin, *end));
        }
    });

    merged_interval.iter().map(|(begin, end)| end - begin + 1).sum()
}


fn main() {
    let filename = "day5.txt";
    let (ranges, ids) = parse_input(filename);
    println!("problem 1: {}", run_problem_1(&ranges, &ids));
    println!("problem 2: {}", run_problem_2(&ranges));
    println!("problem 2 (brute force): {}", run_problem_2_brute_force(&ranges));
}

fn parse_input(file_name: &str) -> (RangesContainer, Vec<u64>) {
    let raw_data = read_to_string(file_name).unwrap();
    let data = raw_data.split_once("\n\n").unwrap();

    let ranges = RangesContainer::new(data.0);
    let ids = data
        .1
        .lines()
        .map(|raw_id| raw_id.parse().unwrap())
        .collect();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_tests() {
        let src = "3-5
10-14
16-20
12-18";
        let ranges = RangesContainer::new(src);
        assert!(ranges.is_within_range(3));
        assert!(ranges.is_within_range(5));
        assert!(ranges.is_within_range(11));
        assert!(ranges.is_within_range(17));
        assert!(!ranges.is_within_range(1));
        assert!(!ranges.is_within_range(8));
        assert!(!ranges.is_within_range(32));
    }

    #[test]
    fn sum_tests_1() {
        let src = "3-5
14-14
10-14
16-20
12-18";
        let ranges = RangesContainer::new(src);
        assert_eq!(run_problem_2(&ranges), 14);
    }
}
