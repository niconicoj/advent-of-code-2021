mod error;

use error::Error;

use std::fs;

fn main() -> Result<(), Error> {
    let data = get_input_data()?;
    let result_list = find_incomplete_chunks(&data);

    let mut scores = result_list
        .iter()
        .map(|l| score(&complete_chunk(&l)))
        .collect::<Vec<u64>>();

    scores.sort();

    let middle_score = scores.get(scores.len() / 2).unwrap();

    println!("middle score : {}", middle_score);

    Ok(())
}

fn find_incomplete_chunks(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| {
            line.chars().try_fold(vec![], |mut acc, c| {
                match c {
                    '(' | '[' | '{' | '<' => acc.push(c),
                    ')' | ']' | '}' | '>' => match acc.pop() {
                        Some(ctx) => {
                            let expecting = match ctx {
                                '(' => ')',
                                '[' => ']',
                                '{' => '}',
                                '<' => '>',
                                _ => unreachable!(),
                            };
                            if c != expecting {
                                return None;
                            }
                        }
                        None => {
                            return None;
                        }
                    },
                    _ => unreachable!(),
                };
                Some(acc)
            })
        })
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn complete_chunk(input: &str) -> String {
    input
        .chars()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        })
        .collect()
}

fn score(s: &str) -> u64 {
    s.chars().fold(0_u64, |acc, c| match c {
        ')' => acc * 5 + 1,
        ']' => acc * 5 + 2,
        '}' => acc * 5 + 3,
        '>' => acc * 5 + 4,
        _ => unreachable!(),
    })
}

fn get_input_data() -> Result<String, Error> {
    let content = fs::read_to_string("input")?;
    Ok(content)
}
