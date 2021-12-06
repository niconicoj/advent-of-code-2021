mod error;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use error::Error;

type Point = (i64, i64);

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal(RangeInclusive<i64>),
    Vertical(RangeInclusive<i64>),
    Diagonal(RangeInclusive<i64>, RangeInclusive<i64>),
    Other,
}

#[derive(Debug)]
struct Vector {
    start: Point,
    end: Point,
}

impl Vector {
    pub fn new(start: Point, end: Point) -> Self {
        return Self { start, end };
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = vec![];

        match self.get_direction() {
            Direction::Horizontal(x) => {
                x.for_each(|x| points.push((x, self.start.1)));
            }
            Direction::Vertical(y) => {
                y.for_each(|y| points.push((self.start.0, y)));
            }
            Direction::Diagonal(x, y) => {
                if (self.start.0 > self.end.0 && self.start.1 <= self.end.1)
                    || (self.start.0 <= self.end.0 && self.start.1 > self.end.1)
                {
                    x.zip(y.rev()).for_each(|(x, y)| points.push((x, y)));
                } else {
                    x.zip(y).for_each(|(x, y)| points.push((x, y)));
                }
            }
            _ => {}
        }

        points
    }

    fn get_direction(&self) -> Direction {
        let axis = self.get_axis_range();
        if self.start.1 == self.end.1 {
            Direction::Horizontal(axis.0)
        } else if self.start.0 == self.end.0 {
            Direction::Vertical(axis.1)
        } else if (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs() {
            Direction::Diagonal(axis.0, axis.1)
        } else {
            Direction::Other
        }
    }

    fn get_axis_range(&self) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
        match ((self.start.0 <= self.end.0), (self.start.1 <= self.end.1)) {
            (true, true) => ((self.start.0..=self.end.0), (self.start.1..=self.end.1)),
            (false, true) => ((self.end.0..=self.start.0), (self.start.1..=self.end.1)),
            (true, false) => ((self.start.0..=self.end.0), (self.end.1..=self.start.1)),
            (false, false) => ((self.end.0..=self.start.0), (self.end.1..=self.start.1)),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut input_data = parse_input()?;

    input_data.retain(|i| match i.get_direction() {
        Direction::Other => false,
        _ => true,
    });

    let mut intersection_map: HashMap<Point, i64> = HashMap::new();

    input_data.iter().for_each(|v| {
        v.get_points().iter().for_each(|p| {
            if let Some(c) = intersection_map.get_mut(&p) {
                *c += 1;
            } else {
                intersection_map.insert(*p, 1);
            }
        });
    });

    let sum: i64 = intersection_map.values().fold(0, |acc, v| {
        if v > &1 {
            return acc + 1;
        }
        return acc;
    });

    println!("total overlap points : {}", sum);

    Ok(())
}

fn parse_input() -> Result<Vec<Vector>, Error> {
    let input_file = File::open("input")?;
    let input_buf = BufReader::new(input_file);

    let parsed_data = input_buf
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(|s| {
            let parts = s.split(" -> ").collect::<Vec<&str>>();
            let start = parts
                .get(0)
                .unwrap()
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let end = parts
                .get(1)
                .unwrap()
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            return Vector::new(
                (*start.get(0).unwrap(), *start.get(1).unwrap()),
                (*end.get(0).unwrap(), *end.get(1).unwrap()),
            );
        })
        .collect();

    return Ok(parsed_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_get_points() {
        // vertical
        let vector = Vector::new((1, 4), (1, 6));
        assert_eq!(vector.get_points(), [(1, 4), (1, 5), (1, 6)]);

        // horizontal
        let vector = Vector::new((3, 1), (5, 1));
        assert_eq!(vector.get_points(), [(3, 1), (4, 1), (5, 1)]);

        // diagonal going in all for possible directions
        let vector = Vector::new((3, 3), (5, 1));
        assert_eq!(vector.get_points(), [(3, 3), (4, 2), (5, 1)]);
        let vector = Vector::new((3, 3), (5, 5));
        assert_eq!(vector.get_points(), [(3, 3), (4, 4), (5, 5)]);
        let vector = Vector::new((3, 3), (1, 1));
        assert_eq!(vector.get_points(), [(1, 1), (2, 2), (3, 3)]);
        let vector = Vector::new((3, 3), (1, 5));
        assert_eq!(vector.get_points(), [(1, 5), (2, 4), (3, 3)]);
    }

    #[test]
    fn vector_get_direction() {
        let vector = Vector::new((34, 25), (12, 25));
        assert_eq!(vector.get_direction(), Direction::Horizontal(12..=34));
        let vector = Vector::new((34, 25), (34, 1));
        assert_eq!(vector.get_direction(), Direction::Vertical(1..=25));
        let vector = Vector::new((34, 25), (50, 9));
        assert_eq!(vector.get_direction(), Direction::Diagonal(34..=50, 9..=25));
        let vector = Vector::new((34, 25), (12, 76));
        assert_eq!(vector.get_direction(), Direction::Other);
    }

    #[test]
    fn vector_intersect() {
        let vector_1 = Vector::new((0, 0), (0, 5));
        assert_eq!(vector_1.get_axis_range(), ((0..=0), (0..=5)));

        let vector_1 = Vector::new((1, 1), (5, 1));
        assert_eq!(vector_1.get_axis_range(), ((1..=5), (1..=1)));

        let vector_1 = Vector::new((5, 1), (1, 1));
        assert_eq!(vector_1.get_axis_range(), ((1..=5), (1..=1)));

        let vector_1 = Vector::new((5, 5), (1, 1));
        assert_eq!(vector_1.get_axis_range(), ((1..=5), (1..=5)));
    }
}
