use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

const BINGO_BOARD_DIM: usize = 5;

struct BingoBoard {
    vals: [[u8; BINGO_BOARD_DIM]; BINGO_BOARD_DIM],
    marks: [[bool; BINGO_BOARD_DIM]; BINGO_BOARD_DIM],
}

impl FromStr for BingoBoard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != BINGO_BOARD_DIM {
            return Err(
                format!("expected {} lines but got {}", BINGO_BOARD_DIM, lines.len()).into(),
            );
        }
        let mut b = BingoBoard {
            vals: [[0; BINGO_BOARD_DIM]; BINGO_BOARD_DIM],
            marks: [[false; BINGO_BOARD_DIM]; BINGO_BOARD_DIM],
        };
        for (i, x) in lines.into_iter().enumerate() {
            let nums: Vec<&str> = x
                .split(" ")
                .filter_map(|n| {
                    let t = n.trim();
                    if !t.is_empty() {
                        Option::from(t)
                    } else {
                        None
                    }
                })
                .collect();
            if nums.len() != BINGO_BOARD_DIM {
                return Err(format!(
                    "expected {} nums in line {} but got {}",
                    BINGO_BOARD_DIM,
                    i,
                    nums.len()
                )
                .into());
            }
            for (j, y) in nums.into_iter().enumerate() {
                b.vals[i][j] = y.parse()?;
            }
        }
        Ok(b)
    }
}

impl BingoBoard {
    fn mark_num(&mut self, n: u8) {
        for i in 0..BINGO_BOARD_DIM {
            for j in 0..BINGO_BOARD_DIM {
                if self.vals[i][j] == n {
                    self.marks[i][j] = true;
                    return;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        // Check rows.
        for i in 0..BINGO_BOARD_DIM {
            let mut b = true;
            for j in 0..BINGO_BOARD_DIM {
                b = b && self.marks[i][j];
            }
            if b {
                return true;
            }
        }
        // Check cols.
        for i in 0..BINGO_BOARD_DIM {
            let mut b = true;
            for j in 0..BINGO_BOARD_DIM {
                b = b && self.marks[j][i];
            }
            if b {
                return true;
            }
        }
        false
    }

    fn unmarked_nums_sum(&self) -> u16 {
        let mut sum: u16 = 0;
        for i in 0..BINGO_BOARD_DIM {
            for j in 0..BINGO_BOARD_DIM {
                if !self.marks[i][j] {
                    sum += self.vals[i][j] as u16;
                }
            }
        }
        sum
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let inputs_str = std::mem::take(&mut line);
    let inputs: Vec<_> = inputs_str
        .trim()
        .split(",")
        .map(|s| s.parse::<u8>())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    while reader.read_line(&mut line)? != 0 {
        std::mem::take(&mut line);
        let mut board_str = String::new();
        for _ in 0..BINGO_BOARD_DIM {
            reader.read_line(&mut line)?;
            board_str.push_str(&std::mem::take(&mut line));
        }
        boards.push(board_str.trim().parse()?);
    }
    for input in inputs {
        let i = input?;
        for board in boards.iter_mut() {
            board.mark_num(i);
            if board.has_won() {
                return Ok((board.unmarked_nums_sum() * (i as u16)).to_string());
            }
        }
    }
    Err(format!("No board won").into())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let inputs_str = std::mem::take(&mut line);
    let inputs: Vec<_> = inputs_str
        .trim()
        .split(",")
        .map(|s| s.parse::<u8>())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    while reader.read_line(&mut line)? != 0 {
        std::mem::take(&mut line);
        let mut board_str = String::new();
        for _ in 0..BINGO_BOARD_DIM {
            reader.read_line(&mut line)?;
            board_str.push_str(&std::mem::take(&mut line));
        }
        boards.push(board_str.trim().parse()?);
    }
    for input in inputs {
        let i = input?;
        for board in boards.iter_mut() {
            board.mark_num(i);
        }
        if boards.len() > 1 {
            boards = boards.into_iter().filter(|b| !b.has_won()).collect();
        } else {
            if boards[0].has_won() {
                return Ok((boards[0].unmarked_nums_sum() * (i as u16)).to_string());
            }
        }
    }
    Err(format!("No board won").into())
}
