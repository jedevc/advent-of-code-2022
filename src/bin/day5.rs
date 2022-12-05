use std::{collections::BTreeMap, convert::identity};

use advent_of_code_2022::{main, Solver};

struct Day5 {
    crates: BTreeMap<char, Vec<char>>,
    instructions: Vec<(usize, char, char)>,
}

impl Solver<String> for Day5 {
    fn new(problem: &str) -> Self {
        let (crate_lines, instruction_lines) = problem.split_once("\n\n").unwrap();
        let crate_lines: Vec<&str> = crate_lines.lines().collect();
        let crate_idxs: Vec<Option<char>> = crate_lines[crate_lines.len() - 1]
            .chars()
            .map(|c| if c.is_alphanumeric() { Some(c) } else { None })
            .collect();

        let mut crates: BTreeMap<char, Vec<char>> = crate_idxs
            .iter()
            .cloned()
            .filter_map(identity)
            .map(|c| (c, vec![]))
            .collect();
        for line in crate_lines[..crate_lines.len() - 1].iter().rev() {
            for (c, idx) in line.chars().zip(0..) {
                if !c.is_alphanumeric() {
                    continue;
                }
                if let Some(k) = crate_idxs[idx] {
                    crates.get_mut(&k).unwrap().push(c);
                }
            }
        }

        let instructions = instruction_lines
            .lines()
            .map(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                (
                    words[1].parse().unwrap(),
                    words[3].parse().unwrap(),
                    words[5].parse().unwrap(),
                )
            })
            .collect();

        Day5 {
            crates,
            instructions,
        }
    }

    fn solve(&self) -> (Option<String>, Option<String>) {
        let mut crates = self.crates.clone();
        for (count, from, to) in &self.instructions {
            for _ in 0..*count {
                let item = crates.get_mut(from).unwrap().pop().unwrap();
                crates.get_mut(to).unwrap().push(item);
            }
        }
        let part1 = crates
            .iter()
            .map(|(_, boxes)| boxes[boxes.len() - 1])
            .collect();

        let mut crates = self.crates.clone();
        for (count, from, to) in &self.instructions {
            let boxes: Vec<char> = {
                let from = crates.get_mut(from).unwrap();
                let boxes = from.splice(from.len() - *count.., vec![]);
                boxes.collect()
            };
            let to = crates.get_mut(to).unwrap();
            to.extend(boxes);
        }
        let part2 = crates
            .iter()
            .map(|(_, boxes)| boxes[boxes.len() - 1])
            .collect();

        (Some(part1), Some(part2))
    }
}

#[cfg(test)]
mod day5tests {
    use super::*;
    use advent_of_code_2022::test_example;

    test_example!(
        Day5,
        "day5.example.txt",
        (Some("CMZ".to_string()), Some("MCD".to_string()))
    );
}

main!(Day5, "day5.txt");
