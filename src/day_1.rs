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
    for i in 3..nums.len() {
        let prev_tot = nums[i - 1] + nums[i - 2] + nums[i - 3];
        let cur_tot = nums[i] + nums[i - 1] + nums[i - 2];
        if cur_tot > prev_tot {
            count += 1;
        }
    }
    Ok(count.to_string())
}
