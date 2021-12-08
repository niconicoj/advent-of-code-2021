mod error;

use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Digit {
    segments: String,
}

impl Digit {
    pub fn new(segments: String) -> Self {
        let slice = &segments[..];
        let mut chars = slice.chars().collect::<Vec<char>>();
        chars.sort();
        Self {
            segments: String::from_iter(chars),
        }
    }

    pub fn count(&self) -> usize {
        self.segments.len()
    }
}

fn main() -> Result<(), Error> {
    let data = get_input_data()?;

    let digit_count = data.iter().fold(0, |acc, tuple| {
        acc + tuple
            .1
            .iter()
            .map(|digit| digit.count())
            .filter(|count| [2, 3, 4, 7].contains(count))
            .count()
    });

    println!("digit count : {}", digit_count);

    Ok(())
}

fn get_input_data() -> Result<Vec<(Vec<Digit>, Vec<Digit>)>, Error> {
    let file = fs::File::open("input")?;
    let file_buffer = io::BufReader::new(file);

    let data = file_buffer
        .lines()
        .map(|l| {
            let parts: Vec<String> = l
                .unwrap()
                .split(" | ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let input = parts
                .get(0)
                .unwrap()
                .split(' ')
                .map(|s| Digit::new(s.to_string()))
                .collect::<Vec<Digit>>();
            let output = parts
                .get(1)
                .unwrap()
                .split(' ')
                .map(|s| Digit::new(s.to_string()))
                .collect::<Vec<Digit>>();

            return (input, output);
        })
        .collect::<Vec<(Vec<Digit>, Vec<Digit>)>>();

    Ok(data)
}
