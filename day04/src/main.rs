use std::{
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

    let mut numbers_iter = numbers.iter();
    let mut last_number = 0;

    while !boards.iter().any(|board| board.has_won()) {
        last_number = *numbers_iter
            .next()
            .expect("Ran out of numbers before any board won");

        for board in &mut boards {
            board.check(last_number);
        }
    }

    let winning_board = boards.iter().find(|board| board.has_won()).unwrap();
    let unmarked_sum: usize = winning_board.unmarked_numbers().iter().sum();
    println!("Part 1: {}", unmarked_sum * last_number);
}

#[derive(Debug)]
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

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
