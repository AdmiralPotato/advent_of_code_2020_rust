use std::{fmt::Result as FmtResult, fs::File, io::Read};

#[derive(PartialEq, Clone, Debug)]
struct Ship {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

fn update_direction(ship: &mut Ship, value: i32) {
    let angle = (value + 360) % 360;
    match angle {
        0 => (),
        90 => {
            let temp = ship.waypoint_y;
            ship.waypoint_y = ship.waypoint_x;
            ship.waypoint_x = -temp;
        }
        180 => {
            ship.waypoint_x = -ship.waypoint_x;
            ship.waypoint_y = -ship.waypoint_y;
        }
        270 => {
            let temp = ship.waypoint_x;
            ship.waypoint_x = ship.waypoint_y;
            ship.waypoint_y = -temp;
        }
        x => panic!("invalid direction! {x}"),
    }
}
fn move_relative(ship: &mut Ship, value: i32) {
    for _ in 0..value {
        ship.x += ship.waypoint_x;
        ship.y += ship.waypoint_y;
    }
}

impl Ship {
    fn new() -> Ship {
        return Ship {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: -1,
        };
    }
    fn process_instruction(&self, text: &str) -> Ship {
        let mut chars = text.chars().into_iter();
        let instruction: char = chars.next().expect("Cannot read instruction");
        let value: i32 = chars.as_str().parse().expect("Unable to parse value");
        let mut new_state = self.clone();
        match instruction {
            'N' => new_state.waypoint_y -= value,
            'E' => new_state.waypoint_x += value,
            'S' => new_state.waypoint_y += value,
            'W' => new_state.waypoint_x -= value,
            'L' => update_direction(&mut new_state, -value),
            'R' => update_direction(&mut new_state, value),
            'F' => move_relative(&mut new_state, value),
            x => panic!("Invalid instruction! '{x}'"),
        }
        return new_state;
    }
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn puzzle_part_2(text: &str) -> i32 {
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
    let result = puzzle_part_2(whole_file.as_str());
    println!("PART 2 SOLUTION!!! {result}");
    // correct answer = 59435;
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
        assert_eq!((ship.x, ship.y), (100, -10), "F10");
        let ship = ship.process_instruction(lines[1]);
        assert_eq!((ship.waypoint_x, ship.waypoint_y), (10, -4), "N3");
        let ship = ship.process_instruction(lines[2]);
        assert_eq!((ship.x, ship.y), (170, -38), "F7");
        let ship = ship.process_instruction(lines[3]);
        assert_eq!((ship.waypoint_x, ship.waypoint_y), (4, 10), "R90");
        let ship = ship.process_instruction(lines[4]);
        assert_eq!((ship.x, ship.y), (214, 72), "F11");
        assert_eq!(ship.manhattan(), 286, "manhattan");
    }
    #[test]
    fn whole_enchilada() {
        assert_eq!(puzzle_part_2(SAMPLE), 286, "whole_enchilada");
    }
}
