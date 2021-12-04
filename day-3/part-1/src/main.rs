use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

mod error;

const W_LENGTH: usize = 12;
const BIT_MASK: u32 = 0x00000fff;

fn main() -> Result<(), Error> {
    let input_data = parse_input()?;

    let bits = input_data
        .iter()
        .map(|a| (a.gt(&0) as u32))
        .collect::<Vec<u32>>();
    let gamma = convert(&bits);

    let epsilon = !gamma & BIT_MASK;

    println!("gamma : {}", gamma);
    println!("epsilon : {}", epsilon);
    println!("result : {}", gamma * epsilon);

    Ok(())
}

fn parse_input() -> Result<Vec<i32>, Error> {
    let input_file = fs::File::open("input")?;
    let buf_reader = io::BufReader::new(input_file);

    let mut result: Vec<i32> = vec![0; W_LENGTH];

    buf_reader.lines().map(|l| l.unwrap()).try_for_each(|l| {
        l.chars().enumerate().try_for_each(|(i, c)| {
            match c {
                '0' => result[i] -= 1,
                '1' => result[i] += 1,
                _ => return Err(Error::BadFormat),
            };
            Ok(())
        })
    })?;

    Ok(result)
}

fn convert(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}
