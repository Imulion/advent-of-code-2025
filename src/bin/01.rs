advent_of_code::solution!(1);

struct Rotation {
    value: isize,
    direction: RotationDirection,
}
enum RotationDirection {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<isize> {
    let rotations: Vec<Rotation> = parse_input(input);
    let mut count = 0;
    rotations.iter().fold(50, |val, rotation| {
        let val = match rotation.direction {
            RotationDirection::Right => val + rotation.value,
            RotationDirection::Left => val - rotation.value,
        };
        let val = val.rem_euclid(100);
        if val == 0 {
            count += 1;
        }
        val
    });
    Some(count)
}

pub fn part_two(input: &str) -> Option<isize> {
    let rotations: Vec<Rotation> = parse_input(input);
    let mut count = 0;
    rotations.iter().fold(50, |val, rotation| {
        count += rotation.value / 100;
        let new_val = match rotation.direction {
            RotationDirection::Right => val + rotation.value,
            RotationDirection::Left => val - rotation.value,
        };
        let new_val = new_val.rem_euclid(100);
        if match rotation.direction {
            RotationDirection::Right => new_val < val,
            RotationDirection::Left => new_val > val,
        } && new_val != 0
            && val != 0
        {
            count += 1;
        }
        if new_val == 0 {
            count += 1;
        }
        new_val
    });
    Some(count)
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|l| {
            let direction = match l.chars().next() {
                Some('R') => RotationDirection::Right,
                Some('L') => RotationDirection::Left,
                _ => panic!("No First Char found!"),
            };
            let value: isize = l[1..].parse().unwrap();
            Rotation { value, direction }
        })
        .collect()
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
