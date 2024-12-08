use core::num;
use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
    Concat,
}

#[derive(Debug)]
struct Equation {
    solution: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn test_operators(&self, operators: Vec<Operator>) -> bool {
        let mut solution = *self.numbers.first().unwrap();
        for (i, num) in self.numbers.iter().skip(1).enumerate() {
            if solution > self.solution {
                break;
            }
            match operators[i] {
                Operator::Plus => solution += num,
                Operator::Times => solution *= num,
                Operator::Concat => solution = format!("{solution}{num}").parse().unwrap(),
            }
        }
        solution == self.solution
    }
}

impl FromStr for Equation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let (solution, numbers) = s.split_once(": ").unwrap();
        Ok(Equation {
            solution: solution.parse()?,
            numbers: numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let equation = Equation::from_str(line).unwrap();
        let num_operators = equation.numbers.len() - 1;
        for operators in (0..num_operators)
            .map(|_| [Operator::Plus, Operator::Times].into_iter())
            .multi_cartesian_product()
        {
            let result = equation.test_operators(operators.clone());
            if result {
                sum += equation.solution;
                break;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let equation = Equation::from_str(line).unwrap();
        let num_operators = equation.numbers.len() - 1;
        for operators in (0..num_operators)
            .map(|_| [Operator::Plus, Operator::Times, Operator::Concat].into_iter())
            .multi_cartesian_product()
        {
            let result = equation.test_operators(operators.clone());
            if result {
                sum += equation.solution;
                break;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
