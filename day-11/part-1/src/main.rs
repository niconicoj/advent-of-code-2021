mod error;
mod grid;

use std::fs;

use error::Error;
use grid::Grid;

fn main() -> Result<(), Error> {
    let data = get_input_data()?;

    let mut grid = Grid::new(data.0, data.1 .0, data.1 .1);

    let mut iteration = 0;

    while !grid.is_synchronized() {
        iteration += 1;
        grid.step();
    }

    println!("synchronized  at iteration {}", iteration);

    Ok(())
}

fn get_input_data() -> Result<(Vec<u8>, (i64, i64)), Error> {
    let content = fs::read_to_string("input")?;

    let width = content.lines().next().unwrap().chars().count() as i64;
    let height = content.lines().count() as i64;

    let data = content
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    Ok((data, (width, height)))
}
