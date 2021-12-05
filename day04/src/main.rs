use std::{
    collections::VecDeque,
    env, fmt,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let mut lines = read_lines()
        .expect("Error reading file")
        .map(|line| line.expect("Error reading line"))
        .peekable();

    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    lines.next();

    let mut boards: Vec<Board> = vec![];

    while lines.peek().is_some() {
        let board_lines: Vec<String> = lines.by_ref().take_while(|line| line != "").collect();
        boards.push(Board::new(board_lines));
    }

    let mut boards_iterator = iterate_boards_in_winning_order(boards, numbers);

    let (number, winning_board) = boards_iterator.next().unwrap();
    let unmarked_sum: usize = winning_board.unmarked_numbers().iter().sum();
    println!("Part 1: {}", unmarked_sum * number);

    let (number, winning_board) = boards_iterator.last().unwrap();
    let unmarked_sum: usize = winning_board.unmarked_numbers().iter().sum();
    println!("Part 2: {}", unmarked_sum * number);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cell {
    number: usize,
    checked: bool,
}

impl Cell {
    fn new(number: usize) -> Self {
        Cell {
            number,
            checked: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Board {
    rows: Vec<Vec<Cell>>,
}

impl Board {
    fn new(rows: Vec<String>) -> Self {
        let built_rows: Vec<Vec<Cell>> = rows
            .into_iter()
            .map(|row| {
                row.split(' ')
                    .map(|number| number.trim())
                    .filter(|number| *number != "")
                    .map(|number| number.parse().unwrap())
                    .map(Cell::new)
                    .collect()
            })
            .collect();

        Board { rows: built_rows }
    }

    fn numbers(&self) -> Vec<usize> {
        self.rows
            .iter()
            .flat_map(|cells| cells.iter().map(|cell| cell.number))
            .collect()
    }

    fn unmarked_numbers(&self) -> Vec<usize> {
        self.rows
            .iter()
            .flat_map(|cells| {
                cells
                    .iter()
                    .filter(|cell| !cell.checked)
                    .map(|cell| cell.number)
            })
            .collect()
    }

    fn check(&mut self, number: usize) {
        for row in &mut self.rows {
            for mut cell in row {
                if cell.number == number {
                    cell.checked = true;
                    return;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        if self
            .rows
            .iter()
            .any(|row| row.iter().all(|cell| cell.checked))
        {
            return true;
        }

        self.columns()
            .iter()
            .any(|column| column.iter().all(|cell| cell.checked))
    }

    fn columns(&self) -> Vec<Vec<&Cell>> {
        let length = self.rows.first().unwrap().len();
        let mut columns: Vec<Vec<&Cell>> = Vec::new();

        for _ in 0..length {
            columns.push(vec![]);
        }

        for row in &self.rows {
            for (index, cell) in row.iter().enumerate() {
                columns[index].push(cell);
            }
        }

        columns
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Find the maximum number of digits in decimal
        let number_width = self
            .numbers()
            .iter()
            // Probably should be log10 but Rust only has it for floats
            .map(|n| n.to_string().len())
            .max()
            .unwrap();

        for row in &self.rows {
            for cell in row {
                if cell.checked {
                    write!(
                        f,
                        "{:^width$}",
                        format!("*{}*", cell.number),
                        width = number_width + 4
                    )?;
                } else {
                    write!(f, "{:^width$}", cell.number, width = number_width + 4)?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn iterate_boards_in_winning_order(boards: Vec<Board>, numbers: Vec<usize>) -> BoardsIterator {
    BoardsIterator {
        boards,
        numbers: VecDeque::from(numbers),
        last_number: 0,
    }
}

struct BoardsIterator {
    boards: Vec<Board>,
    numbers: VecDeque<usize>,
    last_number: usize,
}

impl Iterator for BoardsIterator {
    type Item = (usize, Board);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.boards.iter().any(|board| board.has_won()) && !self.numbers.is_empty() {
            self.last_number = self.numbers.pop_front().unwrap();

            for board in &mut self.boards {
                board.check(self.last_number);
            }
        }

        if let Some(next_winning_board_index) = self.boards.iter().position(|board| board.has_won())
        {
            Some((
                self.last_number,
                self.boards.remove(next_winning_board_index),
            ))
        } else {
            None
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
