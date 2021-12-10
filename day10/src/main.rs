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

    let diagnostics = file.lines().map(|line| diagnose(line));

    let (lines, corrupted_lines): (Vec<Result<Vec<Token>, _>>, Vec<Result<_, ParseError>>) =
        diagnostics.partition(|diagnostic| diagnostic.is_ok());

    let part_1: usize = corrupted_lines
        .iter()
        .filter_map(|result| {
            if let Err(error) = result {
                Some(error)
            } else {
                panic!("Unexpected non-corrupted line")
            }
        })
        .map(|error| error.score())
        .sum();
    println!("Part 1: {}", part_1);

    let mut autocomplete_scores: Vec<usize> = lines
        .iter()
        .map(|line| line.as_ref().unwrap())
        .filter_map(|line| {
            if line.len() > 0 {
                Some(find_completion_tokens(line))
            } else {
                None
            }
        })
        .map(|tokens| autocomplete_score(&tokens))
        .collect();

    autocomplete_scores.sort();
    let part_2 = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("Part 2: {}", part_2);
}

fn diagnose(line: &str) -> Result<Vec<Token>, ParseError> {
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

    Ok(stack)
}

fn find_completion_tokens(line: &Vec<Token>) -> Vec<Token> {
    let mut result = vec![];
    let mut unopened_tokens = 0;

    for token in line.iter().rev() {
        use Token::*;

        match token {
            LeftParen => {
                if unopened_tokens == 0 {
                    result.push(RightParen);
                } else {
                    unopened_tokens -= 1;
                }
            }
            LeftBracket => {
                if unopened_tokens == 0 {
                    result.push(RightBracket);
                } else {
                    unopened_tokens -= 1;
                }
            }
            LeftBrace => {
                if unopened_tokens == 0 {
                    result.push(RightBrace);
                } else {
                    unopened_tokens -= 1;
                }
            }
            LeftAngle => {
                if unopened_tokens == 0 {
                    result.push(RightAngle);
                } else {
                    unopened_tokens -= 1;
                }
            }
            _ => unopened_tokens += 1,
        }
    }

    result
}

fn autocomplete_score(tokens: &Vec<Token>) -> usize {
    let mut score = 0;

    for token in tokens.iter() {
        use Token::*;

        score *= 5;

        score += match token {
            RightParen => 1,
            RightBracket => 2,
            RightBrace => 3,
            RightAngle => 4,
            token => panic!("Unexpected closing token {:?}", token),
        }
    }

    score
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

#[derive(Debug)]
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
