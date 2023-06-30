use std::collections::HashMap;
use std::{fs::File, io::Read};

struct ParseyDoodleDoo {
    data: HashMap<u16, u64>,
    mask_keep: u64,
    mask_reject: u64,
    mask_value: u64,
}

impl ParseyDoodleDoo {
    fn new() -> ParseyDoodleDoo {
        ParseyDoodleDoo {
            data: HashMap::new(),
            mask_keep: 0,
            mask_reject: 0,
            mask_value: 0,
        }
    }
    fn set_mask(&mut self, mask_str: &str) {
        let mask_keep = mask_str.replace("1", "0").replace("X", "1");
        let mask_reject = mask_str.replace("0", "1").replace("X", "0");
        let mask_value = mask_str.replace("X", "0");
        println!(
            "
       mask_str: {mask_str},
  mask_keep_str: {mask_keep},
mask_reject_str: {mask_reject},
 mask_value_str: {mask_value},
"
        );
        self.mask_keep = u64::from_str_radix(&mask_keep, 2).expect("Invalid mask_keep");
        self.mask_reject = u64::from_str_radix(&mask_reject, 2).expect("Invalid mask_reject");
        self.mask_value = u64::from_str_radix(&mask_value, 2).expect("Invalid mask_value");

        println!(
            "
  mask_keep: {},
mask_reject: {},
 mask_value: {},
",
            self.mask_keep, self.mask_reject, self.mask_value,
        );
    }
    fn set_value(&mut self, address_str: &str, value_str: &str) -> u64 {
        println!("address_str: {address_str}, value_str: {value_str}");
        let address: u16 = u16::from_str_radix(&address_str, 10).expect("Invalid destination");
        let incoming_value = u64::from_str_radix(&value_str, 10).expect("Invalid bits_str");
        println!("                    incoming_value: {incoming_value:#036b}");
        // let oof = 0;
        // let mut value: u64 = *self.data.get(&destination).unwrap_or(&oof);
        let mut value = incoming_value;
        value &= self.mask_keep;
        println!("             value after mask_keep: {value:#036b}");
        value |= self.mask_value;
        println!("            value after mask_value: {value:#036b}");
        self.data.insert(address, value);
        println!("                   value after all: {value}");
        value
    }
    fn sum_memory(&self) -> u64 {
        Vec::from_iter(&self.data)
            .iter()
            .fold(0, |acc: u64, (address, value)| {
                let result = acc + *value;
                println!("Totaling: address:{address}, value:{value}, result:{result}");
                result
            })
    }
}

fn puzzle_part_1(text: &str) -> u64 {
    let lines = text.trim().split('\n').into_iter();
    let mut parser = ParseyDoodleDoo::new();
    // format:
    // mask = X1100010011001001XX0100000X11X1XX00X
    // mem[42801] = 57157
    for line in lines {
        if line.contains("mask") {
            parser.set_mask(&line.replace("mask = ", ""));
        } else {
            let cleaned = line.replace("mem[", "").replace("]", "");
            let mut split = cleaned.split(" = ");
            parser.set_value(
                &split.next().expect("Unable to read destination"),
                &split.next().expect("Unable to read value"),
            );
        }
    }
    return parser.sum_memory();
}

fn main() {
    let mut infile = File::open("./day_14/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = 10050490168421;
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
    #[test]
    fn part_1() {
        assert_eq!(puzzle_part_1(SAMPLE), 165);
    }
}
