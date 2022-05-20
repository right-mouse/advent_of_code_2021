use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ExpandDir {
    All,
    AllButLeft,
    AllButRight,
    AllButUp,
    AllButDown,
}

struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl FromStr for HeightMap {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();
        for l in s.lines() {
            let mut h = Vec::new();
            for c in l.chars() {
                h.push(c.to_string().parse()?);
            }
            heights.push(h);
        }
        Ok(HeightMap { heights: heights })
    }
}

fn is_lowest_point(h: &Vec<Vec<u8>>, i: i32, j: i32) -> bool {
    let v = h[i as usize][j as usize];
    let ix_1 = i - 1;
    let ix_2 = i + 1;
    let jy_1 = j - 1;
    let jy_2 = j + 1;
    if ix_1 >= 0 {
        if h[ix_1 as usize][j as usize] <= v {
            return false;
        }
    }
    if ix_2 < h.len() as i32 {
        if h[ix_2 as usize][j as usize] <= v {
            return false;
        }
    }
    if jy_1 >= 0 {
        if h[i as usize][jy_1 as usize] <= v {
            return false;
        }
    }
    if jy_2 < h[i as usize].len() as i32 {
        if h[i as usize][jy_2 as usize] <= v {
            return false;
        }
    }
    true
}

fn expand_basin(b: &mut HashSet<(usize, usize)>, h: &Vec<Vec<u8>>, i: i32, j: i32, d: ExpandDir) {
    let ix_1 = i - 1;
    let ix_2 = i + 1;
    let jy_1 = j - 1;
    let jy_2 = j + 1;
    if d != ExpandDir::AllButUp && ix_1 >= 0 {
        if h[ix_1 as usize][j as usize] != 9 {
            if !b.contains(&(ix_1 as usize, j as usize)) {
                b.insert((ix_1 as usize, j as usize));
                expand_basin(b, h, ix_1, j, ExpandDir::AllButDown);
            }
        }
    }
    if d != ExpandDir::AllButDown && ix_2 < h.len() as i32 {
        if h[ix_2 as usize][j as usize] != 9 {
            if !b.contains(&(ix_2 as usize, j as usize)) {
                b.insert((ix_2 as usize, j as usize));
                expand_basin(b, h, ix_2, j, ExpandDir::AllButUp);
            }
        }
    }
    if d != ExpandDir::AllButLeft && jy_1 >= 0 {
        if h[i as usize][jy_1 as usize] != 9 {
            if !b.contains(&(i as usize, jy_1 as usize)) {
                b.insert((i as usize, jy_1 as usize));
                expand_basin(b, h, i, jy_1, ExpandDir::AllButRight);
            }
        }
    }
    if d != ExpandDir::AllButRight && jy_2 < h[i as usize].len() as i32 {
        if h[i as usize][jy_2 as usize] != 9 {
            if !b.contains(&(i as usize, jy_2 as usize)) {
                b.insert((i as usize, jy_2 as usize));
                expand_basin(b, h, i, jy_2, ExpandDir::AllButLeft);
            }
        }
    }
}

impl HeightMap {
    fn low_points(&self) -> Vec<(u8, usize, usize)> {
        let mut pts = Vec::new();
        for i in 0..self.heights.len() {
            for j in 0..self.heights[i].len() {
                if is_lowest_point(&self.heights, i as i32, j as i32) {
                    pts.push((self.heights[i][j], i, j));
                }
            }
        }
        pts
    }

    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        let low_points = self.low_points();
        let mut basins = Vec::with_capacity(low_points.len());
        for &p in low_points.iter() {
            let mut b = HashSet::new();
            b.insert((p.1, p.2));
            expand_basin(
                &mut b,
                &self.heights,
                p.1 as i32,
                p.2 as i32,
                ExpandDir::All,
            );
            basins.push(b.clone());
        }
        basins
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let height_map = std::fs::read_to_string(input)?.parse::<HeightMap>()?;
    let risk_score: u32 = height_map.low_points().iter().map(|v| v.0 as u32 + 1).sum();
    Ok(risk_score.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let height_map = std::fs::read_to_string(input)?.parse::<HeightMap>()?;
    let mut basin_lens: Vec<usize> = height_map.basins().iter().map(|b| b.len()).collect();
    basin_lens.sort();
    let product: usize = basin_lens.iter().rev().take(3).product();
    Ok(product.to_string())
}
