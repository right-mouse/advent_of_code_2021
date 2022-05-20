use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

struct Positions(Vec<i32>);

impl FromStr for Positions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(",")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .collect();
        let mut pos = Positions(Vec::with_capacity(parts.len()));
        for p in parts.into_iter() {
            pos.0.push(p.parse()?);
        }
        pos.0.sort();
        Ok(pos)
    }
}

// https://i1115.photobucket.com/albums/k544/akinuri/nth%20triangle%20number-01.jpg
fn nth_triangle(n: i32) -> i32 {
    return n * (n + 1) / 2;
}

impl Positions {
    fn get_median(&self) -> i32 {
        let mid = self.0.len() / 2;
        self.0[mid]
    }

    fn get_adjusted_mean(&self) -> i32 {
        // The mean is sufficient as the optimal value when f = d^2, where f is
        // the fuel and d is the distance. However, the actual distance is
        // given by f = d*(d+1)/2. Therefore, the expression for the adjusted
        // value can be derived by first setting up a cost function as follows:
        //     n
        //     ===== /d  - M\ /d  - M + 1\
        //     \     \ i    / \ i        /
        // C =  >    ---------------------
        //     /               2
        //     =====
        //     i = 1
        // where C is the total cost and M is the optimal value. If the
        // derivative of C with respect to M is set to 0 and M is solved for,
        // the adjusted mean is given by:
        //        n
        //      =====      n - 2n
        //     1\                1
        // M = - >    d  + --------
        //     n/      i     2n
        //      =====
        //      i = 1
        // where n_1 is the number of values less than the mean.
        let len = self.0.len() as f32;
        let mean = self.0.iter().sum::<i32>() as f32 / len;
        let n_1 = self.0.iter().filter(|n| (**n as f32) < mean).count() as f32;
        let mut adjusted_mean = mean;
        adjusted_mean += (len - (2.0 * n_1)) / (2.0 * len);
        adjusted_mean.round() as i32
    }

    fn sum_of_differences(&self) -> i32 {
        let mut sum = 0;
        let median = self.get_median();
        for p in self.0.iter() {
            sum += (*p - median).abs();
        }
        sum
    }

    fn sum_of_nth_triangle_differences(&self) -> i32 {
        let mut sum = 0;
        let mean = self.get_adjusted_mean();
        for p in self.0.iter() {
            sum += nth_triangle((*p - mean).abs());
        }
        sum
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let p = line.parse::<Positions>()?;
    Ok(p.sum_of_differences().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let p = line.parse::<Positions>()?;
    Ok(p.sum_of_nth_triangle_differences().to_string())
}
