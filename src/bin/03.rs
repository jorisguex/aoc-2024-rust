use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let re = regex::Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();
    for c in re.captures_iter(input) {
        sum += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap();
    }
    Some(sum)
}

#[derive(Debug)]
enum Match {
    Do,
    Dont,
    Mul(u32),
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hm: std::collections::BTreeMap<usize, Match> = std::collections::BTreeMap::new();

    let do_re = regex::Regex::new(r"do\(\)").unwrap();
    let dont_re = regex::Regex::new(r"don't\(\)").unwrap();
    let mul_re = regex::Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();

    for captures in do_re.captures_iter(input) {
        hm.insert(captures.get(0).unwrap().start(), Match::Do);
    }
    for captures in dont_re.captures_iter(input) {
        hm.insert(captures.get(0).unwrap().start(), Match::Dont);
    }
    for captures in mul_re.captures_iter(input) {
        hm.insert(
            captures.get(0).unwrap().start(),
            Match::Mul(captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap()),
        );
    }

    let mut enabled = true;
    let mut sum = 0;
    for val in hm.values() {
        match val {
            Match::Do => enabled = true,
            Match::Dont => enabled = false,
            Match::Mul(v) => {
                if enabled {
                    sum += v;
                }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
