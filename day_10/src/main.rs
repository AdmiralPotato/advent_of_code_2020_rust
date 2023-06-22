use std::{fs::File, io::Read};

const SAMPLE_A: &str = "
16
10
15
5
1
11
7
19
6
12
4
";
const SAMPLE_B: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

fn puzzle_part_1(text: &str) -> u32 {
    let mut numbers: Vec<usize> = text
        .trim()
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);
    let mut tallies = (0, 0);
    for (index, number) in numbers[0..numbers.len() - 1].iter().enumerate() {
        let next = numbers[index + 1];
        let diff = next - number;
        match diff {
            1 => tallies.0 += 1,
            3 => tallies.1 += 1,
            x => panic!("got a diff that was not 1 or 3: {x}; last: {next}, number: {number}"),
        }
    }
    return tallies.0 * tallies.1;
}

fn main() {
    println!("---- SAMPLE ----");
    assert_eq!(puzzle_part_1(SAMPLE_A), 5 * 7);
    assert_eq!(puzzle_part_1(SAMPLE_B), 22 * 10);

    println!("---- REAL PROGRAM ----");
    let mut infile = File::open("./day_10/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = 2368;
}
