mod error;

use error::Error;

use std::fs;

fn main() -> Result<(), Error> {
    let data = get_input_data()?;
    let errors = find_corruption(&data);

    errors
        .iter()
        .for_each(|err| println!("error on line {} : unexpected '{}'", err.1, err.0));

    let score = errors.iter().fold(0_u64, |acc, c| acc + score(c.0));

    println!("score : {}", score);

    Ok(())
}

fn find_corruption(input: &str) -> Vec<(char, usize)> {
    let mut unexpected_chars: Vec<(char, usize)> = vec![];
    input.lines().enumerate().for_each(|(i, line)| {
        let mut context: Vec<char> = vec![];
        line.chars().for_each(|c| match c {
            '(' | '[' | '{' | '<' => context.push(c),
            ')' | ']' | '}' | '>' => match context.pop() {
                Some(ctx) => {
                    let expecting = match ctx {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => unreachable!(),
                    };
                    if c != expecting {
                        unexpected_chars.push((c, i));
                        return;
                    }
                }
                None => {
                    unexpected_chars.push((c, i));
                    return;
                }
            },
            _ => unreachable!(),
        });
    });

    unexpected_chars
}

fn score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn get_input_data() -> Result<String, Error> {
    let content = fs::read_to_string("input")?;
    Ok(content)
}
