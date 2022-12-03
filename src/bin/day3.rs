use std::collections::BTreeSet;

use advent_of_code_2022::{main, Solver};

struct Day3 {
    backpacks: Vec<(String, String)>,
}

impl Solver<i64> for Day3 {
    fn new(problem: &str) -> Self {
        let backpacks = problem
            .lines()
            .map(|line| {
                (
                    line[..line.len() / 2].to_string(),
                    line[line.len() / 2..].to_string(),
                )
            })
            .collect();
        Day3 { backpacks }
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        let part1 = self
            .backpacks
            .iter()
            .map(|(a, b)| find_common(a.chars().collect(), b.chars().collect())[0])
            .map(priority)
            .sum();

        let part2 = self
            .backpacks
            .chunks_exact(3)
            .map(|chunks| {
                let mut items = chunks[0].0.chars().chain(chunks[0].1.chars()).collect();
                for chunk in &chunks[1..] {
                    items = find_common(items, chunk.0.chars().chain(chunk.1.chars()).collect());
                }
                items[0]
            })
            .map(priority)
            .sum();

        (Some(part1), Some(part2))
    }
}

fn find_common(s1: Vec<char>, s2: Vec<char>) -> Vec<char> {
    let b1: BTreeSet<char> = s1.into_iter().collect();
    let b2: BTreeSet<char> = s2.into_iter().collect();
    b1.intersection(&b2).map(|ch| *ch).collect()
}

fn priority(ch: char) -> i64 {
    match ch {
        'a'..='z' => ch as i64 - 'a' as i64 + 1,
        'A'..='Z' => ch as i64 - 'A' as i64 + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod day3tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(Day3, "day3.example.txt", (Some(157), Some(70)));
}

main!(Day3, "day3.txt");
