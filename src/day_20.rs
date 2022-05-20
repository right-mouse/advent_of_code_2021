use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq)]
struct ImageEnhancementAlgorithm {
    alg: Vec<bool>,
}

impl FromStr for ImageEnhancementAlgorithm {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ImageEnhancementAlgorithm {
            alg: s
                .chars()
                .map(|c| match c {
                    '#' => true,
                    _ => false,
                })
                .collect(),
        })
    }
}

impl ImageEnhancementAlgorithm {
    fn get(&self, p: [bool; 9]) -> bool {
        let mut idx = 0;
        for (i, &v) in p.iter().rev().enumerate() {
            if v {
                idx |= 1 << i;
            }
        }
        self.alg[idx]
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Image {
    background_px: bool,
    buf: Vec<Vec<bool>>,
}

impl FromStr for Image {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = Image {
            background_px: false,
            buf: Vec::new(),
        };
        for l in s.lines() {
            i.buf.push(
                l.chars()
                    .map(|c| match c {
                        '#' => true,
                        _ => false,
                    })
                    .collect(),
            );
        }
        Ok(i)
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bkgd_char = if self.background_px { '#' } else { '.' }.to_string();
        let len = self.buf.len();
        for _ in 0..3 {
            writeln!(f, "{}", bkgd_char.repeat(len + 6))?;
        }
        for l in self.buf.iter() {
            write!(f, "{}", bkgd_char.repeat(3))?;
            write!(
                f,
                "{}",
                l.iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            )?;
            writeln!(f, "{}", bkgd_char.repeat(3))?;
        }
        for _ in 0..2 {
            writeln!(f, "{}", bkgd_char.repeat(len + 6))?;
        }
        write!(f, "{}", bkgd_char.repeat(len + 6))
    }
}

impl Image {
    fn get_lit_pixels(&self) -> usize {
        self.buf
            .iter()
            .map(|l| l.iter().filter(|&v| *v).count())
            .sum()
    }

    fn trim(&mut self) {
        let old_len = self.buf.len();

        let mut rows_to_remove = VecDeque::new();
        for (i, l) in self.buf.iter().enumerate() {
            if l.iter()
                .map(|&b| if b != self.background_px { 1 } else { 0 })
                .sum::<u8>()
                == 0
            {
                rows_to_remove.push_back(i);
            }
        }
        let mut cols_to_remove = VecDeque::new();
        for j in 0..old_len {
            let mut remove = true;
            for i in 0..self.buf.len() {
                if self.buf[i][j] != self.background_px {
                    remove = false;
                    break;
                }
            }
            if remove {
                cols_to_remove.push_back(j);
            }
        }

        // Reduce the cols to remove list to match rows to remove to keep the image a square, if
        // necessary.
        let mut alt = 0;
        while cols_to_remove.len() > rows_to_remove.len() {
            if alt % 2 == 0 {
                cols_to_remove.pop_front();
            } else {
                cols_to_remove.pop_back();
            }
            alt += 1;
        }
        // Reduce the rows to remove list to match cols to remove to keep the image a square, if
        // necessary.
        let mut alt = 0;
        while rows_to_remove.len() > cols_to_remove.len() {
            if alt % 2 == 0 {
                rows_to_remove.pop_front();
            } else {
                rows_to_remove.pop_back();
            }
            alt += 1;
        }

        // Remove rows.
        let mut num_removed = 0;
        for &i in rows_to_remove.iter() {
            self.buf.remove(i - num_removed);
            num_removed += 1;
        }

        // Remove cols.
        let mut num_removed = 0;
        for &j in cols_to_remove.iter() {
            for i in 0..self.buf.len() {
                self.buf[i].remove(j - num_removed);
            }
            num_removed += 1;
        }
    }

    fn enhance(&mut self, alg: &ImageEnhancementAlgorithm) {
        let old_len = self.buf.len();
        let new_len = old_len + 6;
        let new_background_px = alg.get([self.background_px; 9]);
        let mut new_img = Vec::with_capacity(new_len);
        for _ in 0..new_len {
            new_img.push(vec![false; new_len]);
        }

        for x in 0..new_len {
            for y in 0..new_len {
                let mut v = [self.background_px; 9];
                let mut k = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        let ix = x as i32 + i - 3;
                        let jy = y as i32 + j - 3;
                        if ix >= 0 && jy >= 0 && (ix as usize) < old_len && (jy as usize) < old_len
                        {
                            v[k] = self.buf[ix as usize][jy as usize];
                        }
                        k += 1;
                    }
                }
                new_img[x][y] = alg.get(v);
            }
        }

        self.buf = new_img;
        self.background_px = new_background_px;
        self.trim();
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string(input)?;
    let mut lines = file.lines();
    let alg: ImageEnhancementAlgorithm = lines.next().unwrap().parse()?;
    lines.next(); // Skip empty line.
    let mut img: Image = lines.collect::<Vec<_>>().join("\n").parse()?;
    img.enhance(&alg);
    img.enhance(&alg);
    Ok(img.get_lit_pixels().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string(input)?;
    let mut lines = file.lines();
    let alg: ImageEnhancementAlgorithm = lines.next().unwrap().parse()?;
    lines.next(); // Skip empty line.
    let mut img: Image = lines.collect::<Vec<_>>().join("\n").parse()?;
    for _ in 0..50 {
        img.enhance(&alg);
    }
    Ok(img.get_lit_pixels().to_string())
}
