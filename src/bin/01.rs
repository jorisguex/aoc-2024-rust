advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input.lines() {
        let mut parts = line.split("   ");
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    let mut sum = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        sum += l.abs_diff(*r);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let mut right_freq: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    for r in right {
        *right_freq.entry(r).or_insert(0) += 1;
    }
    let mut sum = 0;
    for l in left {
        sum += l * right_freq.get(&l).unwrap_or(&0);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}