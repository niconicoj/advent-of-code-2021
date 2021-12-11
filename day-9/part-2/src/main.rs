mod error;
mod grid;

use std::fs;

use error::Error;
use grid::Grid;

fn main() -> Result<(), Error> {
    let data = get_input_data()?;

    let grid = Grid::new(data.0, data.1 .0, data.1 .1);

    let low_points = grid.get_low_points();

    println!("total low points : {}", low_points.len());
    println!(
        "risk factor: {}",
        low_points.values().fold(0_u64, |acc, v| acc + (*v as u64))
    );

    let mut basins = grid.find_basins_size();
    basins.sort();

    println!("total basins : {:?}", basins);

    let result = basins
        .iter()
        .rev()
        .take(3)
        .enumerate()
        .fold(1_u64, |acc, v| acc * v.1);

    println!("result : {}", result);

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
