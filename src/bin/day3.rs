use aoc_2025::assets::read_to_string;

struct Battery {
    jolt: Vec<u64>
}

impl Battery {
    fn new(raw_data: &str) -> Battery {
        let mut jolt = Vec::new();
        for c in raw_data.chars() {
            jolt.push(c.to_digit(10).unwrap() as u64)
        }

        Battery {
            jolt
        }
    }

}

fn find_max_jolt(battery: &Battery, begin: usize, end: usize) -> (u64, usize) {
    let mut max = u64::MIN;
    let mut max_idx = 0;
    for i in begin..end {
        if battery.jolt[i] > max {
            max = battery.jolt[i];
            max_idx = i;
        }
    }
    (max, max_idx)
}

fn find_joltage(battery: &Battery, target_len: usize) -> u64 {
    let vec = &battery.jolt;
    let jolt_len = vec.len();
    if jolt_len < target_len {
        panic!("can't find {target_len} digits from a too small vec");
    }
    // optimize to use math instead of string for better performance
    let mut result = String::from("");
    let mut iter = 0;
    for i in 0..target_len {
        let max_end = jolt_len - (target_len - i - 1);
        let (digit, pos) = find_max_jolt(battery, iter, max_end);
        iter = pos + 1;
        result.push(char::from_digit(digit as u32, 10).unwrap())
    }
    result.parse().unwrap()
}

fn main() {
    let batteries = parse_batteries("day3.txt");
    println!("problem 1: {}", sum_joltage(&batteries, 2));
    println!("problem 2: {}", sum_joltage(&batteries, 12));
}

fn parse_batteries(file_name: &str) -> Vec<Battery> {
    let mut batteries = Vec::new();
    let data = read_to_string(file_name).unwrap();
    for raw_joltage in data.lines() {
        batteries.push(Battery::new(raw_joltage))
    }
    batteries
}

fn sum_joltage(batteries: &Vec<Battery>, target_len: i32) -> u64 {
    let mut sum = 0;
    for battery in batteries {
        sum += find_joltage(battery, target_len as usize);
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_battery() {
        assert_eq!(Battery::new("9876543210").jolt, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    }

    #[test]
    fn test_find_max_jolt() {
        let batter1 = Battery::new("9876543210");
        assert_eq!(find_max_jolt(&batter1, 0, 9), (9, 0));
        assert_eq!(find_max_jolt(&batter1, 1, 10), (8, 1));

        let s2 = "811111111111119";
        let battery2 = Battery::new(s2);
        assert_eq!(find_max_jolt(&battery2,0, s2.len() - 1), (8, 0));
    }

    #[test]
    fn test_find_joltage_2() {
        assert_eq!(find_joltage(&Battery::new("987654321111111"), 2), 98);
        assert_eq!(find_joltage(&Battery::new("811111111111119"), 2), 89);
        assert_eq!(find_joltage(&Battery::new("234234234234278"), 2), 78);
        assert_eq!(find_joltage(&Battery::new("818181911112111"), 2), 92);
    }

    #[test]
    fn test_find_joltage_12() {
        assert_eq!(find_joltage(&Battery::new("987654321111111"), 12), 987654321111);
        assert_eq!(find_joltage(&Battery::new("811111111111119"), 12), 811111111119);
        assert_eq!(find_joltage(&Battery::new("234234234234278"), 12), 434234234278);
        assert_eq!(find_joltage(&Battery::new("818181911112111"), 12), 888911112111);
    }
}
