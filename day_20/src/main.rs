use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

fn chars_to_u16(chars: &[char]) -> u16 {
    let line: String = chars
        .iter()
        .map(|c| match c {
            '.' => '0',
            '#' => '1',
            _ => panic!("NO! BAD CHAR!"),
        })
        .collect();
    u16::from_str_radix(&line, 2).expect("bad binary parse")
}
fn get_column(chars: &[char], x: usize) -> [char; 10] {
    let mut result: [char; 10] = ['.'; 10];
    for i in 0..10 {
        result[i] = chars[(i * 10) + x];
    }
    result
}
fn reverse_first_10_bits(n: u16) -> u16 {
    n.reverse_bits() >> 6
}

struct Tile {
    id: u16,
    cells: [char; 100],
    n: u16,
    e: u16,
    s: u16,
    w: u16,
    nr: u16,
    er: u16,
    sr: u16,
    wr: u16,
}
impl Tile {
    fn new(input: &str) -> Tile {
        let mut iter = input.split("\n");
        let id: u16 = iter
            .next()
            .expect("Cannot get ID line")
            .split_once(" ")
            .expect("Cannot get ID text")
            .1[..4]
            .parse()
            .expect("Cannot parse ID text");
        let cells: [char; 100] = iter
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("Bad cell size");
        let n = chars_to_u16(&cells[0..10]);
        let e = chars_to_u16(&get_column(&cells, 9));
        let s = chars_to_u16(&cells[90..100]);
        let w = chars_to_u16(&get_column(&cells, 0));
        return Tile {
            id,
            cells,
            n,
            e,
            s,
            w,
            nr: reverse_first_10_bits(n),
            er: reverse_first_10_bits(e),
            sr: reverse_first_10_bits(s),
            wr: reverse_first_10_bits(w),
        };
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grid: String = self
            .cells
            .iter()
            .enumerate()
            .fold("".to_string(), |prev, (i, char)| {
                if i % 10 == 9 {
                    format!("{prev}{char}\n")
                } else {
                    format!("{prev}{char}")
                }
            });
        write!(
            f,
            "id:{id}\n\
            sides - n:{n:010b}  e:{e:010b}  s:{s:010b}  w:{w:010b}\n\
            revs - nr:{nr:010b} er:{er:010b} sr:{sr:010b} wr:{wr:010b}\n\
            {grid}",
            id = self.id,
            n = self.n,
            e = self.e,
            s = self.s,
            w = self.w,
            nr = self.nr,
            er = self.er,
            sr = self.sr,
            wr = self.wr,
        )
    }
}

fn parse_input(input: &str) -> HashMap<u16, Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|item| {
            let result = Tile::new(item);
            (result.id, result)
        })
        .collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::parse_input;

    const SAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    #[test]
    fn process_chunks() {
        let result = parse_input(SAMPLE);
        println!("Debug? {result:?}");
        assert_eq!(
            result.get(&3079).unwrap().n,
            u16::from_str_radix(&"1010111110", 2).unwrap(),
            "bad north"
        );
        assert_eq!(
            result.get(&3079).unwrap().e,
            u16::from_str_radix(&"0100001000", 2).unwrap(),
            "bad east"
        );
        assert_eq!(
            result.get(&3079).unwrap().s,
            u16::from_str_radix(&"0010111000", 2).unwrap(),
            "bad south"
        );
        assert_eq!(
            result.get(&3079).unwrap().w,
            u16::from_str_radix(&"1001101000", 2).unwrap(),
            "bad west"
        );
    }
}
