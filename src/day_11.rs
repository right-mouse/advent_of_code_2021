use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

const GRID_SIDE_LEN: usize = 10;

#[derive(Debug)]
struct OctopusGrid([[u8; GRID_SIDE_LEN]; GRID_SIDE_LEN]);

impl FromStr for OctopusGrid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != GRID_SIDE_LEN {
            return Err(format!("expected {} lines but got {}", GRID_SIDE_LEN, lines.len()).into());
        }
        let mut grid = [[0; GRID_SIDE_LEN]; GRID_SIDE_LEN];
        for (i, &l) in lines.iter().enumerate() {
            if l.len() != GRID_SIDE_LEN {
                return Err(format!(
                    "expected {} characters in line {} but got {}",
                    GRID_SIDE_LEN,
                    i,
                    lines.len()
                )
                .into());
            }
            for (j, c) in l.chars().enumerate() {
                grid[i][j] = c.to_string().parse()?;
            }
        }
        Ok(OctopusGrid(grid))
    }
}

impl OctopusGrid {
    fn advance_step(&mut self) -> usize {
        // Increase energy levels by 1.
        let mut flashes: Vec<(usize, usize)> = Vec::new();
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                self.0[x][y] += 1;
                if self.0[x][y] > 9 {
                    flashes.push((x, y));
                }
            }
        }
        // Keep repeating until no more octopuses flash.
        let mut already_flashed: HashSet<(usize, usize)> = HashSet::new();
        while flashes.len() > 0 {
            // Flash and increase adjacent energy levels by 1.
            for &f in flashes.iter() {
                let i = f.0;
                let j = f.1;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let ix = i as i32 + x;
                        let iy = j as i32 + y;
                        if ix < 0 || ix >= GRID_SIDE_LEN as i32 {
                            continue;
                        }
                        if iy < 0 || iy >= GRID_SIDE_LEN as i32 {
                            continue;
                        }
                        self.0[ix as usize][iy as usize] += 1;
                    }
                }
                already_flashed.insert((i, j));
            }

            // Check for new flashes.
            flashes.clear();
            for x in 0..self.0.len() {
                for y in 0..self.0[x].len() {
                    if self.0[x][y] > 9 && !already_flashed.contains(&(x, y)) {
                        flashes.push((x, y));
                    }
                }
            }
        }

        // Reset the flashed octopuses.
        for &(x, y) in already_flashed.iter() {
            self.0[x][y] = 0;
        }
        already_flashed.len()
    }

    fn advance_steps(&mut self, n: usize) -> usize {
        let mut total_flashes = 0;
        for _ in 0..n {
            total_flashes += self.advance_step();
        }
        total_flashes
    }

    fn get_simultaneous_flash_step(&mut self) -> usize {
        let mut i = 0;
        loop {
            i += 1;
            if self.advance_step() == GRID_SIDE_LEN * GRID_SIDE_LEN {
                return i;
            }
        }
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut grid: OctopusGrid = std::fs::read_to_string(input)?.parse()?;
    Ok(grid.advance_steps(100).to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut grid: OctopusGrid = std::fs::read_to_string(input)?.parse()?;
    Ok(grid.get_simultaneous_flash_step().to_string())
}
