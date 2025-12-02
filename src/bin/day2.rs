use aoc_2025::assets::read_to_string;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct ProductRange {
    begin: u64,
    end: u64,
}

impl ProductRange {
    fn new(raw_range: &str) -> ProductRange {
        let mut r = raw_range.split("-");
        let begin = r.next().unwrap();
        let end = r.next().unwrap();
        ProductRange {
            begin: begin.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn get_range(&self) -> RangeInclusive<u64> {
        self.begin..=self.end
    }
}

trait InvalidIndexDetector {
    fn is_invalid_index(&self, index: &String) -> bool;
}

struct Problem1Detector;
impl InvalidIndexDetector for Problem1Detector {
    fn is_invalid_index(&self, index: &String) -> bool {
        if index.len() % 2 != 0 {
            return false;
        }
        let half = index.len() / 2;
        let left = &index[..half];
        let right = &index[half..];
        left == right
    }
}

struct Problem2Detector;
impl InvalidIndexDetector for Problem2Detector {
    fn is_invalid_index(&self, index: &String) -> bool {
        let total_len = index.len();
        let half = index.len() / 2;
        'check_pattern: for i in 0..half {
            let pattern_len = i + 1;
            // a quick length based check
            if total_len % pattern_len != 0 {
                continue;
            }

            let repeated_pattern = &index[..=i];
            let repeated_num = total_len / pattern_len;
            for j in 1..repeated_num {
                let begin_idx = j * pattern_len;
                let end_idx = (j + 1) * pattern_len;
                let this_sub = &index[begin_idx..end_idx];
                if this_sub != repeated_pattern {
                    continue 'check_pattern;
                }
            }
            return true;
        }
        false
    }
}


fn main() {
    let ranges = parse_product_ranges("day2.txt");
    println!("problem 1: {}", run_problem_1(&ranges));
    println!("problem 2: {}", run_problem_2(&ranges));
}

fn parse_product_ranges(file_name: &str) -> Vec<ProductRange> {
    let mut ranges = Vec::new();
    let data = read_to_string(file_name).unwrap();
    for range in data.split(",") {
        ranges.push(ProductRange::new(range))
    }
    ranges
}

fn find_invalid_indexes(range: &ProductRange, invalid_range_detector: &impl InvalidIndexDetector) -> Vec<u64> {
    let mut invalid_indexes = Vec::new();
    for index in range.get_range() {
        let s = index.to_string();
        if invalid_range_detector.is_invalid_index(&s) {
            invalid_indexes.push(index)
        }
    }

    invalid_indexes
}

fn run_problem(ranges: &Vec<ProductRange>, invalid_range_detector: &impl InvalidIndexDetector) -> u64 {
    let mut sum = 0;
    for range in ranges {
       let invalid_indexes = find_invalid_indexes(range, invalid_range_detector);
        for invalid in invalid_indexes {
            sum += invalid;
        }
    }
    sum
}

fn run_problem_1(ranges: &Vec<ProductRange>) -> u64 {
    run_problem(ranges, &Problem1Detector)
}

fn run_problem_2(ranges: &Vec<ProductRange>) -> u64 {
    run_problem(ranges, &Problem2Detector)
}



#[cfg(test)]
mod production_range_test {
    use super::*;

    #[test]
    fn parse_range() {
        assert_eq!(ProductRange::new("11-22").get_range(), 11..=22);
        assert_eq!(ProductRange::new("2121212118-2121212124").get_range(), 2121212118..=2121212124)
    }

    #[test]
    fn invalid_index_problem_1() {
        let problem1 = Problem1Detector;
        let is_invalid_index = |x: &String| { problem1.is_invalid_index(x)};
        assert!(is_invalid_index(&"11".to_string()));
        assert!(is_invalid_index(&"22".to_string()));
        assert!(is_invalid_index(&"1188511885".to_string()));
        assert!(is_invalid_index(&"38593859".to_string()));

        assert!(!is_invalid_index(&"12".to_string()));
        assert!(!is_invalid_index(&"123".to_string()));
    }

    #[test]
    fn invalid_index_problem_2() {
        let detector = Problem2Detector;
        let is_invalid_index = |x: &String| { detector.is_invalid_index(x)};
        assert!(is_invalid_index(&"11".to_string()));
        assert!(is_invalid_index(&"22".to_string()));
        assert!(is_invalid_index(&"111".to_string()));
        assert!(is_invalid_index(&"565656".to_string()));
        assert!(is_invalid_index(&"1188511885".to_string()));
        assert!(is_invalid_index(&"38593859".to_string()));

        assert!(!is_invalid_index(&"12".to_string()));
        assert!(!is_invalid_index(&"123".to_string()));
    }


}
