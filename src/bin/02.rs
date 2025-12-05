use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(2);

struct Range {
    lower: u64,
    upper: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let result = ranges.iter().fold(0, |mut val, range| {
        let mut current_val = range.lower;
        while current_val <= range.upper {
            let val_string = current_val.to_string();
            if val_string.len() % 2 == 0 {
                let splits = val_string.split_at(val_string.len() / 2);
                if splits.0 == splits.1 {
                    val += current_val;
                }
            }
            current_val += 1;
        }
        val
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let result = ranges.iter().fold(0, |mut val, range| {
        let mut current_val = range.lower;
        while current_val <= range.upper {
            let val_string = current_val.to_string();
            let mut invalid_ids = HashSet::new();
            for i in 1..=val_string.len() / 2 {
                let splits = val_string
                    .chars()
                    .chunks(i)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>())
                    .collect::<Vec<String>>();
                let mut splits = splits.iter();
                let mut prev = splits.next().unwrap();
                let mut result = true;
                for next in splits {
                    if prev != next {
                        result = false;
                        break;
                    }
                    prev = next;
                }
                if result {
                    invalid_ids.insert(current_val);
                }
            }
            val += invalid_ids.iter().sum::<u64>();
            current_val += 1;
        }
        val
    });
    Some(result)
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|line| {
            let splits = line.split_once('-').unwrap();
            Range {
                lower: splits.0.parse().unwrap(),
                upper: splits.1.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
