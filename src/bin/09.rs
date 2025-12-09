use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn calculate_area(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(Debug)]
struct Edge<'a> {
    p1: &'a Point,
    p2: &'a Point,
}

impl<'a> From<(&'a Point, &'a Point)> for Edge<'a> {
    fn from((p1, p2): (&'a Point, &'a Point)) -> Self {
        Self { p1, p2 }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = parse_input(input);
    let mut largest_area = 0;
    for (i1, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i1) {
            let area = p1.calculate_area(p2);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    Some(largest_area)
}

pub fn part_two(input: &str) -> Option<usize> {
    let points = parse_input(input);
    let mut largest_area = 0;
    let mut edges: Vec<Edge> = points
        .iter()
        .tuple_windows::<(&Point, &Point)>()
        .map(Edge::from)
        .collect();
    edges.push(Edge::from((
        points.last().unwrap(),
        points.first().unwrap(),
    )));
    for (i1, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i1 + 1) {
            let area = p1.calculate_area(p2);
            if area > largest_area {
                let res = edges.iter().all(|edge| {
                    let before = p1.x.max(p2.x) <= edge.p1.x.min(edge.p2.x);
                    let after = p1.x.min(p2.x) >= edge.p1.x.max(edge.p2.x);
                    let above = p1.y.max(p2.y) <= edge.p1.y.min(edge.p2.y);
                    let below = p1.y.min(p2.y) >= edge.p1.y.max(edge.p2.y);
                    before || after || above || below
                });
                if res {
                    largest_area = area;
                }
            }
        }
    }
    Some(largest_area)
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|l| {
            l.split(',')
                .flat_map(|n| n.parse())
                .collect_tuple()
                .map(|(x, y)| Point { x, y })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
