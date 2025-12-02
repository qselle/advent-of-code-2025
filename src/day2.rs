use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input.split(',').map(|s| {
        let tmp = s.split_once('-').unwrap();
        (tmp.0.parse::<usize>().unwrap(), tmp.1.parse::<usize>().unwrap())
    }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(usize, usize)]) -> usize{
    println!("{:#?}", input);
    input.iter().fold(0, |acc, x| {
        acc
    })
}
//
// #[aoc(day2, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }
//

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        println!("{:#?}",part1(&parse(INPUT)));
        // assert_eq!(&parse("<"), "<RESULT>");
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}