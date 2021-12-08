mod error;

use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

use error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Digit(u8);

impl Digit {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_segments(segments: &str) -> Self {
        let value = segments
            .as_bytes()
            .iter()
            .fold(0_u8, |acc, b| acc + (1 << (b - b'a') as usize));
        Self(value)
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn common_segments(&self, other: u8) -> u32 {
        println!(
            "searching for common segment between {} and {}",
            self.0, other
        );
        (self.0 & other).count_ones()
    }
}

fn deduct_segments(digits: &Vec<Digit>) -> HashMap<u8, u8> {
    println!("{:?}", digits);
    let mut digit_map: HashMap<u8, u8> = HashMap::new();

    let mut key_index = (0, 0);

    digits.iter().for_each(|d| {
        match d.count() {
            2 => {
                digit_map.insert(d.0, 1);
                println!("marking index 1 : {}", d.0);
                key_index.0 = d.0;
            }

            3 => {
                digit_map.insert(d.0, 7);
            }
            4 => {
                digit_map.insert(d.0, 4);
                println!("marking index 4 : {}", d.0);
                key_index.1 = d.0;
            }
            7 => {
                digit_map.insert(d.0, 8);
            }
            _ => {}
        };
    });

    assert!(digit_map.len() == 4);

    digits.iter().for_each(|d| {
        match d.count() {
            2 => {
                digit_map.insert(d.0, 1);
            }
            3 => {
                digit_map.insert(d.0, 7);
            }
            4 => {
                digit_map.insert(d.0, 4);
            }
            7 => {
                digit_map.insert(d.0, 8);
            }
            5 => {
                if d.common_segments(key_index.0) == 2 {
                    digit_map.insert(d.0, 3);
                } else if d.common_segments(key_index.1) == 3 {
                    digit_map.insert(d.0, 5);
                } else {
                    digit_map.insert(d.0, 2);
                }
            }
            6 => {
                if d.common_segments(key_index.0) == 1 {
                    digit_map.insert(d.0, 6);
                } else if d.common_segments(key_index.1) == 3 {
                    digit_map.insert(d.0, 0);
                } else {
                    digit_map.insert(d.0, 9);
                }
            }
            _ => {}
        };
    });

    assert!(digit_map.len() == 10);

    println!("digit map : {:?}", digit_map);
    digit_map
}

fn compute_output(output_digit: &Vec<Digit>, value_map: &HashMap<u8, u8>) -> u64 {
    output_digit
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| {
            let real_value = value_map.get(&d.0).unwrap();
            acc + (*real_value as u64) * (10_u64.pow(i as u32))
        })
}

fn main() -> Result<(), Error> {
    let data = get_input_data()?;

    let sum_output = data.iter().fold(0, |acc, tuple| {
        let value_map = deduct_segments(&tuple.0);
        let output = compute_output(&tuple.1, &value_map);
        acc + output
    });

    println!("output : {}", sum_output);

    Ok(())
}

fn get_input_data() -> Result<Vec<(Vec<Digit>, Vec<Digit>)>, Error> {
    let file = fs::File::open("input")?;
    let file_buffer = io::BufReader::new(file);

    let data = file_buffer
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let parts: Vec<String> = line
                .split(" | ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let input = parts
                .get(0)
                .unwrap()
                .split(' ')
                .map(|s| Digit::from_segments(s))
                .collect::<Vec<Digit>>();
            let output = parts
                .get(1)
                .unwrap()
                .split(' ')
                .map(|s| Digit::from_segments(s))
                .collect::<Vec<Digit>>();

            return (input, output);
        })
        .collect::<Vec<(Vec<Digit>, Vec<Digit>)>>();

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_from_segments() {
        let segments = "a";
        let digit = Digit::from_segments(segments);
        assert_eq!(digit.0, 0b00000001);
        let segments = "b";
        let digit = Digit::from_segments(segments);
        assert_eq!(digit.0, 0b00000010);
        let segments = "cd";
        let digit = Digit::from_segments(segments);
        assert_eq!(digit.0, 0b00001100);
        let segments = "gabfed";
        let digit = Digit::from_segments(segments);
        assert_eq!(digit.0, 0b01111011);
    }

    #[test]
    fn test_compute_output() {
        let mut value_map: HashMap<u8, u8> = HashMap::new();
        value_map.insert(3, 0);
        value_map.insert(0, 1);
        value_map.insert(4, 2);
        value_map.insert(9, 3);
        value_map.insert(8, 4);
        value_map.insert(6, 5);
        value_map.insert(5, 6);
        value_map.insert(1, 7);
        value_map.insert(2, 8);
        value_map.insert(7, 9);

        let digits = vec![Digit(9), Digit(4), Digit(2), Digit(0)];

        let result = compute_output(&digits, &value_map);

        assert_eq!(result, 3281);
    }
}
