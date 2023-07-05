use std::collections::HashMap;

fn part_1(input: &Vec<u32>, steps: u32) -> u32 {
    if steps <= input.len() as u32 {
        return input[steps as usize - 1];
    }
    let mut mappy_doodle_doo: HashMap<u32, u32> = HashMap::new();
    for (i, num) in input[..input.len() - 1].iter().enumerate() {
        mappy_doodle_doo.insert(*num, i as u32 + 1);
    }
    let mut next_said_number: u32 = *input.last().unwrap();
    for i in input.len() as u32..steps {
        let number_to_say_after_that = match mappy_doodle_doo.get(&next_said_number) {
            None => {
                // It was never said before
                // println!("It is step {i}. We're about to say {next_said_number} for the first time (so the *next* number we're going to say is 0).");
                0
            }
            Some(before) => {
                // It was said before
                // println!("It is step {i}. We're about to say {next_said_number}. It was last said on turn {before} (so the *next* number we're going to say is {}).", i - before);
                i - before
            }
        };
        // vvv This is the part where we "actually say" the number. vvv
        mappy_doodle_doo.insert(next_said_number, i);
        // println!(
        //     "Turn {}, next_said_number: {next_said_number}, mappy_doodle_doo: {mappy_doodle_doo:?}",
        //     i
        // );
        next_said_number = number_to_say_after_that;
    }
    next_said_number
}

fn main() {
    let result = part_1(&vec![2, 15, 0, 9, 1, 20], 2020);

    println!("PART 1 SOLUTION!!! {result}");
    // correct answer is 1280
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_1_sample_a() {
        let sample = vec![0, 3, 6];
        let sample_results = [
            // Turn 1: The 1st number spoken is a starting number, 0.
            (1, 0),
            // Turn 2: The 2nd number spoken is a starting number, 3.
            (2, 3),
            // Turn 3: The 3rd number spoken is a starting number, 6.
            (3, 6),
            // Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
            (4, 0),
            // Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
            (5, 3),
            // Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
            (6, 3),
            // Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
            (7, 1),
            // Turn 8: Since 1 is new, the 8th number spoken is 0.
            (8, 0),
            // Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
            (9, 4),
            // Turn 10: 4 is new, so the 10th number spoken is 0.
            (10, 0),
            // In the example above, the 2020th number spoken will be 436.
            (2020, 436),
        ];
        let mut ok = true;
        for (turn, result) in sample_results.into_iter() {
            let actual = part_1(&sample, turn);
            if actual != result {
                println!("turn {turn} is WRONG! should be {result}, is {actual}");
                ok = false;
            } else {
                println!("turn {turn} is correct!");
            }
        }
        if !ok {
            panic!("Some answers were wrong, see above.");
        }
    }
    /*
    Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
    */
    #[test]
    fn part_1_sample_b() {
        assert_eq!(part_1(&vec![1, 3, 2], 2020), 1);
    }
    #[test]
    fn part_1_sample_c() {
        assert_eq!(part_1(&vec![2, 1, 3], 2020), 10);
    }
    #[test]
    fn part_1_sample_d() {
        assert_eq!(part_1(&vec![1, 2, 3], 2020), 27);
    }
    #[test]
    fn part_1_sample_e() {
        assert_eq!(part_1(&vec![2, 3, 1], 2020), 78);
    }
    #[test]
    fn part_1_sample_f() {
        assert_eq!(part_1(&vec![3, 2, 1], 2020), 438);
    }
    #[test]
    fn part_1_sample_g() {
        assert_eq!(part_1(&vec![3, 1, 2], 2020), 1836);
    }
}
