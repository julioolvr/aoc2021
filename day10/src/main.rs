use std::{convert::TryFrom, env};

use anyhow::bail;

/**
 * --- Day 10: Syntax Scoring ---
 *
 * The input is a list of lines, where each line is a combination of ()[]{}<>. Both parts revolve
 * around finding unbalanced lines and calculating values based on it. The problem statement
 * considers three cases for each of the lines - they might be incomplete (all the characters
 * present are balanced, but the string ends before closing al of them), corrupted (a closing
 * character is present that does not match the currently open one) and (not explicitly mentioned)
 * "balanced" lines (all pairs are properly balanced).
 *
 * The `diagnose` function takes a line (as a string) and returns a result. Balanced and incomplete
 * lines are considered Ok, and corrupted lines are an error (of type `ParseError` that internally
 * contains the offending token). In the main program, Ok and Err are divided into two separate
 * lists.
 *
 * For part 1 we care about the errors (i.e. corrupted lines). We need to calculate a score for each
 * based on the offending character which is why it's included in `ParseError`. We add them all up
 * and get our result.
 *
 * For part 2 we care about incomplete lines. `diagnose` internally uses a stack to which it adds
 * tokens for ([{< and removes tokens for )]}>. The final state of the stack is what's returned in
 * the Ok case. A balanced line will have an empty stack, and an incomplete one will have some
 * remaining tokens on it. Part 2 asks to calculate a score based on the necessary closing tokens
 * that would balance those incomplete lines. We pop elements from the stack one by one and check
 * which token needs to be used. We might run into closing tokens too - this means and internally
 * balanced pair that we have to ignore. E.g. '[()' has a pair of matching parenthesis, so the only
 * necessary token to balance it out is the closing ]. For that reason we keep track of "unopened"
 * pairs when we find a closing token to know that we have to ignore it when it's opened. Since
 * diagnose already checked that the pairs match correctly, we can only count them, there's no need
 * to check if they're balanced again. The rest is calculating the score based on the result.
 */

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
        .map(Result::as_ref)
        .map(Result::unwrap_err)
        .map(|error| error.score())
        .sum();
    println!("Part 1: {}", part_1);

    let mut autocomplete_scores: Vec<usize> = lines
        .iter()
        .map(Result::as_ref)
        .map(Result::unwrap)
        .map(|line| find_completion_tokens(line))
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
    tokens.iter().fold(0, |score, token| {
        use Token::*;

        score * 5
            + match token {
                RightParen => 1,
                RightBracket => 2,
                RightBrace => 3,
                RightAngle => 4,
                token => panic!("Unexpected closing token {:?}", token),
            }
    })
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
