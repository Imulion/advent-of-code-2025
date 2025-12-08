use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct LastConnected {
    from: usize,
    to: usize,
    distance: f64,
}

impl LastConnected {
    fn default() -> Self {
        LastConnected {
            from: 0,
            to: 0,
            distance: 0.0,
        }
    }
}

impl Box {
    fn distance_to(&self, other: &Self) -> f64 {
        let d_x = (self.x - other.x).pow(2) as f64;
        let d_y = (self.y - other.y).pow(2) as f64;
        let d_z = (self.z - other.z).pow(2) as f64;
        (d_x + d_y + d_z).sqrt()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_amount(input, 1000)
}

pub fn part_one_amount(input: &str, amount: usize) -> Option<usize> {
    let boxs = parse_input(input);
    let mut top_distances = Vec::new();
    for (i1, b1) in boxs.iter().enumerate() {
        for (i2, b2) in boxs.iter().enumerate().skip(i1 + 1) {
            let distance = b1.distance_to(b2);
            top_distances.push((distance, (i1, i2)));
        }
    }
    top_distances.sort_by(|d1, d2| d1.0.total_cmp(&d2.0));
    let mut circuit_assign: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for (_, (i1, i2)) in top_distances.into_iter().take(amount) {
        let b1 = circuit_assign.get(&i1).cloned();
        let b2 = circuit_assign.get(&i2).cloned();
        match (b1, b2) {
            (None, None) => {
                let circuit = vec![i1, i2];
                circuits.push(circuit);
                circuit_assign.insert(i1, circuits.len() - 1);
                circuit_assign.insert(i2, circuits.len() - 1);
            }
            (Some(index), None) => {
                let circuit = circuits.get_mut(index).unwrap();
                circuit.push(i2);
                circuit_assign.insert(i2, index);
            }
            (None, Some(index)) => {
                let circuit = circuits.get_mut(index).unwrap();
                circuit.push(i1);
                circuit_assign.insert(i1, index);
            }
            (Some(c1), Some(c2)) if c1 == c2 => (),
            (Some(c1), Some(c2)) => {
                let circuit2 = circuits.get(c2).cloned().unwrap();
                let circuit1 = circuits.get_mut(c1).unwrap();
                for index in circuit2 {
                    if let Some(b) = circuit_assign.get_mut(&index) {
                        *b = c1;
                    }
                    circuit1.push(index);
                }
            }
        }
    }
    circuits.sort_by(|c1, c2| c2.len().cmp(&c1.iter().len()));
    let result = circuits.iter().take(3).map(|c| c.len()).product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let boxs = parse_input(input);
    let mut distance_maps: Vec<Vec<f64>> = (0..boxs.len()).map(|_| vec![0.0; boxs.len()]).collect();
    let mut smallest_distance = (f64::MAX, (0, 0));
    for (i1, b1) in boxs.iter().enumerate() {
        for (i2, b2) in boxs.iter().enumerate().skip(i1 + 1) {
            let distance = b1.distance_to(b2);
            distance_maps[i1][i2] = distance;
            distance_maps[i2][i1] = distance;
            if distance < smallest_distance.0 {
                smallest_distance = (distance, (i1, i2));
            }
        }
    }

    let mut connected = Vec::new();
    connected.push(smallest_distance.1.0);
    connected.push(smallest_distance.1.1);
    let mut missing_boxs: Vec<usize> = (0..boxs.len()).collect();
    missing_boxs.remove(smallest_distance.1.0);
    missing_boxs.remove(smallest_distance.1.0);

    let mut last_connected = LastConnected::default();
    loop {
        let min_connection = connected
            .iter()
            .flat_map(|box_index| {
                let map = distance_maps.get(*box_index).unwrap();
                let min_from_box = missing_boxs
                    .iter()
                    .flat_map(|i| map.get(*i).map(|dis| (i, dis)))
                    .enumerate()
                    .min_by(|(_, (_, d1)), (_, (_, d2))| d1.total_cmp(d2));
                min_from_box.map(|(missing_index, (to, d))| (missing_index, box_index, to, d))
            })
            .min_by(|(_, _, _, d1), (_, _, _, d2)| d1.total_cmp(d2));
        if let Some(min_connection) = min_connection {
            last_connected.from = *min_connection.1;
            last_connected.to = *min_connection.2;
            last_connected.distance = *min_connection.3;
            missing_boxs.remove(min_connection.0);
            connected.push(last_connected.to);
        }
        if connected.len() == boxs.len() {
            break;
        }
    }
    match (boxs.get(last_connected.from), boxs.get(last_connected.to)) {
        (Some(box1), Some(box2)) => Some(box1.x * box2.x),
        _ => None,
    }
}

fn parse_input(input: &str) -> Vec<Box> {
    input
        .lines()
        .flat_map(|l| {
            if let Some((x, y, z)) = l.split(',').flat_map(|n| n.parse()).collect_tuple() {
                Some(Box { x, y, z })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_amount(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
