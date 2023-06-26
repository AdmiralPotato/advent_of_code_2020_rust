use std::{fmt::Result as FmtResult, fs::File, io::Read};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(PartialEq, Clone, Debug)]
struct Ship {
    direction: Direction,
    x: i32,
    y: i32,
}

fn get_relative_direction(current: Direction, value: i32) -> Direction {
    let start: i32 = match current {
        Direction::East => 0,
        Direction::South => 90,
        Direction::West => 180,
        Direction::North => 270,
    };

    let new = (start + value + 360) % 360;
    match new {
        0 => Direction::East,
        90 => Direction::South,
        180 => Direction::West,
        270 => Direction::North,
        x => panic!("invalid direction! {x}"),
    }
}
fn move_relative(ship: &mut Ship, value: i32) {
    match ship.direction {
        Direction::East => ship.x += value,
        Direction::South => ship.y += value,
        Direction::West => ship.x -= value,
        Direction::North => ship.y -= value,
    };
}

impl Ship {
    fn new() -> Ship {
        return Ship {
            direction: Direction::East,
            x: 0,
            y: 0,
        };
    }
    fn process_instruction(&self, text: &str) -> Ship {
        let mut chars = text.chars().into_iter();
        let instruction: char = chars.next().expect("Cannot read instruction");
        let value: i32 = chars.as_str().parse().expect("Unable to parse value");
        let mut new_state = self.clone();
        match instruction {
            'N' => new_state.y -= value,
            'E' => new_state.x += value,
            'S' => new_state.y += value,
            'W' => new_state.x -= value,
            'L' => new_state.direction = get_relative_direction(new_state.direction, -value),
            'R' => new_state.direction = get_relative_direction(new_state.direction, value),
            'F' => move_relative(&mut new_state, value),
            x => panic!("Invalid instruction! '{x}'"),
        }
        return new_state;
    }
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn puzzle_part_1(text: &str) -> i32 {
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut ship = Ship::new();
    for line in lines {
        ship = ship.process_instruction(line);
        println!("ship: {ship:?}, line: {line}");
    }
    return ship.manhattan();
}

fn main() {
    let mut infile = File::open("./day_12/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = ???;
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
F10
N3
F7
R90
F11
";

    #[test]
    fn movement_components() {
        let lines: Vec<&str> = SAMPLE.trim().split('\n').collect();
        let ship = Ship::new().process_instruction(lines[0]);
        assert_eq!((ship.x, ship.y), (10, 0), "F10");
        let ship = ship.process_instruction(lines[1]);
        assert_eq!((ship.x, ship.y), (10, -3), "N3");
        let ship = ship.process_instruction(lines[2]);
        assert_eq!((ship.x, ship.y), (17, -3), "F7");
        let ship = ship.process_instruction(lines[3]);
        assert_eq!(
            (ship.x, ship.y, ship.direction),
            (17, -3, Direction::South),
            "R90"
        );
        let ship = ship.process_instruction(lines[4]);
        assert_eq!((ship.x, ship.y), (17, 8), "F11");
        assert_eq!(ship.manhattan(), 25, "manhattan");
    }
    #[test]
    fn whole_enchilada() {
        assert_eq!(puzzle_part_1(SAMPLE), 25, "whole_enchilada");
    }
}
