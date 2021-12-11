use std::collections::{HashMap, HashSet};

pub struct Grid {
    values: Vec<u8>,
    height: i64,
    width: i64,
}

impl Grid {
    pub fn new(values: Vec<u8>, width: i64, height: i64) -> Self {
        Self {
            values,
            width,
            height,
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }

    pub fn find_basins_size(&self) -> Vec<u64> {
        let low_points = self.get_low_points();

        let mut visited_points = HashSet::new();

        low_points
            .keys()
            .map(|point| {
                println!("computing basins from [{}, {}]", point.0, point.1);
                self.basin_recurse(&mut visited_points, *point)
            })
            .collect()
    }

    fn basin_recurse(&self, visited_points: &mut HashSet<(i64, i64)>, point: (i64, i64)) -> u64 {
        if !visited_points.insert((point.0, point.1)) {
            println!("already visited [{},{}]", point.0, point.1);
            return 0;
        }
        let mut size = 0;
        for neighbour in [
            (point.0 - 1, point.1),
            (point.0, point.1 - 1),
            (point.0 + 1, point.1),
            (point.0, point.1 + 1),
        ] {
            match self.get(neighbour.0, neighbour.1) {
                Some(p) => {
                    if p == 9 {
                        continue;
                    } else {
                        size += self.basin_recurse(visited_points, neighbour);
                    }
                }
                None => continue,
            }
        }
        return 1 + size;
    }

    pub fn get_low_points(&self) -> HashMap<(i64, i64), u8> {
        let mut low_points_map = HashMap::new();
        self.values
            .iter()
            .enumerate()
            .map(|(i, v)| {
                (
                    v,
                    (i as i64).rem_euclid(self.width),
                    (i as i64).div_euclid(self.width),
                )
            })
            .for_each(|p| {
                let mut low_point_flag = true;
                for neighbour in [
                    (p.1 - 1, p.2),
                    (p.1, p.2 - 1),
                    (p.1 + 1, p.2),
                    (p.1, p.2 + 1),
                ] {
                    match self.get(neighbour.0, neighbour.1) {
                        Some(v) => {
                            if v <= *p.0 {
                                low_point_flag = false;
                                break;
                            }
                        }
                        None => continue,
                    }
                }
                if low_point_flag {
                    low_points_map.insert((p.1, p.2), p.0 + 1);
                }
            });
        low_points_map
    }
}

impl IntoIterator for Grid {
    type Item = (u8, i64, i64);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, (i as i64) % self.width, (i as i64) / self.height))
            .collect::<Vec<(u8, i64, i64)>>()
            .into_iter()
    }
}
