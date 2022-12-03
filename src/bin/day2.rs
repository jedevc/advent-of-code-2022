use std::cmp::{Ordering,PartialOrd};
use advent_of_code_2022::{main, Solver};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if *self == Choice::Rock && *other == Choice::Paper {
            Some(Ordering::Less)
        } else if *self == Choice::Paper && *other == Choice::Scissors {
            Some(Ordering::Less)
        } else if *self == Choice::Scissors && *other == Choice::Rock {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Choice {
    fn find(&self, order: Ordering) -> Choice {
        match order {
            Ordering::Equal => *self,
            Ordering::Greater => 
                match self {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissors,
                    Choice::Scissors => Choice::Rock,
                },
            Ordering::Less => 
                match self {
                    Choice::Rock => Choice::Scissors,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissors => Choice::Paper,
                },
        }
    }
}


#[derive(Debug)]
struct Match {
    mine: Choice,
    yours: Choice,
}

impl Match {
    fn score(&self) -> i64 {
        let base = match self.mine {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        let win = match self.mine.partial_cmp(&self.yours) {
            None => 0,
            Some(Ordering::Less) => 0,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
        };
        return base + win
    }
}

#[derive(Debug)]
struct Target {
    yours: Choice,
    goal: Ordering,
}

impl Target {
    fn score(&self) -> i64 {
        let base = match self.yours.find(self.goal) {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        let win = match self.goal {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        return base + win
    }
}

struct Day2 {
    matches: Vec<Match>,
    targets: Vec<Target>,
}

impl Solver<i64> for Day2 {
    fn new(problem: &str) -> Self {
        let matches = problem.lines().map(|line| {
            let (yours, mine) = line.split_once(" ").unwrap();
            Match {
                mine: match mine {
                    "X" => Choice::Rock,
                    "Y" => Choice::Paper,
                    "Z" => Choice::Scissors,
                    _ => panic!("unknown choice"),
                },
                yours: match yours {
                    "A" => Choice::Rock,
                    "B" => Choice::Paper,
                    "C" => Choice::Scissors,
                    _ => panic!("unknown choice"),
                },
            }
        }).collect();

        let targets = problem.lines().map(|line| {
            let (yours, goal) = line.split_once(" ").unwrap();
            Target {
                goal: match goal {
                    "X" => Ordering::Less,
                    "Y" => Ordering::Equal,
                    "Z" => Ordering::Greater,
                    _ => panic!("unknown goal"),
                },
                yours: match yours {
                    "A" => Choice::Rock,
                    "B" => Choice::Paper,
                    "C" => Choice::Scissors,
                    _ => panic!("unknown choice"),
                },
            }
        }).collect();

        Day2 { matches, targets }
    }

    fn solve(&self) -> (Option<i64>, Option<i64>) {
        let part1 = self.matches.iter().map(Match::score).sum();
        let part2 = self.targets.iter().map(Target::score).sum();
        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day2tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(Day2, "day2.example.txt", (Some(15), Some(12)));
}

main!(Day2, "day2.txt");
