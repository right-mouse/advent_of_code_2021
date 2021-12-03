use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in reader.lines() {
        let instruction = line?;
        let parts: Vec<&str> = instruction.split(" ").collect();
        if parts.len() != 2 {
            return Err(format!("invalid instruction: {}", instruction).into());
        }
        let val = parts[1].parse::<i32>()?;
        match parts[0] {
            "forward" => horizontal_pos += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => return Err(format!("{} is an invalid move command", parts[0]).into()),
        }
    }
    Ok((horizontal_pos * depth).to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in reader.lines() {
        let instruction = line?;
        let parts: Vec<&str> = instruction.split(" ").collect();
        if parts.len() != 2 {
            return Err(format!("invalid instruction: {}", instruction).into());
        }
        let val = parts[1].parse::<i32>()?;
        match parts[0] {
            "down" => aim += val,
            "up" => aim -= val,
            "forward" => {
                horizontal_pos += val;
                depth += aim * val;
            }
            _ => return Err(format!("{} is an invalid move command", parts[0]).into()),
        }
    }
    Ok((horizontal_pos * depth).to_string())
}
