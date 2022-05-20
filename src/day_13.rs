use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

enum FoldDir {
    X,
    Y,
}

impl FromStr for FoldDir {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(FoldDir::X),
            "y" => Ok(FoldDir::Y),
            _ => Err(format!("invalid fold dir: {}", s).into()),
        }
    }
}

struct Sheet {
    dots: Vec<Vec<bool>>,
    folds: VecDeque<(FoldDir, usize)>,
}

impl FromStr for Sheet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reached_end_of_sheet: bool = false;
        let mut dots: HashSet<(usize, usize)> = HashSet::new();
        let mut folds: VecDeque<(FoldDir, usize)> = VecDeque::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for l in s.lines() {
            if l.is_empty() {
                reached_end_of_sheet = true;
                continue;
            }
            if !reached_end_of_sheet {
                let parts: Vec<&str> = l.split(",").collect();
                if parts.len() != 2 {
                    return Err(format!("malformed line: {}", l).into());
                }
                let x = parts[0].parse()?;
                let y = parts[1].parse()?;
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                dots.insert((x, y));
            } else {
                let parts: Vec<&str> = l.split(" ").collect();
                if parts.len() != 3 || parts[0] != "fold" || parts[1] != "along" {
                    return Err(format!("malformed line: {}", l).into());
                }
                let sub_parts: Vec<&str> = parts[2].split("=").collect();
                if sub_parts.len() != 2 {
                    return Err(format!("malformed line: {}", l).into());
                }
                folds.push_back((sub_parts[0].parse()?, sub_parts[1].parse()?));
            }
        }
        max_x += 1;
        max_y += 1;
        let mut sheet = Sheet {
            dots: Vec::with_capacity(max_x),
            folds: folds,
        };
        for _ in 0..max_x {
            sheet.dots.push(vec![false; max_y]);
        }
        for &d in dots.iter() {
            sheet.dots[d.0][d.1] = true;
        }
        Ok(sheet)
    }
}

impl fmt::Debug for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.dots.len();
        let max_y = self.dots[0].len();
        for y in 0..max_y {
            for x in 0..max_x {
                write!(f, "{}", if self.dots[x][y] { '#' } else { '.' })?;
            }
            if y != max_y - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Sheet {
    fn fold(&mut self, n: usize) {
        for _ in 0..n {
            let f = self.folds.pop_front().unwrap();
            let max_x = self.dots.len();
            let max_y = self.dots[0].len();
            match f.0 {
                FoldDir::X => {
                    for x in (f.1 + 1)..max_x {
                        let x_off = x - f.1;
                        for y in 0..max_y {
                            self.dots[f.1 - x_off][y] |= self.dots[x][y];
                        }
                    }
                    self.dots.truncate(f.1);
                }
                FoldDir::Y => {
                    for x in 0..max_x {
                        for y in (f.1 + 1)..max_y {
                            let y_off = y - f.1;
                            self.dots[x][f.1 - y_off] |= self.dots[x][y];
                        }
                        self.dots[x].truncate(f.1);
                    }
                }
            }
        }
    }

    fn visible_dots(&self) -> usize {
        let mut count = 0;
        for x in 0..self.dots.len() {
            for y in 0..self.dots[x].len() {
                if self.dots[x][y] {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut sheet: Sheet = std::fs::read_to_string(input)?.parse()?;
    sheet.fold(1);
    Ok(sheet.visible_dots().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut sheet: Sheet = std::fs::read_to_string(input)?.parse()?;
    sheet.fold(sheet.folds.len());
    Ok(format!("{:?}", sheet))
}
