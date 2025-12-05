use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};

advent_of_code::solution!(4);

enum GridItem {
    PaperRoll,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

const CHECK_MATRIX: [Position; 8] = [
    Position::new(-1, -1),
    Position::new(-1, 0),
    Position::new(-1, 1),
    Position::new(1, -1),
    Position::new(1, 0),
    Position::new(1, 1),
    Position::new(0, -1),
    Position::new(0, 1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut result = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            if let GridItem::PaperRoll = symbol {
                let current_pos = Position::from((x, y));
                let mut count = 0;
                for offset in CHECK_MATRIX {
                    let pos = current_pos + offset;
                    if let Some(GridItem::PaperRoll) = get_from_grid(&grid, &pos) {
                        count += 1
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    let mut result = 0;
    loop {
        let positions = find_loose_paper(&grid);
        let len = positions.len();
        if positions.is_empty() {
            break;
        }
        for pos in positions {
            set_blank(&mut grid, &pos);
        }
        result += len;
    }
    Some(result)
}

fn find_loose_paper(grid: &[Vec<GridItem>]) -> HashSet<Position> {
    let mut positions = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            if let GridItem::PaperRoll = symbol {
                let current_pos = Position::from((x, y));
                let mut count = 0;
                for offset in CHECK_MATRIX {
                    let pos = current_pos + offset;
                    if let Some(GridItem::PaperRoll) = get_from_grid(grid, &pos) {
                        count += 1
                    }
                }
                if count < 4 {
                    positions.insert(current_pos);
                }
            }
        }
    }
    positions
}

fn parse_input(input: &str) -> Vec<Vec<GridItem>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => GridItem::PaperRoll,
                    _ => GridItem::Empty,
                })
                .collect()
        })
        .collect()
}

fn get_from_grid<'a>(grid: &'a [Vec<GridItem>], pos: &Position) -> Option<&'a GridItem> {
    let x = pos.x as usize;
    let y = pos.y as usize;
    grid.get(y).and_then(|row| row.get(x))
}

fn set_blank(grid: &mut [Vec<GridItem>], pos: &Position) {
    let x = pos.x as usize;
    let y = pos.y as usize;
    if let Some(row) = grid.get_mut(y) {
        row[x] = GridItem::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
