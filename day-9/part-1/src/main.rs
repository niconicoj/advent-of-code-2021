mod error;

use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

fn main() -> Result<(), Error> {
    let data = get_input_data()?;

    let mut sum_risk: u64 = 0;

    data.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, curr_height)| {
            let mut is_low_point = true;
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if y_offset == 0 && x_offset == 0 {
                        continue;
                    }
                    let x_val = match (x as i64).checked_add(x_offset) {
                        Some(v) => v as usize,
                        None => continue,
                    };
                    let y_val = match (y as i64).checked_add(y_offset) {
                        Some(v) => v as usize,
                        None => continue,
                    };

                    match data.get(y_val).map_or(None, |l| l.get(x_val)) {
                        Some(h) => {
                            if h < curr_height {
                                is_low_point = false;
                                break;
                            }
                        }
                        None => {}
                    };
                }
                if !is_low_point {
                    break;
                }
            }
            if is_low_point {
                sum_risk += 1 + (*curr_height as u64);
            }
        });
    });

    println!("total risk factor : {}", sum_risk);

    Ok(())
}

fn get_input_data() -> Result<Vec<Vec<u8>>, Error> {
    let file = fs::File::open("input")?;
    let file_buffer = io::BufReader::new(file);

    let data = file_buffer
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Ok(data)
}
