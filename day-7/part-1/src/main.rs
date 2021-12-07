mod error;
use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

fn main() -> Result<(), Error> {
    let mut data = get_input_data()?;

    data.sort();

    let target = data.get(data.len() / 2).expect("failed to find a target");

    let total_fuel = data.iter().fold(0, |acc, n| acc + (target - *n).abs());

    println!("total fuel used : {}", total_fuel);

    Ok(())
}

fn get_input_data() -> Result<Vec<i64>, Error> {
    let file = fs::File::open("input")?;
    let file_buffer = io::BufReader::new(file);

    let string_data = match file_buffer.lines().next() {
        Some(d) => d.unwrap(),
        None => return Err(Error::BadFormat),
    };

    let data = string_data
        .split(',')
        .map(|v| v.parse::<i64>().or(Err(Error::BadFormat)))
        .collect::<Result<Vec<i64>, Error>>();

    data
}
