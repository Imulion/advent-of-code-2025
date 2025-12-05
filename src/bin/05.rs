use std::collections::HashSet;

advent_of_code::solution!(5);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn is_included_by(&self, other: &Range) -> bool {
        other.lower < self.lower && other.upper > self.upper
    }

    fn is_upper_extended_by(&self, other: &Range) -> bool {
        other.upper > self.upper && other.lower >= self.lower && other.lower <= self.upper
    }

    fn is_lower_extended_by(&self, other: &Range) -> bool {
        other.lower < self.lower && other.upper >= self.lower && other.upper <= self.upper
    }

    fn is_redudant_by(&self, other: &Range) -> bool {
        other.lower <= self.lower && other.upper >= self.upper
    }
}

#[derive(Debug)]
enum RangeFindResult {
    LowerExtend(usize),
    UpperExtend(usize),
    Includes(usize),
    Redundant,
}

type Item = usize;

pub fn part_one(input: &str) -> Option<usize> {
    let (items, ranges) = parse_input(input);
    let result = items
        .iter()
        .filter(|item| {
            ranges
                .iter()
                .any(|r| r.lower <= **item && r.upper >= **item)
        })
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, mut ranges) = parse_input(input);
    let mut new_ranges: Vec<Range> = Vec::new();
    while let Some(range) = ranges.pop() {
        let result = find_range_result(&range, &new_ranges);
        match result {
            Some(RangeFindResult::Includes(index)) => {
                let other = new_ranges.get_mut(index).unwrap();
                other.lower = range.lower;
                other.upper = range.upper;
                ranges.insert(0, other.clone());
            }
            Some(RangeFindResult::UpperExtend(index)) => {
                let other = new_ranges.get_mut(index).unwrap();
                other.upper = range.upper;
                ranges.insert(0, other.clone());
            }
            Some(RangeFindResult::LowerExtend(index)) => {
                let other = new_ranges.get_mut(index).unwrap();
                other.lower = range.lower;
                ranges.insert(0, other.clone());
            }
            Some(RangeFindResult::Redundant) => (),
            None => {
                new_ranges.push(range.clone());
            }
        }
    }
    let mut final_ranges = HashSet::new();
    for range in new_ranges {
        final_ranges.insert(range);
    }
    let result = final_ranges.iter().fold(0, |mut val, range| {
        val += (range.upper + 1) - range.lower;
        val
    });
    Some(result)
}

fn find_range_result(range: &Range, ranges: &[Range]) -> Option<RangeFindResult> {
    let mut current_state = None;
    for (index, other) in ranges.iter().enumerate() {
        if other.is_included_by(range) {
            return Some(RangeFindResult::Includes(index));
        }
        if other.is_lower_extended_by(range) {
            return Some(RangeFindResult::LowerExtend(index));
        }
        if other.is_upper_extended_by(range) {
            return Some(RangeFindResult::UpperExtend(index));
        }
        if range.is_redudant_by(other) {
            current_state = Some(RangeFindResult::Redundant);
        }
    }
    current_state
}

fn parse_input(input: &str) -> (Vec<Item>, Vec<Range>) {
    let (ranges, items) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|l| {
            let (lower, upper) = l.split_once('-').unwrap();
            Range {
                lower: lower.parse().unwrap(),
                upper: upper.parse().unwrap(),
            }
        })
        .collect();
    let items = items.lines().flat_map(|l| l.parse()).collect();
    (items, ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(24));
    }
}
