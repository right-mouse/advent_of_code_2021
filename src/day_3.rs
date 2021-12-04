use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const NUM_BITS: usize = 12;

enum RatingType {
    Oxygen,
    CO2,
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut num_diagnostic_vals = 0;
    let mut bit_counts: [u16; NUM_BITS] = [0; NUM_BITS];
    for line in reader.lines() {
        let diagnostic_val_str: &str = &line?;
        let diagnostic_val = u16::from_str_radix(diagnostic_val_str, 2)?;
        num_diagnostic_vals += 1;
        for i in 0..NUM_BITS {
            bit_counts[i] += (diagnostic_val >> i) & 1;
        }
    }
    let mut gamma: u16 = 0;
    for i in 0..NUM_BITS {
        if bit_counts[i] > (num_diagnostic_vals / 2) {
            gamma |= 1 << i;
        }
    }
    let epsilon = !gamma & 0b111111111111;
    Ok(((gamma as u32) * (epsilon as u32)).to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut diagnostic_vals: Vec<u16> = Vec::new();
    for line in reader.lines() {
        let diagnostic_val: &str = &line?;
        diagnostic_vals.push(u16::from_str_radix(diagnostic_val, 2)?);
    }
    let mut oxygen_rating: u16 = 0;
    let mut co2_rating: u16 = 0;
    let mut oxygen_filtered = diagnostic_vals.clone();
    let mut co2_filtered = diagnostic_vals.clone();
    for i in (0..NUM_BITS).rev() {
        if oxygen_filtered.len() > 1 {
            oxygen_filtered = apply_bit_criteria(&oxygen_filtered, i, RatingType::Oxygen);
            if oxygen_filtered.len() == 1 {
                oxygen_rating = oxygen_filtered[0];
            }
        }
        if co2_filtered.len() > 1 {
            co2_filtered = apply_bit_criteria(&co2_filtered, i, RatingType::CO2);
            if co2_filtered.len() == 1 {
                co2_rating = co2_filtered[0];
            }
        }
        if oxygen_filtered.len() == 1 && co2_filtered.len() == 1 {
            break;
        }
    }
    Ok(((oxygen_rating as u32) * (co2_rating as u32)).to_string())
}

fn apply_bit_criteria(vals: &Vec<u16>, bit: usize, typ: RatingType) -> Vec<u16> {
    let mut filtered: Vec<u16> = Vec::new();
    let mut bit_count: u16 = 0;
    for v in vals {
        bit_count += (v >> bit) & 1;
    }
    let mut match_bit: u16;
    match typ {
        RatingType::Oxygen => match_bit = 0,
        RatingType::CO2 => match_bit = 1,
    }
    if bit_count as f32 >= (vals.len() as f32 / 2.0) {
        match_bit = !match_bit & 1;
    }
    for v in vals {
        if (v >> bit) & 1 == match_bit {
            filtered.push(*v);
        }
    }
    filtered
}
