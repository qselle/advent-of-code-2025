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
    input
        .iter()
        .map(|bank| {
            bank.iter()
                .enumerate()
                .flat_map(|(idx, &tens)| bank[idx + 1..].iter().map(move |&ones| tens * 10 + ones))
                .max()
                .unwrap()
        })
        .sum()

    // .map(|bank| {
    //     let mut batteries: Vec<(usize, &usize)> = bank.iter().enumerate().collect();
    //     batteries.sort_unstable_by(|a, b| b.1.cmp(a.1));
    //
    //     let mut num = 0;
    //     let mut last = 0;
    //     let mut count = 0;
    //     for (idx, digit) in batteries.iter() {
    //         if count == 0 && *idx == batteries.len() - 1 {
    //             continue;
    //         }
    //         if *idx < last {
    //             continue;
    //         }
    //         num = num * 10 + *digit;
    //         last = *idx;
    //         count += 1;
    //         if count == 2 {
    //             break;
    //         }
    //     }
    //
    //     // let mut top_k: Vec<(usize, &usize)> = items.into_iter().take(2).collect();
    //     // top_k.sort_by_key(|k| k.0);
    //
    //     // row.sort_unstable_by(|a, b| b.cmp(a));
    //     // let test = row[0] * 10 + row[1];
    //     // row.iter().position(|p| {
    //     //     if
    //     // })
    //
    //     // let test = top_k.iter().fold(0, |acc, k| acc * 10 + k.1);
    //     println!("top: {:#?}", num);
    //     num
    // })
    // .sum()
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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 357);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(INPUT)), 357);
    // }
}
