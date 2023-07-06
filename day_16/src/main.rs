use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::{fs::File, io::Read};

struct ParsedTrainTickets<'a> {
    rules: HashMap<&'a str, (RangeInclusive<u16>, RangeInclusive<u16>)>,
    your_ticket: Vec<u16>,
    their_tickets: Vec<Vec<u16>>,
}

impl<'a> ParsedTrainTickets<'a> {
    fn new(input: &'a str) -> ParsedTrainTickets {
        let mut splits = input.trim().split("\n\n");
        let mut rules = HashMap::new();
        splits
            .next()
            .expect("Could not get rules")
            .split("\n")
            .for_each(|rule_string: &str| {
                let (label, segments) = Self::parse_rule(rule_string);
                rules.insert(label, segments);
            });
        let your_ticket: Vec<u16> = ParsedTrainTickets::parse_ticket(
            splits
                .next()
                .expect("Could not start your_ticket")
                .split("\n")
                .skip(1)
                .next()
                .expect("Could not start your_ticket"),
        );
        let their_tickets: Vec<Vec<u16>> = splits
            .next()
            .expect("Could not get rules")
            .split("\n")
            .skip(1)
            .map(|ticket_string| Self::parse_ticket(ticket_string))
            .collect();
        ParsedTrainTickets {
            rules,
            your_ticket,
            their_tickets,
        }
    }
    fn parse_rule_segment(text: &str) -> RangeInclusive<u16> {
        let range: Vec<u16> = text
            .trim()
            .split("-")
            .map(|t| t.parse().expect("parse range fail"))
            .collect();
        range[0]..=range[1]
    }
    fn parse_rule(text: &str) -> (&str, (RangeInclusive<u16>, RangeInclusive<u16>)) {
        let (label, segments) = text.split_once(":").expect("No can split rule from label");
        let (a, b) = segments
            .trim()
            .split_once(" or ")
            .expect("No can split rule segments");
        (
            label,
            (Self::parse_rule_segment(a), Self::parse_rule_segment(b)),
        )
    }
    fn parse_ticket(text: &str) -> Vec<u16> {
        text.trim()
            .split(",")
            .map(|t| t.parse().expect(&format!("parse ticket fail: {text}")))
            .collect()
    }
}

fn get_field_that_is_not_in_any_range(
    ticket: &Vec<u16>,
    rules: &HashMap<&str, (RangeInclusive<u16>, RangeInclusive<u16>)>,
) -> Option<u16> {
    for value in ticket {
        let mut any_rule_matched_yet = false;
        for (_key, (rule_0, rule_1)) in rules {
            if rule_0.contains(value) {
                println!("In ticket: {ticket:?}, the value: {value}, DID match rule_0: {rule_0:?}");
                any_rule_matched_yet = true;
            } else if rule_1.contains(value) {
                println!("In ticket: {ticket:?}, the value: {value}, DID match rule_1: {rule_1:?}");
                any_rule_matched_yet = true;
            }
        }
        if !any_rule_matched_yet {
            println!("In ticket: {ticket:?}, the value: {value}, did NOT match any rules");
            return Some(*value);
        }
    }
    None
}

fn puzzle_part_1(input: ParsedTrainTickets) -> u32 {
    input
        .their_tickets
        .iter()
        .fold(0 as u32, |total, ticket: &Vec<u16>| {
            total + (get_field_that_is_not_in_any_range(&ticket, &input.rules).unwrap_or(0) as u32)
        })
}

fn main() {
    let mut infile = File::open("./day_16/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");

    let parsed_train_tickets = ParsedTrainTickets::new(&whole_file.as_str());
    let result = puzzle_part_1(parsed_train_tickets);
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer is 25984
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_sample_a() {
        const SAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let input = ParsedTrainTickets::new(SAMPLE);
        let result = puzzle_part_1(input);
        assert_eq!(result, 71);
    }
}
