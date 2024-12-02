advent_of_code::solution!(2);
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn report_safe(report: &[i32]) -> bool {
    let mut prev_sign = 0;
    for (r1, r2) in report.iter().tuple_windows::<(_, _)>() {
        let diff = r2 - r1;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        let sign = diff.signum();
        if prev_sign + sign == 0 {
            return false;
        }
        prev_sign = sign;
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_input(input).iter().filter(|r| report_safe(r)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for r in parse_input(input) {
        for i in 0..r.len() {
            let mut new_r = r.clone();
            new_r.remove(i);
            if report_safe(&new_r) {
                sum += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
