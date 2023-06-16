use std::{fmt::Result, fs::File, io::Read};

fn parse_line(text: &str) -> u32 {
    let mut result: u32 = 0;
    for char in text.chars() {
        let value = char::to_digit(char, 36).unwrap() as u8;
        let sub = value.saturating_sub(10) as u8;
        // println!("char:{char}, value:{value}, sub:{sub}");
        result |= (1 as u32) << sub;
    }
    return result;
}

fn main() {
    let mut infile = File::open("./day_06/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let groups = whole_file.trim().split("\n\n");

    assert_eq!(parse_line("a").count_ones(), 1);
    assert_eq!(parse_line("ab").count_ones(), 2);
    assert_eq!(parse_line("abc").count_ones(), 3);
    assert_eq!(parse_line("aaa").count_ones(), 1);
    assert_eq!(parse_line("aaabbb").count_ones(), 2);
    assert_eq!(parse_line("abcdefghijklmnopqrstuvwxyz").count_ones(), 26);

    let mut total = 0;
    for group in groups {
        let mut group_total = u32::MAX;
        for line in group.trim().split("\n") {
            group_total &= parse_line(line.trim());
        }
        let group_total_value = group_total.count_ones();
        println!("group_total_value:{group_total_value}");
        total += group_total_value;
    }
    println!("total:{total}");
}
