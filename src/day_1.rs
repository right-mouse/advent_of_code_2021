use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut prev = i32::MAX;
    for line in reader.lines() {
        let num = line?.parse::<i32>()?;
        if num > prev {
            count += 1;
        }
        prev = num;
    }
    Ok(count.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut nums: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let num = line?.parse::<i32>()?;
        nums.push(num);
    }
    let mut count = 0;
    let mut prev_tot = i32::MAX;
    for i in 2..nums.len() {
        let cur_tot = nums[i] + nums[i - 1] + nums[i - 2];
        if cur_tot > prev_tot {
            count += 1;
        }
        prev_tot = cur_tot;
    }
    Ok(count.to_string())
}
