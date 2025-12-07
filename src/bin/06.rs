advent_of_code::solution!(6);

struct Problem {
    opperation: Opperation,
    nums: Vec<usize>,
}

#[derive(Debug)]
struct CephProblem {
    opperation: Opperation,
    index: usize,
    length: usize,
    nums: Vec<usize>,
}

impl Problem {
    fn new(opperation: Opperation) -> Self {
        Problem {
            opperation,
            nums: Vec::new(),
        }
    }

    fn calc(&self) -> usize {
        match self.opperation {
            Opperation::Addition => self.nums.iter().sum(),
            Opperation::Multiplication => self.nums.iter().product(),
        }
    }
}

impl CephProblem {
    fn new(opperation: Opperation, index: usize) -> Self {
        CephProblem {
            opperation,
            index,
            length: 0,
            nums: Vec::new(),
        }
    }

    fn calc(&self) -> usize {
        match self.opperation {
            Opperation::Addition => self.nums.iter().sum(),
            Opperation::Multiplication => self.nums.iter().product(),
        }
    }
}

#[derive(Debug)]
enum Opperation {
    Multiplication,
    Addition,
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.lines();
    let mut problems = input
        .clone()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|l| {
            let opperation = match l {
                "+" => Opperation::Addition,
                "*" => Opperation::Multiplication,
                _ => panic!("How?"),
            };
            Problem::new(opperation)
        })
        .collect::<Vec<Problem>>();
    input.for_each(|l| {
        l.split_whitespace()
            .flat_map(|num| num.parse::<usize>())
            .enumerate()
            .for_each(|(index, num)| problems.get_mut(index).unwrap().nums.push(num));
    });
    let result = problems.iter().fold(0, |val, problem| val + problem.calc());
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.lines();
    let mut problems: Vec<CephProblem> = Vec::new();
    let mut current_problem: Option<CephProblem> = None;
    let chars = input.clone().last().unwrap().chars();
    let mut last_index = 0;
    for (index, c) in chars.enumerate() {
        let opperation = match c {
            '+' => Some(Opperation::Addition),
            '*' => Some(Opperation::Multiplication),
            _ => None,
        };
        if let Some(opperation) = opperation {
            if let Some(mut problem) = current_problem {
                problem.length = (index - problem.index) - 1;
                problems.push(problem);
            }
            current_problem = Some(CephProblem::new(opperation, index));
        }
        last_index = index;
    }
    if let Some(mut problem) = current_problem {
        problem.length = (last_index - problem.index) + 1;
        problems.push(problem);
    }

    let grid: Vec<Vec<char>> = input.map(|l| l.chars().collect()).collect();
    problems.iter_mut().for_each(|p| {
        for x in p.index..p.index + p.length {
            let mut num = String::from("");
            for y in 0..grid.len() - 1 {
                if let Some(c) = get_from_grid(&grid, x, y) {
                    num.push(*c);
                }
            }
            if let Ok(num) = num.trim().parse() {
                p.nums.push(num);
            }
        }
    });
    let result = problems.iter().fold(0, |val, problem| val + problem.calc());
    Some(result)
}

fn get_from_grid(grid: &[Vec<char>], x: usize, y: usize) -> Option<&char> {
    grid.get(y).and_then(|row| row.get(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
