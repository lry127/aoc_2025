use crate::Direction::Left;
use Direction::Right;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(d: char) -> Direction {
        match d {
            'L' => Left,
            'R' => Right,
            other => panic!("Invalid direction {other}"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            if self.direction == Left {
                "left"
            } else {
                "right"
            },
            self.distance
        )
    }
}

impl Instruction {
    fn new(raw_instruction: &str) -> Instruction {
        let mut chars = raw_instruction.chars();
        let direction = Direction::new(chars.next().unwrap());
        let distance: i32 = chars.as_str().parse().unwrap();
        Instruction {
            direction,
            distance,
        }
    }

    fn apply(&self, current_pos: i32) -> (i32, i32) {
        let mut passing_zero_times = 0;
        passing_zero_times += self.distance / 100;

        let effective_distance = self.distance % 100;
        if effective_distance == 0 {
            return (current_pos, passing_zero_times);
        }

        let raw_pos = match self.direction {
            Left => current_pos - effective_distance,
            Right => current_pos + effective_distance,
        };

        if raw_pos >= 100 {
            (raw_pos - 100, passing_zero_times + 1)
        } else if raw_pos == 0 {
            (raw_pos, passing_zero_times + 1)
        } else if raw_pos < 0 {
            (
                raw_pos + 100,
                passing_zero_times + if current_pos == 0 { 0 } else { 1 },
            )
        } else {
            (raw_pos, passing_zero_times)
        }
    }
}

fn parse_instructions(input_file: &str) -> Vec<Instruction> {
    let data = aoc_2025::assets::read_to_string(input_file).unwrap();
    let mut instructions = Vec::new();
    for line in data.lines() {
        instructions.push(Instruction::new(line))
    }
    instructions
}

fn main() {
    let instructions = parse_instructions("day1.txt");
    let question_1 = run_question_1(&instructions);
    println!("question 1: {question_1} zeros");

    let question_2 = run_question_2(&instructions);
    println!("question 2: passing {question_2} times");
}

fn run_question_1(instructions: &Vec<Instruction>) -> i32 {
    let mut curr_pos = 50;
    let mut zeros = 0;
    for inst in instructions {
        let (new_pos, _) = inst.apply(curr_pos);
        if new_pos == 0 {
            zeros += 1;
        }
        // println!("curr {},  {} , new {}, ({zeros})", curr_pos, inst, new_pos);
        curr_pos = new_pos;
    }
    zeros
}

fn run_question_2(instructions: &Vec<Instruction>) -> i32 {
    let mut curr_pos = 50;
    let mut total_passing_times = 0;
    for inst in instructions {
        let (new_pos, passing_time) = inst.apply(curr_pos);
        total_passing_times += passing_time;
        // println!(
        //     "curr {},  {} , new {}, ({total_passing_times})",
        //     curr_pos, inst, new_pos
        // );
        curr_pos = new_pos;
    }
    total_passing_times
}

#[cfg(test)]
mod integration_test {
    #[test]
    fn test_question_1_examples() {
        assert_eq!(
            super::run_question_1(&super::parse_instructions("day1_example.txt")),
            3
        )
    }

    #[test]
    fn test_question_2_examples() {
        assert_eq!(
            super::run_question_2(&super::parse_instructions("day1_example.txt")),
            6
        )
    }
}

#[cfg(test)]
mod instruction_test {
    use crate::Direction::{Left, Right};
    use crate::Instruction;

    #[test]
    fn correct_parse() {
        assert_eq!(
            Instruction {
                direction: Left,
                distance: 68
            },
            Instruction::new("L68")
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 48
            },
            Instruction::new("R48")
        );
    }

    #[test]
    #[should_panic]
    fn incorrect_instruction() {
        Instruction::new("");
    }

    #[test]
    fn correct_apply() {
        assert_eq!(
            Instruction {
                direction: Left,
                distance: 1
            }
            .apply(10),
            (9, 0)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 1
            }
            .apply(10),
            (11, 0)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 68
            }
            .apply(50),
            (82, 1)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 30
            }
            .apply(82),
            (52, 0)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 48
            }
            .apply(52),
            (0, 1)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 48
            }
            .apply(53),
            (1, 1)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 501
            }
            .apply(50),
            (51, 5)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 550
            }
            .apply(50),
            (0, 6)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 5
            }
            .apply(0),
            (95, 0)
        );

        assert_eq!(
            Instruction {
                direction: Right,
                distance: 200
            }
            .apply(0),
            (0, 2)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 5
            }
            .apply(5),
            (0, 1)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 5
            }
            .apply(0),
            (95, 0)
        );

        assert_eq!(
            Instruction {
                direction: Left,
                distance: 105
            }
            .apply(5),
            (0, 2)
        );
    }
}
