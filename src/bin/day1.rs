use advent_of_code_2022::{main, Solver};

struct Day1 {}

impl Solver<i64> for Day1 {
    fn new(_problem: &str) -> Self {
        Day1 {}
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        (None, None)
    }
}

#[cfg(test)]
mod day1tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(Day1, "day1.txt", (None, None));
}

main!(Day1, "day1.txt");
