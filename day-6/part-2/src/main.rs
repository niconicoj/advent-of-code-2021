mod error;

use std::{
    fs,
    io::{self, BufRead},
};

use error::Error;

struct FishSchooling {
    fishes: [u64; 9],
}

impl FishSchooling {
    pub fn empty() -> Self {
        Self { fishes: [0; 9] }
    }

    pub fn new(fishes: [u64; 9]) -> Self {
        Self { fishes }
    }

    pub fn tick(&mut self) {
        self.fishes.rotate_left(1);
        self.fishes[6] += self.fishes[8];
    }

    pub fn sum(&self) -> u64 {
        self.fishes
            .into_iter()
            .reduce(|acc, c| acc + c)
            .unwrap_or(0)
    }

    pub fn add_fish(&mut self, timer: u8) {
        if let Some(t) = self.fishes.get_mut(timer as usize) {
            *t += 1;
        }
    }
}

fn main() -> Result<(), Error> {
    let input_data = get_input_data()?;
    let mut fish_schooling = FishSchooling::empty();

    input_data.iter().for_each(|v| {
        fish_schooling.add_fish(*v);
    });

    (0..256).for_each(|d| {
        println!("computing day {}...", d);
        fish_schooling.tick();
    });

    println!("fish count after 256 days : {}", fish_schooling.sum());

    Ok(())
}

fn get_input_data() -> Result<Vec<u8>, Error> {
    let file = fs::File::open("input")?;
    let file_buffer = io::BufReader::new(file);

    let string_data = match file_buffer.lines().next() {
        Some(d) => d.unwrap(),
        None => return Err(Error::BadFormat),
    };

    let data = string_data
        .split(',')
        .map(|v| v.parse::<u8>().or(Err(Error::BadFormat)))
        .collect::<Result<Vec<u8>, Error>>();

    data
}

#[cfg(test)]
mod tests {
    use crate::FishSchooling;

    #[test]
    fn fish_schooling_count() {
        let fish_schooling = FishSchooling::new([0, 1, 0, 2, 1, 0, 0, 0, 0]);
        assert_eq!(fish_schooling.sum(), 4);
    }

    #[test]
    fn fish_schooling_tick() {
        let mut fish_schooling = FishSchooling::new([0, 1, 0, 2, 1, 0, 0, 0, 0]);

        fish_schooling.tick();
        assert_eq!(fish_schooling.fishes, [1, 0, 2, 1, 0, 0, 0, 0, 0]);

        fish_schooling.tick();
        assert_eq!(fish_schooling.fishes, [0, 2, 1, 0, 0, 0, 1, 0, 1]);

        fish_schooling.tick();
        assert_eq!(fish_schooling.fishes, [2, 1, 0, 0, 0, 1, 0, 1, 0]);

        fish_schooling.tick();
        assert_eq!(fish_schooling.fishes, [1, 0, 0, 0, 1, 0, 3, 0, 2]);
    }
}
