use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::Read,
};

#[derive(PartialEq)]
struct Board {
    seats: Vec<char>,
    width: usize,
    height: usize,
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for row in self.seats.chunks_exact(self.width) {
            write!(f, "{}\n", row.iter().collect::<String>())?;
        }
        return Ok(());
    }
}
impl Board {
    pub fn new(text: &str) -> Board {
        let width: usize = text.trim().find("\n").expect("could not find line break");
        let seats: Vec<char> = text.replace("\n", "").chars().collect();
        let height = seats.len() / width;
        assert_eq!(width * height, seats.len(), "not a rectangular board! >:(");
        return Board {
            seats,
            width,
            height,
        };
    }
    pub fn get_seat_char(&self, index: usize) -> char {
        return self.seats[index];
    }
    pub fn get_seat_value(&self, index: usize) -> u8 {
        seat_char_to_u8(self.get_seat_char(index))
    }

    pub fn get_neighbor_count(&self, index: usize) -> u8 {
        let width = self.width;
        let state = &self.seats;
        let row = index / width;
        let col = index % width;
        let mut result: u8 = 0;

        if row > 0 {
            // ↖
            if col > 0 {
                result += self.get_seat_value(index - width - 1);
            }
            // ⬆
            result += self.get_seat_value(index - width);
            // ↗
            if col < (width - 1) {
                result += self.get_seat_value(index - width + 1);
            }
        }

        // ⬅
        if col > 0 {
            result += self.get_seat_value(index - 1);
        }
        // ➡
        if col < (width - 1) {
            result += self.get_seat_value(index + 1);
        }

        if row < self.height - 1 {
            // ↙
            if col > 0 {
                result += self.get_seat_value(index + width - 1);
            }
            // ⬇
            result += self.get_seat_value(index + width);
            // ↙
            if col < (width - 1) {
                result += self.get_seat_value(index + width + 1);
            }
        }

        return result;
    }

    pub fn count_occupied_seats(&self) -> usize {
        let result: usize = self.seats.iter().fold(0, |acc: usize, c: &char| {
            acc + match c {
                '#' => 1,
                _ => 0,
            }
        });
        return result;
    }

    pub fn step_state(&self) -> Self {
        let mut next_state = vec!['.'; self.seats.len()];
        for (index, seat) in self.seats.iter().enumerate() {
            let neighbors = self.get_neighbor_count(index);
            match (seat, neighbors) {
                ('L', 0) => next_state[index] = '#',
                ('#', x) if x >= 4 => next_state[index] = 'L',
                (seat_value, _) => next_state[index] = *seat_value,
            }
        }
        return Board {
            width: self.width,
            height: self.height,
            seats: next_state,
        };
    }
}

fn seat_char_to_u8(seat: char) -> u8 {
    return match seat {
        '#' => 1,
        _ => 0,
    };
}

fn puzzle_part_1(text: &str) -> usize {
    let mut state = Board::new(text);
    loop {
        println!("state:\n{state}");
        let new_state = state.step_state();
        if new_state == state {
            // state is stable
            break;
        }
        state = new_state
    }
    return state.count_occupied_seats();
}

fn main() {
    let mut infile = File::open("./day_11/input.txt").expect("could not find file");
    let mut whole_file = String::new();
    infile
        .read_to_string(&mut whole_file)
        .expect("could not read file");
    let result = puzzle_part_1(whole_file.as_str());
    println!("PART 1 SOLUTION!!! {result}");
    // correct answer = 2194;
}

#[cfg(test)]
mod test {
    use super::*;
    const NO_NEIGHBORS_A: &str = "
...
.#.
...
";
    const NO_NEIGHBORS_B: &str = "
LLL
L#L
LLL
";
    const OOPS_ALL_NEIGHBORS: &str = "
###
###
###
";
    const SAMPLE_A: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    #[test]
    fn no_neighbors_a() {
        let board = Board::new(NO_NEIGHBORS_A);
        assert_eq!(board.get_neighbor_count(0), 1, "Testing NO_NEIGHBORS_A 1");
        assert_eq!(board.get_neighbor_count(4), 0, "Testing NO_NEIGHBORS_A 0");
    }
    #[test]
    fn count_occupied_seats() {
        let board = Board::new(NO_NEIGHBORS_A);
        assert_eq!(
            board.count_occupied_seats(),
            1,
            "Testing count_occupied_seats on NO_NEIGHBORS_A"
        );
        let board = Board::new(NO_NEIGHBORS_B);
        assert_eq!(
            board.count_occupied_seats(),
            1,
            "Testing count_occupied_seats on NO_NEIGHBORS_B"
        );
        let board = Board::new(OOPS_ALL_NEIGHBORS);
        assert_eq!(
            board.count_occupied_seats(),
            9,
            "Testing count_occupied_seats on OOPS_ALL_NEIGHBORS"
        );
    }
    #[test]
    fn no_neighbors_b() {
        let board = Board::new(NO_NEIGHBORS_B);
        assert_eq!(board.get_neighbor_count(0), 1, "Testing NO_NEIGHBORS_B 1");
        assert_eq!(board.get_neighbor_count(4), 0, "Testing NO_NEIGHBORS_B 0");
    }
    #[test]
    fn oops_all_neighbors() {
        let board = Board::new(OOPS_ALL_NEIGHBORS);
        assert_eq!(
            board.get_neighbor_count(0),
            3,
            "Testing OOPS_ALL_NEIGHBORS 3"
        );
        assert_eq!(
            board.get_neighbor_count(5),
            5,
            "Testing OOPS_ALL_NEIGHBORS 5"
        );
        assert_eq!(
            board.get_neighbor_count(4),
            8,
            "Testing OOPS_ALL_NEIGHBORS 8"
        );
    }
    #[test]
    fn sample() {
        assert_eq!(
            puzzle_part_1(SAMPLE_A),
            37,
            "correct stabilized value not found in sample"
        );
    }
}
