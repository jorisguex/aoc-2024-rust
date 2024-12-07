use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    size: usize,
    obstacles: Vec<(usize, usize)>,
    current_pos: (usize, usize),
    current_dir: Direction,
}

impl Map {
    fn next_step(&mut self) -> bool {
        let (pos_x, pos_y) = self.current_pos;
        let next_position = match self.current_dir {
            Direction::North => match pos_x {
                0 => return false,
                pos_x => (pos_x - 1, pos_y),
            },
            Direction::East => match pos_y {
                pos_y if pos_y == self.size - 1 => return false,
                pos_y => (pos_x, pos_y + 1),
            },
            Direction::South => match pos_x {
                pos_x if pos_x == self.size - 1 => return false,
                pos_x => (pos_x + 1, pos_y),
            },
            Direction::West => match pos_y {
                0 => return false,
                pos_y => (pos_x, pos_y - 1),
            },
        };
        if self.obstacles.contains(&next_position) {
            self.current_dir = self.current_dir.rotate();
        } else {
            self.current_pos = next_position;
        }
        true
    }

    fn get_path(&mut self) -> Option<std::collections::HashSet<(usize, usize)>> {
        let mut path: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut visited: std::collections::HashSet<((usize, usize), Direction)> =
            std::collections::HashSet::new();
        loop {
            path.insert(self.current_pos);
            if !visited.insert((self.current_pos, self.current_dir)) {
                return None;
            }
            if !self.next_step() {
                break;
            }
        }
        Some(path)
    }
}

#[derive(Debug)]
struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let size = lines.len();
        let mut obstacles = vec![];
        let mut current_pos = None;
        for (i, line) in lines.iter().enumerate() {
            if line.len() != size {
                return Err(ParseMapError);
            }
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => continue,
                    '#' => obstacles.push((i, j)),
                    '^' => current_pos = Some((i, j)),
                    _ => return Err(ParseMapError),
                }
            }
        }
        match current_pos {
            Some(current_pos) => Ok(Map {
                size,
                obstacles,
                current_pos,
                current_dir: Direction::North,
            }),
            None => Err(ParseMapError),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).unwrap();
    Some(map.get_path().unwrap().len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_str(input).unwrap();
    let path = map.clone().get_path().unwrap();
    let mut sum = 0;
    for p in path.iter().skip(1) {
        let mut new_map = map.clone();
        new_map.obstacles.push(*p);
        if new_map.get_path().is_none() {
            sum += 1;
        };
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
