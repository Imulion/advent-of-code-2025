advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines().fold(0, |mut val, l| {
        let (n1_index, n1) = l[..l.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .max_by(|(_i1, n1), (_i2, n2)| n1.cmp(n2))
            .unwrap();
        let n1_index = l.len() - (n1_index + 2);
        let (_n2_index, n2) = l[n1_index + 1..l.len()]
            .chars()
            .rev()
            .enumerate()
            .max_by(|(_i1, n1), (_i2, n2)| n1.cmp(n2))
            .unwrap();
        let mut num = n1.to_string();
        num.push(n2);
        val += num.parse::<u64>().unwrap();
        val
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines().fold(0, |mut val, l| {
        let mut nums: Vec<u8> = Vec::new();
        let mut start_index = 0;
        for i in (0..12).rev() {
            let (index, n) = l[start_index..l.len() - i]
                .chars()
                .rev()
                .enumerate()
                .max_by(|(_i1, n1), (_i2, n2)| n1.cmp(n2))
                .unwrap();
            nums.push(n as u8);
            start_index = l.len() - (index + i);
        }

        let num = String::from_utf8(nums).unwrap();
        val += num.parse::<u64>().unwrap();
        val
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
