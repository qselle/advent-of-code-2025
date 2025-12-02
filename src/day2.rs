use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|s| {
            let tmp = s.split_once('-').unwrap();
            (
                tmp.0.parse::<usize>().unwrap(),
                tmp.1.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|x| x.to_string().len().is_multiple_of(2))
        .filter(|x| {
            let divisor = 10_usize.pow((x.to_string().len() / 2) as u32);
            x.rem_euclid(divisor) == x.div_euclid(divisor)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|x| {
            let s = x.to_string();
            for n in 1..=(s.len() / 2) {
                let mut chunks = s.as_bytes().chunks(n);
                let first = chunks.next().unwrap();
                if chunks.all(|c| c == first) {
                    return true;
                }
            }
            false
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 4174379265);
    }
}
