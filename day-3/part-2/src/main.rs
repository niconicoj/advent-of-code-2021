use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

mod error;

const W_LENGTH: usize = 12;
// const BIT_MASK: u32 = 0xfff;

fn main() -> Result<(), Error> {
    let mut oxy_list = get_lines_as_u32()?;
    filter_values(&mut oxy_list, false)?;

    match oxy_list.get(0) {
        Some(v) => println!("oxy : {}", v),
        None => println!("could not find oxy value"),
    };

    let mut o2_list = get_lines_as_u32()?;
    filter_values(&mut o2_list, true)?;

    match o2_list.get(0) {
        Some(v) => println!("o2 : {}", v),
        None => println!("could not find o2 value"),
    };

    Ok(())
}

fn get_common_word(values: &Vec<u32>, epsilon: bool) -> Result<u32, Error> {
    let mut result: Vec<i32> = vec![0; W_LENGTH];
    values.iter().try_for_each(|l| {
        format!("{:0width$b}", l, width = W_LENGTH)
            .chars()
            .enumerate()
            .try_for_each(|(i, c)| {
                match c {
                    '0' => result[i] -= 1,
                    '1' => result[i] += 1,
                    _ => return Err(Error::BadFormat),
                };
                Ok(())
            })
    })?;

    match epsilon {
        false => {
            let value = result
                .iter()
                .map(|a| (a.ge(&0) as u32))
                .collect::<Vec<u32>>();
            Ok(convert(&value))
        }
        true => {
            let value = result
                .iter()
                .map(|a| (a.lt(&0) as u32))
                .collect::<Vec<u32>>();
            Ok(convert(&value))
        }
    }
}

fn filter_values(values: &mut Vec<u32>, epsilon: bool) -> Result<(), Error> {
    let mut ply = 0;
    while values.len() > 1 {
        let common_word = get_common_word(&values, epsilon)?;
        let ply_common_bit = common_word >> (W_LENGTH - 1 - ply) & 1;
        values.retain(|v| {
            let current_bit = v >> (W_LENGTH - 1 - ply) & 1;
            current_bit == ply_common_bit
        });
        ply += 1;
    }

    Ok(())
}

fn get_lines_as_u32() -> Result<Vec<u32>, Error> {
    let input_file = fs::File::open("input")?;
    let buf_reader = io::BufReader::new(input_file);
    let result = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| u32::from_str_radix(&l, 2).or(Err(Error::BadFormat)))
        .collect::<Result<Vec<u32>, Error>>()?;

    Ok(result)
}

fn convert(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}
