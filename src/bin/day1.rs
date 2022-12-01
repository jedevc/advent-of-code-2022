use advent_of_code_2022::{main, Solver};

struct Day1 {
    calories: Vec<Vec<i64>>,
}

impl Solver<i64> for Day1 {
    fn new(problem: &str) -> Self {
        let calories = problem
            .split("\n\n")
            .map(|elf| elf.split("\n").map(|s| s.parse().unwrap()).collect())
            .collect();
        Day1 { calories: calories }
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        let mut counts: Vec<i64> = self
            .calories
            .iter()
            .map(|calories| calories.iter().sum())
            .collect();
        counts.sort();
        counts.reverse();

        let part1 = counts[0];
        let part2 = counts[..3].iter().sum();
        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day1tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(Day1, "day1.example.txt", (Some(24000), Some(45000)));
}

main!(Day1, "day1.txt");
