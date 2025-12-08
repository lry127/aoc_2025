use aoc_2025::assets::read_to_string;

#[derive(Debug)]
struct ProblemSheet {
    problems: Vec<Problem>,
}

impl ProblemSheet {
    fn new(raw_problem: &str) -> Self {
        let problem_lines: Vec<Vec<char>> =
            raw_problem.lines().map(|s| s.chars().collect()).collect();

        let max_len = problem_lines.iter().map(|l| l.len()).max().unwrap();
        let operands_each_problem = problem_lines.len() - 1;

        let mut problems: Vec<Problem> = Vec::new();

        let mut operands: Vec<Operand> = vec![Operand::new(); operands_each_problem];
        let mut operator: char = ' ';
        for i in 0..max_len {
            let chars_in_each_line: Vec<char> = problem_lines
                .iter()
                .map(|l| if i < l.len() { l[i] } else { ' ' })
                .collect();

            if chars_in_each_line.iter().all(|c| *c == ' ') {
                let prev_problem = Problem::new(operator, operands);
                operands = vec![Operand::new(); operands_each_problem];
                operator = ' ';
                problems.push(prev_problem);
                continue;
            }

            let possible_operator = *chars_in_each_line.last().unwrap();
            if possible_operator == '+' || possible_operator == '*' {
                operator = possible_operator;
            }
            chars_in_each_line
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != operands_each_problem)
                .for_each(|(idx, c)| {
                    operands[idx].add_char(*c);
                })
        }

        let last_problem = Problem::new(operator, operands);
        problems.push(last_problem);

        ProblemSheet { problems }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn new(symbol: char) -> Operator {
        match symbol {
            '+' => Operator::Plus,
            '*' => Operator::Multiply,
            other => panic!("unknown operator: {other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Operand {
    raw_operand: Vec<char>,
}

impl Operand {
    fn new() -> Operand {
        Operand {
            raw_operand: Vec::new(),
        }
    }

    fn add_char(&mut self, c: char) {
        self.raw_operand.push(c);
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    operator: Operator,
    operands: Vec<Operand>,
}

impl Problem {
    fn new(operator: char, operands: Vec<Operand>) -> Self {
        Problem {
            operator: Operator::new(operator),
            operands,
        }
    }

    fn evaluate<T>(&self, transformer: &mut T) -> u64
    where
        T: FnMut(&Vec<Operand>) -> Vec<u64>,
    {
        let operands = transformer(&self.operands);

        match self.operator {
            Operator::Plus => Self::plus(&operands),
            Operator::Multiply => Self::multiply(&operands),
        }
    }

    fn multiply(operands: &Vec<u64>) -> u64 {
        operands.iter().fold(1, |acc, v| acc * v)
    }

    fn plus(operands: &Vec<u64>) -> u64 {
        operands.iter().fold(0, |acc, v| acc + v)
    }
}

fn sum_problems<T>(problem_sheet: &ProblemSheet, mut transformer: T) -> u64
where
    T: Fn(&Vec<Operand>) -> Vec<u64>,
{
    let mut sum = 0;
    for problem in &problem_sheet.problems {
        sum += problem.evaluate(&mut transformer);
    }
    sum
}

fn run_problem_1(problem_sheet: &ProblemSheet) -> u64 {
    sum_problems(problem_sheet, |vec| {
        vec.iter()
            .map(|operand| operand.raw_operand.iter().filter(|c| **c != ' ').collect())
            .map(|s: String| s.parse().unwrap())
            .collect()
    })
}

fn run_problem_2(problem_sheet: &ProblemSheet) -> u64 {
    let transformer = |vec: &Vec<Operand>| -> Vec<u64> {
        let mut operands = vec![String::new(); vec[0].raw_operand.len()];

        operands.iter_mut().enumerate().for_each(|(operand_idx, operand)| {
            vec.iter().for_each(|v| {
                let c = v.raw_operand[operand_idx];
                if c != ' ' {
                    operand.push(c);
                }
            });
        });

        operands.iter().rev().map(|o| o.parse().unwrap()).collect()
    };
    sum_problems(problem_sheet, transformer)
}

fn main() {
    let sheet = ProblemSheet::new(&read_to_string("day6.txt").unwrap());
    println!("problem 1 {}", run_problem_1(&sheet));
    println!("problem 2 {}", run_problem_2(&sheet));
}

#[cfg(test)]

mod tests {
    use crate::{Operand, Problem, ProblemSheet};
    use aoc_2025::assets::read_to_string;

    impl Operand {
        fn new_with_operands(operands: &str) -> Operand {
            let mut operand = Self::new();
            operands.chars().for_each(|c| operand.add_char(c));
            operand
        }
    }

    #[test]
    fn test_parse_problems() {
        let sheet = ProblemSheet::new(&read_to_string("day6_example.txt").unwrap());
        assert_eq!(sheet.problems.len(), 4);
        assert_eq!(
            sheet.problems[0],
            Problem::new(
                '*',
                vec![
                    Operand::new_with_operands("123"),
                    Operand::new_with_operands(" 45"),
                    Operand::new_with_operands("  6"),
                ]
            )
        );

        assert_eq!(
            sheet.problems[1],
            Problem::new(
                '+',
                vec![
                    Operand::new_with_operands("328"),
                    Operand::new_with_operands("64 "),
                    Operand::new_with_operands("98 "),
                ]
            )
        );
        assert_eq!(
            sheet.problems[2],
            Problem::new(
                '*',
                vec![
                    Operand::new_with_operands(" 51"),
                    Operand::new_with_operands("387"),
                    Operand::new_with_operands("215"),
                ]
            )
        );
        assert_eq!(
            sheet.problems[3],
            Problem::new(
                '+',
                vec![
                    Operand::new_with_operands("64 "),
                    Operand::new_with_operands("23 "),
                    Operand::new_with_operands("314"),
                ]
            )
        );
    }
}
