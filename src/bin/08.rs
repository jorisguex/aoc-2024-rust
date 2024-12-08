use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Map {
    size: i32,
    antenna: HashMap<char, HashSet<(i32, i32)>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let size = lines.len() as i32;
        let mut antenna = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(line.len() as i32, size);
            for (j, char) in line.chars().enumerate() {
                if char != '.' {
                    antenna
                        .entry(char)
                        .or_insert(HashSet::new())
                        .insert((i as i32, j as i32));
                }
            }
        }
        Ok(Map { size, antenna })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from_str(input).unwrap();
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for v in map.antenna.values() {
        for pair in v.iter().combinations(2).collect::<Vec<_>>() {
            let x_diff = pair[0].0 - pair[1].0;
            let y_diff = pair[0].1 - pair[1].1;
            let new_1 = (pair[0].0 + x_diff, pair[0].1 + y_diff);
            let new_2 = (pair[1].0 - x_diff, pair[1].1 - y_diff);
            if (0..map.size).contains(&new_1.0) && (0..map.size).contains(&new_1.1) {
                anti_nodes.insert(new_1);
            }
            if (0..map.size).contains(&new_2.0) && (0..map.size).contains(&new_2.1) {
                anti_nodes.insert(new_2);
            }
        }
    }
    Some(anti_nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_str(input).unwrap();
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for v in map.antenna.values() {
        for pair in v.iter().combinations(2).collect::<Vec<_>>() {
            let x_diff = pair[0].0 - pair[1].0;
            let y_diff = pair[0].1 - pair[1].1;
            let mut new_1 = (pair[0].0, pair[0].1);
            let mut new_2 = (pair[1].0, pair[1].1);
            while (0..map.size).contains(&new_1.0) && (0..map.size).contains(&new_1.1) {
                anti_nodes.insert(new_1);
                new_1.0 += x_diff;
                new_1.1 += y_diff;
            }
            while (0..map.size).contains(&new_2.0) && (0..map.size).contains(&new_2.1) {
                anti_nodes.insert(new_2);
                new_2.0 -= x_diff;
                new_2.1 -= y_diff;
            }
        }
    }
    Some(anti_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
