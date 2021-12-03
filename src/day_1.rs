use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut num_increases = 0;
    let mut prev_depth = i32::MAX;
    for line in reader.lines() {
        let cur_depth = line?.parse::<i32>()?;
        if cur_depth > prev_depth {
            num_increases += 1;
        }
        prev_depth = cur_depth;
    }
    Ok(num_increases.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut depths: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let num = line?.parse::<i32>()?;
        depths.push(num);
    }
    let mut num_increases = 0;
    let mut prev_total = i32::MAX;
    for i in 2..depths.len() {
        let cur_total = depths[i] + depths[i - 1] + depths[i - 2];
        if cur_total > prev_total {
            num_increases += 1;
        }
        prev_total = cur_total;
    }
    Ok(num_increases.to_string())
}
