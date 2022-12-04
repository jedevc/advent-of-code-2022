use advent_of_code_2022::{main, Solver};

struct DayX {
}

impl Solver<i64> for DayX {
    fn new(problem: &str) -> Self {
        DayX {}
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        (None, None)
    }
}

#[cfg(test)]
mod dayxtests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(DayX, "dayx.example.txt", (None, None));
}

main!(DayX, "dayx.txt");
