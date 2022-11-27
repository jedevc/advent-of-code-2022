use std::fmt::{Debug, Display};

pub trait Solver<T> {
    fn new(problem: &str) -> Self;
    fn solve(&self) -> (Option<T>, Option<T>);
}

#[macro_export]
macro_rules! main {
    ($solver:ident, $filename:expr) => {
        pub fn main() {
            use std::fs;
            use std::path::Path;

            let filename = Path::new("resources").join($filename);
            let problem = fs::read_to_string(filename).unwrap();
            let problem = problem.trim();

            advent_of_code_2022::solver::run_solve::<$solver, _>(problem);
        }
    };
}

#[macro_export]
macro_rules! test_example {
    ($solver:ident, $filename:expr, $expected:expr) => {
        #[test]
        fn test_example() {
            use std::fs;
            use std::path::Path;

            let filename = Path::new("resources").join($filename);
            let problem = fs::read_to_string(filename).unwrap();
            let problem = problem.trim();

            advent_of_code_2022::solver::run_test::<$solver, _>(problem, $expected);
        }
    };
}

pub fn run_solve<S, T>(problem: &str)
where
    S: Solver<T>,
    T: Display,
{
    let solver = S::new(problem);
    let (part1, part2) = solver.solve();

    if let Some(result1) = part1 {
        println!("part 1: {}", result1);
    } else {
        println!("part 1 not implemented");
    }

    if let Some(result2) = part2 {
        println!("part 2: {}", result2);
    } else {
        println!("part 2 not implemented");
    }
}

pub fn run_test<S, T>(problem: &str, expected: (Option<T>, Option<T>))
where
    S: Solver<T>,
    T: Debug + Display + PartialEq,
{
    let solver = S::new(problem);
    assert_eq!(solver.solve(), expected);
}
