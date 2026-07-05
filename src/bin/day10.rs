use aoc_2025::assets::read_to_string;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static BRACKET_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.*?)]").unwrap());
static PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((.*?)\)").unwrap());

#[derive(Debug)]
struct Button {
    control_btn_idx: Vec<usize>,
}

impl FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let control_btn_idx = s
            .split(',')
            .map(|n| n.trim().parse().ok())
            .collect::<Option<Vec<usize>>>()
            .ok_or(())?;
        Ok(Self { control_btn_idx })
    }
}

#[derive(Debug)]
struct Machine {
    final_light_state: Vec<bool>,
    buttons: Vec<Button>,
    joltage: String,
}

impl Machine {
    fn parse(raw_data: &str) -> Option<Self> {
        let final_light_state = Self::extract_final_state(raw_data)?;
        let buttons = Self::extract_buttons(raw_data)?;
        let res = Self {
            final_light_state,
            buttons,
            joltage: String::new(),
        };
        Some(res)
    }

    fn extract_final_state(raw_data: &str) -> Option<Vec<bool>> {
        BRACKET_RE
            .captures(raw_data)?
            .get(1)?
            .as_str()
            .chars()
            .map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect()
    }

    fn extract_buttons(raw_data: &str) -> Option<Vec<Button>> {
        PAREN_RE
            .captures_iter(raw_data)
            .filter_map(|capture| capture.get(1).map(|m| m.as_str()))
            .map(|raw_str| raw_str.parse().ok())
            .collect()
    }
}

fn compute_least_buttons_pushed(machine: &Machine) -> Option<usize> {
    machine.buttons.iter().powerset().find_map(|buttons| {
        let final_state = &machine.final_light_state;
        let mut curr_state = vec![false; final_state.len()];
        for button in &buttons {
            for idx in &button.control_btn_idx {
                curr_state[*idx] = !curr_state[*idx];
            }
        }

        if curr_state == *final_state {
            Some(buttons.len())
        } else {
            None
        }
    })
}

fn main() {
    let file_name = "day10.txt";
    let machines: Vec<Machine> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| Machine::parse(l).unwrap())
        .collect();

    let prob_1_res: usize = machines
        .iter()
        .filter_map(compute_least_buttons_pushed)
        .sum();
    println!("problem 1: {}", prob_1_res);
}

#[cfg(test)]
mod unit_tests {
    use crate::{Button, Machine};

    #[test]
    fn test_buttons_parse() {
        let b1 = "1,2,5".parse::<Button>().unwrap();
        assert_eq!(b1.control_btn_idx, [1, 2, 5]);

        assert!("1,3,,".parse::<Button>().is_err());
        assert!("1,3,q,".parse::<Button>().is_err());
    }

    #[test]
    fn test_button_groups_parse() {
        let btns = Machine::extract_buttons("(3) (1,3) (2) (2,3) (0,2) (0,1)");
        println!("{:?}", btns)
    }
}
