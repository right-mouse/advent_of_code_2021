use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

const CYCLE_TIME: usize = 7;
const INIT_CYCLE_TIME: usize = 9;

struct Colony {
    // An array of bins containing how many fish have their internal timers
    // equal to each of the possible values.
    timer_bins: [u64; INIT_CYCLE_TIME],
}

impl FromStr for Colony {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(",")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .collect();
        let mut colony = Colony {
            timer_bins: [0; INIT_CYCLE_TIME],
        };
        for p in parts.into_iter() {
            colony.timer_bins[p.parse::<usize>()?] += 1;
        }
        Ok(colony)
    }
}

impl Colony {
    fn advance_day(&mut self) {
        let mut new_bins: [u64; INIT_CYCLE_TIME] = [0; INIT_CYCLE_TIME];
        for i in 1..INIT_CYCLE_TIME {
            new_bins[i - 1] = self.timer_bins[i];
        }
        new_bins[CYCLE_TIME - 1] += self.timer_bins[0];
        new_bins[INIT_CYCLE_TIME - 1] += self.timer_bins[0];
        self.timer_bins = new_bins;
    }

    fn get_total(&self) -> u64 {
        let mut total = 0;
        for b in self.timer_bins {
            total += b;
        }
        total
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut colony: Colony = line.parse()?;
    for _ in 0..80 {
        colony.advance_day();
    }
    Ok(colony.get_total().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut colony: Colony = line.parse()?;
    for _ in 0..256 {
        colony.advance_day();
    }
    Ok(colony.get_total().to_string())
}
