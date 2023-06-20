use regex::Regex;
use std::{collections::HashMap, fmt::Result, fs::File, io::Read};

struct Bag<'a> {
    name: &'a str,
    count_a: u8,
    label_a: &'a str,
    count_b: u8,
    label_b: &'a str,
    count_c: u8,
    label_c: &'a str,
    count_d: u8,
    label_d: &'a str,
}
impl Bag<'_> {
    fn new(text: &str) -> Bag<'_> {
        // this is the worst regex I have ever had to write
        let crime = Regex::new(r"(?P<name>.*) bags contain ((?P<count_a>\d*) (?P<label_a>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags)( (?P<count_b>\d*) (?P<label_b>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}( (?P<count_c>\d*) (?P<label_c>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}( (?P<count_d>\d*) (?P<label_d>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}").unwrap();
        let matches = crime.is_match(text);
        // println!("text: {text}, matches: {matches}");
        let captures = crime.captures(text).unwrap();
        let name = captures.name("name").map_or("", |m| m.as_str());
        let count_a = captures
            .name("count_a")
            .map_or("0", |m| m.as_str())
            .parse::<u8>()
            .unwrap();
        let label_a = captures.name("label_a").map_or("", |m| m.as_str());
        let count_b = captures
            .name("count_b")
            .map_or("0", |m| m.as_str())
            .parse::<u8>()
            .unwrap();
        let label_b = captures.name("label_b").map_or("", |m| m.as_str());
        let count_c = captures
            .name("count_c")
            .map_or("0", |m| m.as_str())
            .parse::<u8>()
            .unwrap();
        let label_c = captures.name("label_c").map_or("", |m| m.as_str());
        let count_d = captures
            .name("count_d")
            .map_or("0", |m| m.as_str())
            .parse::<u8>()
            .unwrap();
        let label_d = captures.name("label_d").map_or("", |m| m.as_str());
        // println!("name: '{name}' | count_a: '{count_a}' | label_a: '{label_a}' | count_a: '{count_b}' | label_a: '{label_b}'");

        return Bag {
            name,
            count_a,
            label_a,
            count_b,
            label_b,
            count_c,
            label_c,
            count_d,
            label_d,
        };
    }
}

fn recurse_totals(name: &str, bag_map: &HashMap<&str, Bag>) -> usize {
    let bag = bag_map.get(name).unwrap();
    let mut result: usize = 1;
    // println!("&entry.1.name: {:?}", &entry.1.name);
    if bag.label_a != "" {
        let contents = recurse_totals(&bag.label_a, bag_map);
        result += bag.count_a as usize * contents;
    }
    if bag.label_b != "" {
        let contents = recurse_totals(&bag.label_b, bag_map);
        result += bag.count_b as usize * contents;
    }
    if bag.label_c != "" {
        let contents = recurse_totals(&bag.label_c, bag_map);
        result += bag.count_c as usize * contents;
    }
    if bag.label_d != "" {
        let contents = recurse_totals(&bag.label_d, bag_map);
        result += bag.count_d as usize * contents;
    }
    result
}

fn main() {
    let mut infile = File::open("./day_07/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let mut bag_map = HashMap::new();
    let lines = whole_file.trim().split("\n");
    let mut result = 0;
    for line in lines {
        let bag = Bag::new(line);
        bag_map.insert(bag.name, bag);
    }

    result += recurse_totals("shiny gold", &bag_map);

    println!("----- LOOP OVER ------");
    println!("bag_map.keys().len(): {:?}", bag_map.keys().len());
    // 150 is too low.
    // 5957 is too high.
    // 5956 is correct.
    // It took seeing that both sample1 and sample2 were both larger by 1.
    // the minus 1 is because I suppose I'm counting the root bag twice.
    println!("result: {:?}", result - 1);
}
