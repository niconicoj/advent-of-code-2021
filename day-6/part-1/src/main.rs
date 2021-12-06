mod error;

use rayon::prelude::*;

use std::{
    fs,
    io::{self, BufRead},
    sync::{Arc, Mutex},
};

use error::Error;
use rayon::slice::ParallelSlice;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fish(u8);

impl Fish {
    pub fn new(timer: u8) -> Self {
        Self(timer)
    }

    pub fn tick(&mut self) -> bool {
        match self.0.checked_sub(1) {
            Some(r) => {
                self.0 = r;
                return false;
            }
            None => {
                self.0 = 6;
                return true;
            }
        }
    }
}

struct FishSchooling {
    fishes: Vec<Fish>,
}

impl FishSchooling {
    pub fn new(fishes: Vec<Fish>) -> Self {
        Self { fishes }
    }

    pub fn tick(&mut self) {
        let count = Arc::new(Mutex::new(0));
        self.fishes.par_chunks_mut(2048).for_each(|chunk| {
            let mut local_count = 0;
            chunk.iter_mut().for_each(|f| {
                if f.tick() {
                    local_count += 1;
                }
            });

            let mut global_count = count.lock().unwrap();
            *global_count += local_count;
        });

        let mut new_fishes = vec![Fish::new(8); *count.lock().unwrap()];

        self.fishes.append(&mut new_fishes);
    }

    pub fn count(&self) -> usize {
        self.fishes.len()
    }
}

fn main() -> Result<(), Error> {
    let input_data = get_input_data()?;

    let fishes = input_data.iter().map(|v| Fish::new(*v)).collect();

    let mut fish_schooling = FishSchooling::new(fishes);

    (0..80).for_each(|d| {
        println!("computing day {}...", d + 1);
        fish_schooling.tick();
    });

    println!("fish count after 80 days : {}", fish_schooling.count());

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
    use crate::{Fish, FishSchooling};

    #[test]
    fn fish_tick() {
        let mut fish = Fish::new(2);
        assert!(!fish.tick());
        assert_eq!(fish.0, 1);
        assert!(!fish.tick());
        assert_eq!(fish.0, 0);
        assert!(fish.tick());
        assert_eq!(fish.0, 6);
    }

    #[test]
    fn fish_schooling_count() {
        let fish_schooling = FishSchooling::new(vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        assert_eq!(fish_schooling.count(), 5);
    }

    #[test]
    fn fish_schooling_tick() {
        let mut fish_schooling = FishSchooling::new(vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);

        fish_schooling.tick();
        assert_eq!(
            fish_schooling.fishes,
            [Fish(2), Fish(3), Fish(2), Fish(0), Fish(1)]
        );

        fish_schooling.tick();
        assert_eq!(
            fish_schooling.fishes,
            [Fish(1), Fish(2), Fish(1), Fish(6), Fish(0), Fish(8)]
        );

        fish_schooling.tick();
        assert_eq!(
            fish_schooling.fishes,
            [
                Fish(0),
                Fish(1),
                Fish(0),
                Fish(5),
                Fish(6),
                Fish(7),
                Fish(8)
            ]
        );

        fish_schooling.tick();
        assert_eq!(
            fish_schooling.fishes,
            [
                Fish(6),
                Fish(0),
                Fish(6),
                Fish(4),
                Fish(5),
                Fish(6),
                Fish(7),
                Fish(8),
                Fish(8)
            ]
        );
    }
}
