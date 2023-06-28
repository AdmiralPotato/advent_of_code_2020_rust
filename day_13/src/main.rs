use std::{fs::File, io::Read};

fn puzzle_part_1(text: &str) -> i32 {
    let mut split = text.trim().split('\n').into_iter();
    let available_at = split.next().expect("can't get start time");
    let available_at: i32 = available_at.trim().parse().expect("Can't parse start time");
    let busses = split.next().expect("can't get busses");
    println!("wtf? available_at(parsed): {available_at}; busses: {busses}");
    let busses: Vec<i32> = busses
        .trim()
        .split(',')
        .into_iter()
        .map(|text: &str| text.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();

    let mut smallest_bus_id = 0;
    let mut smallest_wait = available_at;
    for bus in busses {
        let current_wait_a = available_at % bus;
        let multiple = (available_at / bus) + 1;
        let next_arrival = multiple * bus;
        let current_wait_b = next_arrival - available_at;
        if current_wait_b <= smallest_wait {
            smallest_wait = smallest_wait.min(current_wait_b);
            smallest_bus_id = bus;
        }
        println!("bus: {bus}; current_wait_a: {current_wait_a}, current_wait_b: {current_wait_b}, multiple: {multiple} next_arrival: {next_arrival}");
    }
    println!("smallest_bus_id: {smallest_bus_id}; smallest_wait: {smallest_wait}");
    return smallest_bus_id * smallest_wait;
}

fn main() {
    let mut infile = File::open("./day_13/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = 2995;
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
939
7,13,x,x,59,x,31,19
";

    #[test]
    fn whole_enchilada() {
        assert_eq!(puzzle_part_1(SAMPLE), 295, "whole_enchilada");
    }
}
