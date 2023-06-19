use regex::Regex;
use std::{collections::HashMap, fmt::Result, fs::File, io::Read};

struct Bag<'a> {
    name: &'a str,
    count_a: &'a str,
    label_a: &'a str,
    count_b: &'a str,
    label_b: &'a str,
    count_c: &'a str,
    label_c: &'a str,
    count_d: &'a str,
    label_d: &'a str,
}
impl<'a> Bag<'a> {
    fn new(text: &'a str) -> Self {
        // this is the worst regex I have ever had to write
        let crime = Regex::new(r"(?P<name>.*) bags contain ((?P<count_a>\d*) (?P<label_a>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags)( (?P<count_b>\d*) (?P<label_b>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}( (?P<count_c>\d*) (?P<label_c>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}( (?P<count_d>\d*) (?P<label_d>\w* \w*) bag[s]{0,1}[,]{0,1}|no other bags){0,1}").unwrap();
        let matches = crime.is_match(text);
        // println!("text: {text}, matches: {matches}");
        let captures = crime.captures(text).unwrap();
        let name = captures.name("name").map_or("", |m| m.as_str());
        let count_a = captures.name("count_a").map_or("", |m| m.as_str());
        let label_a = captures.name("label_a").map_or("", |m| m.as_str());
        let count_b = captures.name("count_b").map_or("", |m| m.as_str());
        let label_b = captures.name("label_b").map_or("", |m| m.as_str());
        let count_c = captures.name("count_c").map_or("", |m| m.as_str());
        let label_c = captures.name("label_c").map_or("", |m| m.as_str());
        let count_d = captures.name("count_d").map_or("", |m| m.as_str());
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

fn main() {
    let mut infile = File::open("./day_07/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let mut bag_map = HashMap::new();
    let lines = whole_file.trim().split("\n");
    let mut results = Vec::<&str>::new();
    let mut current_search = Vec::<&str>::new();
    for line in lines {
        let bag = Bag::new(line);
        if bag.label_a == "shiny gold"
            || bag.label_b == "shiny gold"
            || bag.label_c == "shiny gold"
            || bag.label_d == "shiny gold"
        {
            // results.push(bag.name);
            current_search.push(bag.name);
        }
        bag_map.insert(bag.name, bag);
    }
    let mut keep_going = true;
    while keep_going {
        let mut next_search = Vec::<&str>::new();
        for entry in &bag_map {
            // println!("&entry.1.name: {:?}", &entry.1.name);
            if !results.contains(&entry.1.name)
                && !next_search.contains(&entry.1.name)
                && !current_search.contains(&entry.1.name)
                && (current_search.contains(&entry.1.label_a)
                    || current_search.contains(&entry.1.label_b)
                    || current_search.contains(&entry.1.label_c)
                    || current_search.contains(&entry.1.label_d))
            {
                println!("Term added: {:?}", &entry.1.name);
                next_search.push(&entry.1.name);
            }
        }
        println!("next_search: {:?}", next_search);
        println!("current_search: {:?}", current_search);
        results = [results, current_search].concat();
        println!("results: {:?}", results);
        keep_going = next_search.len() > 0;
        current_search = next_search;
    }
    println!("----- LOOP OVER ------");
    println!("bag_map.keys().len(): {:?}", bag_map.keys().len());
    println!("current_search: {:?}", current_search);
    println!("results: {:?}", results);
    // 11 is not the right answer
    println!("results.len(): {}", results.len());
}
