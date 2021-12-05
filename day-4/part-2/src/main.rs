mod error;
use std::{
    fs,
    io::{BufRead, BufReader},
};

use error::Error;
use ndarray::{Array, Array2};

#[derive(Debug)]
struct BingoBoard {
    numbers: Array2<Option<u64>>,
}

impl BingoBoard {
    pub fn new(numbers: Array2<Option<u64>>) -> Self {
        Self { numbers }
    }

    pub fn check(&self) -> bool {
        self.numbers.rows().into_iter().fold(false, |acc, row| {
            acc | row.iter().fold(true, |acc, number| acc & number.is_none())
        }) | self.numbers.columns().into_iter().fold(false, |acc, row| {
            acc | row.iter().fold(true, |acc, number| acc & number.is_none())
        })
    }

    pub fn score(&self) -> u64 {
        self.numbers.rows().into_iter().fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, number| acc + number.unwrap_or(0))
        })
    }

    pub fn mark(&mut self, drawn_number: u64) {
        self.numbers.iter_mut().for_each(|number| match number {
            Some(v) => {
                if *v == drawn_number {
                    *number = None
                }
            }
            _ => {}
        });
    }
}

fn main() -> Result<(), Error> {
    let mut bingo_boards: Vec<BingoBoard> = vec![];

    let numbers = parse_input(&mut bingo_boards)?;

    let mut numbers_iter = numbers.iter();
    let mut score: Option<u64> = None;

    while let Some(number) = numbers_iter.next() {
        println!("drawing {}", number);
        println!("board count before filter : {}", bingo_boards.len());
        bingo_boards
            .iter_mut()
            .for_each(|board| board.mark(*number));
        bingo_boards.retain(|board| !board.check());
        println!("board count after filter : {}", bingo_boards.len());

        if bingo_boards.len() == 1 && score.is_none() {
            println!("last standing board found");
            println!("{:?}", bingo_boards.get(0).unwrap());
            score = Some(bingo_boards.get(0).unwrap().score());
            println!("score : {}", score.unwrap());
        };
        if bingo_boards.is_empty() {
            println!("all board won after drawing {}", number);
            println!("final score : {}", (score.unwrap() - number) * number);
            break;
        };
    }

    Ok(())
}

fn parse_input(bingo_boards: &mut Vec<BingoBoard>) -> Result<Vec<u64>, Error> {
    let input_file = fs::File::open("input")?;
    let mut lines = BufReader::new(input_file).lines();

    let numbers = lines
        .next()
        .ok_or(Error::BadFormat)??
        .split(',')
        .map(|e| e.parse::<u64>().or(Err(Error::BadFormat)))
        .collect::<Result<Vec<u64>, Error>>()?;

    // discard empty line
    lines.next();
    let mut arr: Vec<Option<u64>> = vec![];
    lines.try_for_each(|r| {
        match r {
            Ok(line) => {
                if line.is_empty() {
                    bingo_boards.push(BingoBoard::new(Array::from_shape_vec((5, 5), arr.clone())?));
                    arr = vec![];
                } else {
                    let mut parsed_numbers = line
                        .split_whitespace()
                        .map(|e| Some(e.parse::<u64>().unwrap()))
                        .collect::<Vec<Option<u64>>>();

                    arr.append(&mut parsed_numbers);
                }
            }
            Err(_) => return Err(Error::BadFormat),
        }
        Ok(())
    })?;

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use crate::BingoBoard;

    #[test]
    fn bingo_board_check_horizontal() {
        let bingo_board = BingoBoard::new(array![
            [Some(1), Some(2), Some(3), None],
            [Some(5), None, Some(7), Some(1)],
            [Some(9), Some(10), None, Some(12)],
            [None, None, None, None],
        ]);

        assert!(bingo_board.check());
    }

    #[test]
    fn bingo_board_check_vertical() {
        let bingo_board = BingoBoard::new(array![
            [Some(1), Some(2), None, Some(4)],
            [Some(5), Some(6), None, Some(1)],
            [Some(9), Some(10), None, Some(12)],
            [Some(13), Some(14), None, Some(16)],
        ]);
        assert!(bingo_board.check());
    }

    #[test]
    fn bingo_board_score() {
        let bingo_board = BingoBoard::new(array![
            [Some(1), Some(2), Some(3), None],
            [Some(5), None, Some(7), Some(1)],
            [Some(9), Some(10), None, Some(12)],
            [None, Some(14), Some(15), None],
        ]);

        assert_eq!(bingo_board.score(), 79);
    }

    #[test]
    fn bingo_board_mark() {
        let mut bingo_board = BingoBoard::new(array![
            [Some(1), Some(2), Some(3), None],
            [Some(5), None, Some(7), Some(1)],
            [Some(9), Some(10), None, Some(12)],
            [None, Some(14), Some(15), None],
        ]);

        assert_eq!(bingo_board.score(), 79);
        bingo_board.mark(12);
        assert_eq!(bingo_board.score(), 67);
        bingo_board.mark(13);
        assert_eq!(bingo_board.score(), 67);
        bingo_board.mark(3);
        assert_eq!(bingo_board.score(), 64);
    }
}
