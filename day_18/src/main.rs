use std::fmt::{Display, Formatter};
use std::{fs::File, io::Read};

enum Operation {
    Add,
    Multiply,
}
impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result: &str = match self {
            Operation::Add => "+",
            Operation::Multiply => "*",
        };
        write!(f, "{}", result)
    }
}

fn evaluate_hunk(chars: Vec<char>) -> u64 {
    let mut left = 0;
    let mut operation = Operation::Add;
    println!("WHAT IS MY SEGMENT? {chars:?}");
    let mut i = 0;
    let end = chars.len();
    loop {
        let char = chars[i];
        println!("i: {i}, char: {char}");
        let result_number: Option<u64> = match char {
            '+' => {
                operation = Operation::Add;
                println!("Set operator: {operation}");
                None
            }
            '*' => {
                operation = Operation::Multiply;
                println!("Set operator: {operation}");
                None
            }
            '(' => {
                let slice_of_chars = &chars[(i + 1)..];
                let mut count_of_open_parens_found = 0;
                let index_of_closing_paren = slice_of_chars
                    .iter()
                    .position(|x| match *x {
                        ')' => {
                            if count_of_open_parens_found == 0 {
                                true
                            } else {
                                count_of_open_parens_found -= 1;
                                false
                            }
                        }
                        '(' => {
                            count_of_open_parens_found += 1;
                            false
                        }
                        _ => false,
                    })
                    .expect("Unable to find closing parentheses, GAME OVER");
                i += index_of_closing_paren + 1;
                let slice_of_chars: Vec<char> = slice_of_chars[..index_of_closing_paren]
                    .iter()
                    .map(|x| *x)
                    .collect();
                Some(evaluate_hunk(slice_of_chars))
            }
            ')' => panic!("You should NEVER hit a close paren! This should have been sliced out!"),
            x => Some(x.to_string().parse().expect("Cannot parse bad number!")),
        };

        left = match result_number {
            Some(x) => {
                let result = match operation {
                    Operation::Add => left + x,
                    Operation::Multiply => left * x,
                };
                println!("{left} {operation} {x} = {result}");
                result
            }
            _ => left,
        };
        i += 1;
        if i >= end {
            break;
        }
    }
    println!("WHAT IS MY RESULT? {left}");
    left
}

fn puzzle_part_1(input: &str) -> u64 {
    let cleaned = input.trim().replace(" ", "");
    let lines = cleaned.split("\n");
    lines.fold(0, |total, line| {
        total + evaluate_hunk(line.chars().collect())
    })
}

fn main() {
    let mut infile = File::open("./day_18/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");

    let result = puzzle_part_1(&whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer is 69490582260
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample_a() {
        let result = puzzle_part_1("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(result, 71);
    }

    #[test]
    fn part_1_sample_b() {
        let result = puzzle_part_1("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(result, 51);
    }

    #[test]
    fn part_1_sample_composite() {
        let result = puzzle_part_1("1 + 2 * 3 + 4 * 5 + 6\n1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(result, 71 + 51);
    }

    #[test]
    fn part_1_sample_c() {
        let result = puzzle_part_1("2 * 3 + (4 * 5)");
        assert_eq!(result, 26);
    }

    #[test]
    fn part_1_sample_d() {
        let result = puzzle_part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(result, 437);
    }

    #[test]
    fn part_1_sample_e() {
        let result = puzzle_part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(result, 12240);
    }

    #[test]
    fn part_1_sample_f() {
        let result = puzzle_part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(result, 13632);
    }
}
