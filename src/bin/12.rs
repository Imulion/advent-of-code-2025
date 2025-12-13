use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Clone)]
struct Box {
    empty: Vec<usize>,
    size: usize,
    total: usize,
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    boxs: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (boxs, areas) = parse_input(input);
    for area in &areas {
        println!("{area:?}");
    }
    let result = areas
        .iter()
        .filter(|area| area.boxs.iter().sum::<usize>() * 9 <= (area.width * area.height))
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (Vec<Box>, Vec<Area>) {
    let mut boxs = Vec::new();
    let mut bx = Box {
        empty: Vec::new(),
        size: 0,
        total: 0,
    };
    for l in input.lines().skip(1).take(30) {
        if l.chars().nth(1) == Some(':') {
            continue;
        }
        if l.is_empty() {
            boxs.push(bx.clone());
            bx = Box {
                empty: Vec::new(),
                size: 0,
                total: 0,
            };
            continue;
        }
        for c in l.chars() {
            match c {
                '.' => bx.empty.push(bx.total),
                '#' => bx.size += 1,
                _ => (),
            }
            bx.total += 1;
        }
    }
    let areas = input
        .lines()
        .skip(30)
        .flat_map(|l| {
            let (size, boxs) = l.split_once(':')?;
            let (width, height) = size.split('x').flat_map(|n| n.parse()).collect_tuple()?;
            let boxs = boxs
                .split_ascii_whitespace()
                .flat_map(|n| n.parse())
                .collect();
            Some(Area {
                width,
                height,
                boxs,
            })
        })
        .collect();
    (boxs, areas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
