use std::{convert::TryFrom, env};

use anyhow::bail;

fn main() {
    let file = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        include_str!("../sample.txt")
    } else {
        include_str!("../input.txt")
    };

    let part_1: usize = file
        .lines()
        .map(|line| diagnose(line))
        .filter_map(|result| {
            if let Err(error) = result {
                Some(error)
            } else {
                None
            }
        })
        .map(|error| error.score())
        .sum();
    println!("Part 1: {}", part_1);
}

fn diagnose(line: &str) -> Result<(), ParseError> {
    let mut stack = vec![];

    for token in line.chars().map(|char| Token::try_from(char).unwrap()) {
        use Token::*;

        // The problem statement only considers two errors - when the line is incomplete (and for
        // part 1, we don't care about that one) and when there's an incorrect closing bracket. That
        // means it's not accounting for extra closing brackets (e.g. '<[]>)'), so we'll ignore that
        // case for now
        match token {
            LeftAngle | LeftBrace | LeftBracket | LeftParen => stack.push(token),
            RightParen => {
                let last_token = stack.pop().unwrap();
                if last_token != LeftParen {
                    return Err(ParseError(RightParen));
                }
            }
            RightBracket => {
                let last_token = stack.pop().unwrap();
                if last_token != LeftBracket {
                    return Err(ParseError(RightBracket));
                }
            }
            RightBrace => {
                let last_token = stack.pop().unwrap();
                if last_token != LeftBrace {
                    return Err(ParseError(RightBrace));
                }
            }
            RightAngle => {
                let last_token = stack.pop().unwrap();
                if last_token != LeftAngle {
                    return Err(ParseError(RightAngle));
                }
            }
        }
    }

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Token {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftAngle,
    RightAngle,
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Token::*;

        match value {
            '(' => Ok(LeftParen),
            ')' => Ok(RightParen),
            '[' => Ok(LeftBracket),
            ']' => Ok(RightBracket),
            '{' => Ok(LeftBrace),
            '}' => Ok(RightBrace),
            '<' => Ok(LeftAngle),
            '>' => Ok(RightAngle),
            _ => bail!("Unexpected token {}", value),
        }
    }
}

struct ParseError(Token);

impl ParseError {
    fn score(&self) -> usize {
        use Token::*;

        match &self.0 {
            RightParen => 3,
            RightBracket => 57,
            RightBrace => 1197,
            RightAngle => 25137,
            token => panic!("Unexpected parse error token {:?}", token),
        }
    }
}
