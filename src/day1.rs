use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Direction{
    Left{sub: isize},
    Right{add: isize},
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Direction>  {
    input.lines().map(|l| {
        let (direction, steps) = l.split_at(1);
        if direction == "L" {
            Direction::Left {sub: steps.parse().unwrap()}
        } else {
            Direction::Right {add: steps.parse().unwrap()}
        }
    }).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Direction]) -> usize {
    let mut position = 50;
    let mut count = 0;

    for d in input {
        match d {
            Direction::Left { sub } => {
                position -= sub;
            }
            Direction::Right { add } => {
                position += add;
            }
        }
        position = position.rem_euclid(100);
        if position == 0 {
            count+= 1;
        }
    }
    count
}

// [-100 ; -1] = -1
// [0 ; 99] = 0
// [100 ; 199]= 1
// [200 ; 299]= 2
#[aoc(day1, part2)]
fn part2(input: &[Direction]) -> isize {
    let mut position: isize = 50;
    let mut count = 0;

    for d in input {
        match d {
            Direction::Left { sub } => {
                let start_bucket = (position - 1).div_euclid(100);
                position -= sub;
                let end_bucket = (position - 1).div_euclid(100);

                count += (start_bucket - end_bucket).abs();
            }
            Direction::Right { add } => {
                let start_bucket = position.div_euclid(100);
                position += add;
                let end_bucket = position.div_euclid(100);

                count += (end_bucket - start_bucket).abs();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 6);
    }

    const INPUT_BIS: &str = "L50
L100
R200
";

    #[test]
    fn part2_example_bis() {
        assert_eq!(part2(&parse(INPUT_BIS)), 4);
    }
}
