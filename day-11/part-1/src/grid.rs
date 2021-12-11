use std::{collections::HashSet, fmt::Display};

pub struct Grid {
    values: Vec<u8>,
    width: i64,
    height: i64,
}

impl Grid {
    pub fn new(values: Vec<u8>, width: i64, height: i64) -> Self {
        Self {
            values,
            width,
            height,
        }
    }

    pub fn step(&mut self) -> u64 {
        // start by incrementing everything by 1
        self.values.iter_mut().for_each(|v| *v += 1);

        // then flash stuff until everything above 9 has flashed once
        let mut flashed: HashSet<(i64, i64)> = HashSet::new();
        while self.values.contains(&10) {
            while let Some(v) = self.values.iter_mut().enumerate().find(|v| {
                let x = (v.0 as i64).rem_euclid(self.height);
                let y = (v.0 as i64).div_euclid(self.height);
                (*v.1 >= 10) && (!flashed.contains(&(x, y)))
            }) {
                let x = (v.0 as i64).rem_euclid(self.height);
                let y = (v.0 as i64).div_euclid(self.height);
                self.flash(x, y);
                flashed.insert((x, y));
            }
        }

        //then reset everything that flashed back to 0
        flashed.iter().for_each(|p| {
            if let Some(v) = self.get_mut(p.0, p.1) {
                *v = 0;
            }
        });

        flashed.len() as u64
    }

    pub fn is_synchronized(&self) -> bool {
        self.values.iter().min() == self.values.iter().max()
    }

    pub fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut u8> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds
            .then(|| self.values.get_mut((x + y * self.width) as usize))
            .unwrap_or(None)
    }

    pub fn flash(&mut self, x: i64, y: i64) {
        (-1..=1).into_iter().for_each(|dx| {
            (-1..=1).into_iter().for_each(|dy| {
                if let Some(v) = self.get_mut(x + dx, y + dy) {
                    *v += 1;
                }
            })
        });
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.values.chunks(self.width as usize).try_for_each(|l| {
            l.iter().try_for_each(|v| write!(f, "{} ", v))?;
            writeln!(f)
        })?;

        Ok(())
    }
}
