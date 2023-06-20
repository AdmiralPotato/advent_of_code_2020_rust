use std::{collections::HashSet, fs::File, io::Read};

const SAMPLE: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

fn add_offset(big_indexer: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        big_indexer.checked_sub(delta.wrapping_abs() as usize)
    } else {
        big_indexer.checked_add(delta as usize)
    }
}

// Immediately before any instruction is executed a second time, what value is in the accumulator?
fn puzzle_part_1(text: &str) -> i32 {
    let lines: Vec<&str> = text.trim().split("\n").collect();
    let mut lines_set: HashSet<usize> = HashSet::new();
    let mut step: usize = 0;
    let mut pos: usize = 0;
    let mut acc: i32 = 0;
    loop {
        if lines_set.contains(&pos) {
            break;
        }
        lines_set.insert(pos);
        let line: &str = lines.get(pos).unwrap();
        let mut split = line.split(' ');
        // ask Solra about destructuring from a split into 2 variables
        let opcode = split.next().unwrap();
        let number: i32 = split.next().unwrap().parse().unwrap();
        match opcode {
            "acc" => {
                acc += number;
                pos += 1;
            }
            "jmp" => {
                pos = add_offset(pos, number as isize).unwrap();
            }
            "nop" => {
                pos += 1;
            }
            _ => {
                assert!(false, "Bad instruction!")
            }
        }
        println!("step: {step} | acc: {acc} | pos: {pos} | opcode: {opcode} | number: {number}");
        step += 1;
    }
    println!("Program looped at step {step} with acc value of {acc}");
    return acc;
}

fn main() {
    println!("---- SAMPLE ----");
    assert_eq!(puzzle_part_1(SAMPLE), 5);

    println!("---- REAL PROGRAM ----");
    let mut infile = File::open("./day_08/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    puzzle_part_1(whole_file.as_str());
}
