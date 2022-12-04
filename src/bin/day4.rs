use advent_of_code_2022::{main, Solver};
use std::convert::identity;
use std::ops::RangeInclusive;

struct Day4 {
    assignments: Vec<Assignment>,
}

struct Assignment {
    first: RangeInclusive<i64>,
    second: RangeInclusive<i64>,
}

impl Solver<i64> for Day4 {
    fn new(problem: &str) -> Self {
        let assignments = problem
            .lines()
            .map(|line| line.split_once(","))
            .filter_map(identity)
            .filter_map(|(first, second)| {
                let first = first.split_once("-")?;
                let second = second.split_once("-")?;
                let assign = Assignment {
                    first: RangeInclusive::new(
                        first.0.parse().ok()?,
                        first.1.parse().ok()?,
                    ),
                    second: RangeInclusive::new(
                        second.0.parse().ok()?,
                        second.1.parse().ok()?,
                    ),
                };
                Some(assign)
            })
            .collect();
        Day4 { assignments }
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        let part1 = self
            .assignments
            .iter()
            .filter_map(|assign| {
                if assign.first.contains(&assign.second.start())
                    && assign.first.contains(&assign.second.end())
                {
                    Some(assign)
                } else if assign.second.contains(&assign.first.start())
                    && assign.second.contains(&assign.first.end())
                {
                    Some(assign)
                } else {
                    None
                }
            })
            .count() as i64;

        let part2 = self
            .assignments
            .iter()
            .filter_map(|assign| {
                if assign.first.contains(&assign.second.start()) {
                    Some(assign)
                } else if assign.first.contains(&assign.second.end()) {
                    Some(assign)
                } else if assign.second.contains(&assign.first.start()) {
                    Some(assign)
                } else if assign.second.contains(&assign.first.end()) {
                    Some(assign)
                } else {
                    None
                }
            })
            .count() as i64;

        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day4tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(Day4, "day4.example.txt", (Some(2), Some(4)));
}

main!(Day4, "day4.txt");
