use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let mut count = 0;
    let input_data = get_input_data().unwrap();
    input_data.windows(2).for_each(|vs| {
        if vs[0] < vs[1] {
            count += 1;
        };
    });

    println!("{}", count);
}

fn get_input_data() -> std::io::Result<Vec<i16>> {
    let input_file = File::open("input")?;
    let buf_reader = io::BufReader::new(input_file);
    let input_data = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.parse::<i16>().unwrap())
        .collect();

    Ok(input_data)
}
