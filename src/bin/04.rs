advent_of_code::solution!(4);

const GOAL1: (Option<char>, Option<char>, Option<char>, Option<char>) =
    (Some('X'), Some('M'), Some('A'), Some('S'));

const GOAL2: (Option<char>, Option<char>, Option<char>) = (Some('M'), Some('A'), Some('S'));

struct WordSearch1 {
    data: Vec<Vec<char>>,
}

struct WordSearch2 {
    data: Vec<Vec<char>>,
}

impl WordSearch1 {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).unwrap_or(&vec![]).get(x).copied()
    }

    fn check_horizontal(&self, x: usize, y: usize) -> bool {
        let a = self.get(x, y);
        let b = self.get(x + 1, y);
        let c = self.get(x + 2, y);
        let d = self.get(x + 3, y);
        (a, b, c, d) == GOAL1 || (d, c, b, a) == GOAL1
    }

    fn check_vertical(&self, x: usize, y: usize) -> bool {
        let a = self.get(x, y);
        let b = self.get(x, y + 1);
        let c = self.get(x, y + 2);
        let d = self.get(x, y + 3);
        (a, b, c, d) == GOAL1 || (d, c, b, a) == GOAL1
    }

    fn check_diagonal_1(&self, x: usize, y: usize) -> bool {
        let a = self.get(x, y);
        let b = self.get(x + 1, y + 1);
        let c = self.get(x + 2, y + 2);
        let d = self.get(x + 3, y + 3);
        (a, b, c, d) == GOAL1 || (d, c, b, a) == GOAL1
    }

    fn check_diagonal_2(&self, x: usize, y: usize) -> bool {
        let a = self.get(x, y);
        if y < 3 {
            return false;
        }
        let b = self.get(x + 1, y - 1);
        let c = self.get(x + 2, y - 2);
        let d = self.get(x + 3, y - 3);
        (a, b, c, d) == GOAL1 || (d, c, b, a) == GOAL1
    }

    fn check_cell(&self, x: usize, y: usize) -> u32 {
        u32::from(self.check_horizontal(x, y))
            + u32::from(self.check_vertical(x, y))
            + u32::from(self.check_diagonal_1(x, y))
            + u32::from(self.check_diagonal_2(x, y))
    }

    fn check_all(&self) -> u32 {
        let mut sum = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                sum += self.check_cell(x, y);
            }
        }
        sum
    }
}

impl WordSearch2 {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).unwrap_or(&vec![]).get(x).copied()
    }

    fn check_diagonal_1(&self, x: usize, y: usize) -> bool {
        if x < 1 || y < 1 {
            return false;
        }
        let a = self.get(x - 1, y - 1);
        let b = self.get(x, y);
        let c = self.get(x + 1, y + 1);
        (a, b, c) == GOAL2 || (c, b, a) == GOAL2
    }

    fn check_diagonal_2(&self, x: usize, y: usize) -> bool {
        if x < 1 || y < 1 {
            return false;
        }
        let a = self.get(x + 1, y - 1);
        let b = self.get(x, y);
        let c = self.get(x - 1, y + 1);
        (a, b, c) == GOAL2 || (c, b, a) == GOAL2
    }

    fn check_cell(&self, x: usize, y: usize) -> bool {
        self.check_diagonal_1(x, y) && self.check_diagonal_2(x, y)
    }

    fn check_all(&self) -> u32 {
        let mut sum = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                sum += u32::from(self.check_cell(x, y));
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let ws = WordSearch1 {
        data: input.lines().map(|l| l.chars().collect()).collect(),
    };
    Some(ws.check_all())
}

pub fn part_two(input: &str) -> Option<u32> {
    let ws = WordSearch2 {
        data: input.lines().map(|l| l.chars().collect()).collect(),
    };
    Some(ws.check_all())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
