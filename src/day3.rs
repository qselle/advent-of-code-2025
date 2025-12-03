use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as usize)
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<usize>]) -> usize {
    println!("{:#?}", input);
    0
}

// #[aoc(day3, part2)]
// fn part2(input: &[(usize, usize)]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(&parse(INPUT)), 357);
    // }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(INPUT)), 357);
    // }
}
