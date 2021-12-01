use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let mut count = 0;
    let input_data = get_input_data().unwrap();
    let summed_data: Vec<i32> = input_data
        .windows(3)
        .map(|values| values[0] + values[1] + values[2])
        .collect();

    summed_data.windows(2).for_each(|vs| {
        if vs[0] < vs[1] {
            count += 1;
        };
    });
    println!("{}", count);
}

fn get_input_data() -> std::io::Result<Vec<i32>> {
    let input_file = File::open("input")?;
    let buf_reader = io::BufReader::new(input_file);
    let input_data = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.parse::<_>().unwrap())
        .collect();

    Ok(input_data)
}
