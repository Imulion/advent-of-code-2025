use good_lp::{Expression, Solution, SolverModel, microlp, variable, variables};

use itertools::Itertools;

advent_of_code::solution!(10);

type Machine = (Vec<bool>, Vec<Vec<usize>>, Vec<usize>);

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse_input(input);
    let result = machines.iter().map(find_best_lights_result).sum();
    Some(result)
}

fn find_best_lights_result(machine: &Machine) -> usize {
    let goal = &machine.0;
    let mut min_take = 1;
    loop {
        let result = machine.1.iter().combinations(min_take).any(|button_group| {
            let end = button_group
                .iter()
                .fold(vec![false; goal.len()], |mut test, group| {
                    for x in group.iter() {
                        test[*x] = !test[*x];
                    }
                    test
                });
            &end == goal
        });
        if result {
            break;
        } else {
            min_take += 1;
        }
    }
    min_take
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = parse_input(input);
    let result = machines.iter().map(find_best_voltage_result).sum();
    Some(result)
}
fn find_best_voltage_result(machine: &Machine) -> usize {
    let targets = machine.2.clone();

    let groups: Vec<Vec<usize>> = machine.1.clone();

    let n = targets.len();
    let m = groups.len();

    // coverage matrix
    let mut a = vec![vec![0_f64; m]; n];
    for (j, group) in groups.iter().enumerate() {
        for &idx in group {
            a[idx][j] = 1.0;
        }
    }

    let mut vars = variables!();

    // Integer decision variables x_j >= 0
    let mut x = Vec::new();
    for j in 0..m {
        x.push(
            vars.add(
                variable()
                    .min(0)
                    .max(*targets.iter().max().unwrap() as f64)
                    .integer()
                    .name(format!("x_{}", j)),
            ),
        );
    }

    let mut problem = vars
        .minimise(x.iter().fold(0.into(), |acc: Expression, var| acc + var))
        .using(microlp);

    // Constraints: sum_j A[i][j] * x_j >= T[i]
    for i in 0..n {
        let mut expr: Expression = 0.into();
        for j in 0..m {
            expr += a[i][j] * x[j];
        }
        problem = problem.with((expr - targets[i] as i32).eq(0));
    }

    let solution = problem.solve().unwrap();

    let mut total = 0;
    let mut control = vec![0.0; targets.len()];
    for (i, val) in x.iter().enumerate() {
        let v = solution.value(*val);
        for val in &groups[i] {
            control[*val] += 1.0 * v;
        }
        total += v.round() as usize;
    }
    // println!("Control: {control:?}");
    // let new_control: Vec<usize> = control.iter().map(|x| x.round() as usize).collect();
    // println!("Target: {targets:?}");
    // println!("Usize Control: {new_control:?}");
    // assert!(targets == new_control);
    // let total = total as usize;
    // println!("Total {total}");
    // println!("New Total {new_total}");
    // assert!(total == new_total);
    total
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|l| {
            let splits: Vec<&str> = l.split_whitespace().collect();
            let lights = splits
                .first()
                .map(|arr| {
                    arr.chars()
                        .skip(1)
                        .take(arr.len() - 2)
                        .map(|c| c == '#')
                        .collect()
                })
                .unwrap();
            let voltages = splits
                .last()
                .map(|arr| {
                    arr[1..arr.len() - 1]
                        .split(',')
                        .flat_map(|num| num.parse())
                        .collect()
                })
                .unwrap();
            let buttons = splits
                .iter()
                .skip(1)
                .take(splits.len() - 2)
                .map(|group| {
                    group[1..group.len() - 1]
                        .split(',')
                        .flat_map(|num| num.parse())
                        .collect()
                })
                .collect();
            (lights, buttons, voltages)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
