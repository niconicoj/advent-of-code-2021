mod error;

use std::{
    fs::File,
    io::{self, BufRead},
};

use error::Error;

enum Move {
    Up(i32),
    Down(i32),
    Forward(i32),
}
fn main() -> Result<(), Error> {
    let mut pos = (0, 0);
    let input_data = get_input_data()?;

    input_data.iter().for_each(|m| match m {
        Move::Forward(a) => pos.0 += a,
        Move::Up(a) => pos.1 -= a,
        Move::Down(a) => pos.1 += a,
    });

    println!("x :{} y : {}", pos.0, pos.1);
    println!("mult : {}", pos.0 * pos.1);
    Ok(())
}

fn get_input_data() -> Result<Vec<Move>, Error> {
    let input_file = File::open("input")?;
    let buf_reader = io::BufReader::new(input_file);
    let input_data = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let mut str_parts = s.split_whitespace();
            let move_str = str_parts.next().unwrap();
            let amount = str_parts.next().unwrap().parse::<i32>()?;
            return match move_str {
                "forward" => Ok(Move::Forward(amount)),
                "up" => Ok(Move::Up(amount)),
                "down" => Ok(Move::Down(amount)),
                _ => Err(Error::BadFormat),
            };
        })
        .collect::<Result<Vec<Move>, Error>>()?;

    Ok(input_data)
}
