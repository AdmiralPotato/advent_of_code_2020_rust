use std::{collections::HashSet, fs::File, io::Read};

const SAMPLE: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

fn find_source_pair(target: &usize, preamble: &[usize]) -> Option<(usize, usize)> {
    let size = preamble.len();
    for index_a in 0..(size - 1) {
        for index_b in (index_a + 1)..size {
            let a = preamble[index_a];
            let b = preamble[index_b];
            let c = a + b;
            let correct = c == *target;
            // println!(
            //     "Comparing indices: {index_a}/{index_b} | {a} + {b} = {c} === {target}?{correct}"
            // );
            if correct {
                return Some((a, b));
            }
        }
    }
    return None;
}

fn puzzle_part_1(text: &str, preamble_size: usize) -> usize {
    let numbers: Vec<usize> = text
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect();
    for (index, number) in numbers[preamble_size..].iter().enumerate() {
        let result = find_source_pair(number, &numbers[index..]);
        match result {
            Some(x) => {
                // println!("Valid number: {x:?}");
            }
            None => {
                println!("Invalid number: {number:?}");
                return *number;
            }
        }
    }
    panic!("Result not found. Something is wrong.")
}

fn main() {
    println!("---- SAMPLE ----");
    assert_eq!(puzzle_part_1(SAMPLE, 5), 127);

    println!("---- REAL PROGRAM ----");
    let mut infile = File::open("./day_09/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str(), 25);
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = 70639851;
}
