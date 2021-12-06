use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

struct Line {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 3 || parts[1] != "->" {
            return Err(format!("malformed line: {}", s).into());
        }
        let p0 = parts[0]
            .split(",")
            .map(|n| n.parse::<u16>())
            .collect::<Vec<_>>();
        let p1 = parts[2]
            .split(",")
            .map(|n| n.parse::<u16>())
            .collect::<Vec<_>>();
        if p0.len() != 2 || p1.len() != 2 {
            return Err(format!("malformed line: {}", s).into());
        }
        Ok(Line {
            x0: p0[0].clone()?,
            y0: p0[1].clone()?,
            x1: p1[0].clone()?,
            y1: p1[1].clone()?,
        })
    }
}

impl Line {
    fn max_x(&self) -> u16 {
        max(self.x0, self.x1)
    }

    fn max_y(&self) -> u16 {
        max(self.y0, self.y1)
    }

    fn is_horizontal(&self) -> bool {
        return self.x0 == self.x1;
    }

    fn is_vertical(&self) -> bool {
        return self.y0 == self.y1;
    }

    fn points(&self) -> Result<Vec<(u16, u16)>, Box<dyn Error>> {
        let mut points: Vec<(u16, u16)> = Vec::new();
        if self.is_horizontal() {
            let x = self.x0;
            let range: std::ops::RangeInclusive<u16>;
            if self.y0 < self.y1 {
                range = self.y0..=self.y1;
            } else {
                range = self.y1..=self.y0;
            }
            for y in range {
                points.push((x, y));
            }
        } else if self.is_vertical() {
            let y = self.y0;
            let range: std::ops::RangeInclusive<u16>;
            if self.x0 < self.x1 {
                range = self.x0..=self.x1;
            } else {
                range = self.x1..=self.x0;
            }
            for x in range {
                points.push((x, y));
            }
        } else if (self.x1 as i32 - self.x0 as i32).abs() == (self.y1 as i32 - self.y0 as i32).abs()
        {
            let inc_x: i32 = if self.x1 > self.x0 { 1 } else { -1 };
            let inc_y: i32 = if self.y1 > self.y0 { 1 } else { -1 };
            let mut x: i32 = self.x0 as i32;
            let mut y: i32 = self.y0 as i32;
            while x != self.x1 as i32 {
                points.push((x as u16, y as u16));
                x += inc_x;
                y += inc_y;
            }
            points.push((x as u16, y as u16));
        } else {
            return Err(format!(
                "line {},{} -> {},{} is not horizontal, vertical or diagonal",
                self.x0, self.x1, self.y0, self.y1
            )
            .into());
        }
        Ok(points)
    }
}

struct Grid {
    lines: Vec<Line>,
}

impl Grid {
    fn get_intersections(
        &self,
        treshold: u16,
        include_diag: bool,
    ) -> Result<Vec<(u16, u16)>, Box<dyn Error>> {
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        for line in self.lines.iter() {
            rows = max(cols, line.max_y() as usize);
            cols = max(rows, line.max_x() as usize);
        }
        let mut intersections = vec![vec![0 as u16; cols + 1]; rows + 1];
        for line in self.lines.iter() {
            if include_diag || (line.is_horizontal() || line.is_vertical()) {
                for point in line.points()?.iter() {
                    intersections[point.0 as usize][point.1 as usize] += 1;
                }
            }
        }
        let mut points: Vec<(u16, u16)> = Vec::new();
        for x in 0..rows {
            for y in 0..cols {
                if intersections[x][y] >= treshold {
                    points.push((x as u16, y as u16));
                }
            }
        }
        Ok(points)
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Line> = Vec::new();
    for line in reader.lines() {
        lines.push(line?.parse()?);
    }
    let grid = Grid { lines: lines };
    Ok(format!("{}", grid.get_intersections(2, false)?.len()))
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Line> = Vec::new();
    for line in reader.lines() {
        lines.push(line?.parse()?);
    }
    let grid = Grid { lines: lines };
    Ok(format!("{}", grid.get_intersections(2, true)?.len()))
}
