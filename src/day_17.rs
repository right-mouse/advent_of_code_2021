use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl FromStr for Area {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_malformed_input = Err(format!("malformed input: {}", s).into());
        let mut a: Area = Default::default();
        let parts = s.trim().split(": ").collect::<Vec<_>>();
        if parts.len() != 2 || parts[0] != "target area" {
            return err_malformed_input;
        }
        let sub_parts = parts[1].split(", ").collect::<Vec<_>>();
        if sub_parts.len() != 2 {
            return err_malformed_input;
        }
        let x_parts = sub_parts[0].split("=").collect::<Vec<_>>();
        let y_parts = sub_parts[1].split("=").collect::<Vec<_>>();
        if x_parts.len() != 2 || y_parts.len() != 2 || x_parts[0] != "x" || y_parts[0] != "y" {
            return err_malformed_input;
        }
        let x_bounds = x_parts[1].split("..").collect::<Vec<_>>();
        let y_bounds = y_parts[1].split("..").collect::<Vec<_>>();
        if x_bounds.len() != 2 || y_bounds.len() != 2 {
            return err_malformed_input;
        }
        a.min_x = x_bounds[0].parse()?;
        a.max_x = x_bounds[1].parse()?;
        a.min_y = y_bounds[0].parse()?;
        a.max_y = y_bounds[1].parse()?;
        Ok(a)
    }
}

fn get_max_height(a: &Area) -> i32 {
    // The maximum height is achieved by hitting the minimum y in the area. The highest value that
    // can reach this point is -(y_min + 1). It is reached after step -(y_min + 1) and has a value
    // of ((y * y) + y) / 2.
    let y = -(a.min_y + 1);
    ((y * y) + y) / 2
}

fn get_all_starting_coords(a: &Area) -> HashSet<(i32, i32)> {
    // The maximum height is achieved by hitting the minimum y in the area. The highest value that
    // can reach this point is -(y_min + 1). The area is reached after step -(y_min * 2). Use this
    // as a starting point and iterate down to get all possible starting y values.
    let y_max = -(a.min_y + 1);
    let mut y_vals = Vec::new();
    for y in (a.min_y)..=y_max {
        let mut pos = 0;
        let mut step = 0;
        loop {
            pos += y - step;
            step += 1;
            if pos <= a.max_y {
                if pos >= a.min_y {
                    y_vals.push((y, step));
                } else {
                    break;
                }
            }
        }
    }
    // The x velocity has to guarantee reaching at least the minimum x. Therefore, the minimum
    // value for x using the quadratic formula is ceil(sqrt((x_min * 8) + 1) / 2). It will reach
    // this position in x steps. Use this as a lower bound for iterating and finding all starting
    // x values for each starting y value.
    let x_min = (((((a.min_x * 8) + 1) as f64).sqrt() - 1.0) / 2.0).ceil() as i32;
    let mut starting_vals = HashSet::new();
    for &(y, step) in y_vals.iter() {
        for x in x_min..=a.max_x {
            let mut pos = 0;
            for s in 0..step {
                if x == s {
                    break;
                }
                pos += x - s;
            }
            if pos >= a.min_x && pos <= a.max_x {
                starting_vals.insert((x, y));
            }
        }
    }
    starting_vals
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let area: Area = std::fs::read_to_string(input)?.parse()?;
    Ok(get_max_height(&area).to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let area: Area = std::fs::read_to_string(input)?.parse()?;
    let start_vals = get_all_starting_coords(&area);
    Ok(start_vals.len().to_string())
}
