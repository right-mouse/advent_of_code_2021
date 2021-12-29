use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SeaCucumber {
    East,
    South,
}

impl FromStr for SeaCucumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(SeaCucumber::East),
            "v" => Ok(SeaCucumber::South),
            _ => Err("not a sea cucumber".to_string()),
        }
    }
}

struct SeaFloor {
    region: Vec<Vec<Option<SeaCucumber>>>,
}

impl FromStr for SeaFloor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sea_floor = SeaFloor { region: Vec::new() };
        for l in s.lines() {
            sea_floor.region.push(
                l.chars()
                    .map(|c| c.to_string().parse::<SeaCucumber>().ok())
                    .collect(),
            );
        }
        Ok(sea_floor)
    }
}

impl SeaFloor {
    fn advance_step(&mut self) -> usize {
        let mut east_moves = Vec::new();
        for i in 0..self.region.len() {
            for j in 0..self.region[i].len() {
                match self.region[i][j] {
                    Some(sc) => match sc {
                        SeaCucumber::East => {
                            let mut k = j + 1;
                            if k >= self.region[i].len() {
                                k -= self.region[i].len();
                            }
                            if self.region[i][k].is_none() {
                                east_moves.push(((i, j), (i, k)));
                            }
                        }
                        _ => (),
                    },
                    None => (),
                };
            }
        }
        for m in east_moves.iter() {
            self.region[m.1 .0][m.1 .1] = self.region[m.0 .0][m.0 .1];
            self.region[m.0 .0][m.0 .1] = None;
        }
        let mut south_moves = Vec::new();
        for i in 0..self.region.len() {
            for j in 0..self.region[i].len() {
                match self.region[i][j] {
                    Some(sc) => match sc {
                        SeaCucumber::South => {
                            let mut k = i + 1;
                            if k >= self.region.len() {
                                k -= self.region.len();
                            }
                            if self.region[k][j].is_none() {
                                south_moves.push(((i, j), (k, j)));
                            }
                        }
                        _ => (),
                    },
                    None => (),
                };
            }
        }
        for m in south_moves.iter() {
            self.region[m.1 .0][m.1 .1] = self.region[m.0 .0][m.0 .1];
            self.region[m.0 .0][m.0 .1] = None;
        }
        east_moves.len() + south_moves.len()
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut sea_floor: SeaFloor = std::fs::read_to_string(input)?.parse()?;
    let mut step = 0;
    loop {
        step += 1;
        if sea_floor.advance_step() == 0 {
            break;
        }
    }
    Ok(step.to_string())
}
