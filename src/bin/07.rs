use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut beams: HashSet<usize> = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    let start = lines
        .first()
        .expect("Did not find First Row")
        .chars()
        .enumerate()
        .find_map(|(i, c)| match c {
            'S' => Some(i),
            _ => None,
        })
        .expect("No S in First Row");
    beams.insert(start);
    let mut result = 0;
    for l in lines {
        let chars: Vec<char> = l.chars().collect();
        let mut new_beams = Vec::new();
        let mut remove_beams = Vec::new();
        for beam in beams.clone() {
            if chars.get(beam) == Some(&'^') {
                new_beams.push(beam - 1);
                new_beams.push(beam + 1);
                remove_beams.push(beam);
                result += 1;
            }
        }
        for beam in remove_beams {
            beams.remove(&beam);
        }
        for beam in new_beams {
            beams.insert(beam);
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let start = lines
        .first()
        .expect("Did not find First Row")
        .chars()
        .enumerate()
        .find_map(|(i, c)| match c {
            'S' => Some(i),
            _ => None,
        })
        .expect("No S in First Row");
    beams.insert(start, 1);
    for l in lines {
        let chars: Vec<char> = l.chars().collect();
        let mut new_beams = Vec::new();
        let mut remove_beams = Vec::new();
        for (beam, count) in beams.clone() {
            if chars.get(beam) == Some(&'^') {
                new_beams.push((beam - 1, count));
                new_beams.push((beam + 1, count));
                remove_beams.push(beam);
            }
        }
        for beam in remove_beams.iter().rev() {
            beams.remove(beam);
        }
        for (beam, count) in new_beams {
            if let Some(val) = beams.get_mut(&beam) {
                *val += count;
            } else {
                beams.insert(beam, count);
            }
        }
    }
    let result = beams.values().sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
