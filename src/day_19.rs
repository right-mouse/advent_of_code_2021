use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::{self, Debug};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Rotation {
    None,
    Heading90,
    Heading180,
    Heading270,
    Altitude90,
    Altitude90Heading90,
    Altitude90Heading180,
    Altitude90Heading270,
    Altitude270,
    Altitude270Heading90,
    Altitude270Heading180,
    Altitude270Heading270,
    Bank90,
    Bank90Heading90,
    Bank90Heading180,
    Bank90Heading270,
    Bank180,
    Bank180Heading90,
    Bank180Heading180,
    Bank180Heading270,
    Bank270,
    Bank270Heading90,
    Bank270Heading180,
    Bank270Heading270,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct RotationMatrix([[i32; 3]; 3]);

impl RotationMatrix {
    // Reference: https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
    fn from_rot(r: Rotation) -> Self {
        let m;
        match r {
            Rotation::None => m = [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            Rotation::Heading90 => m = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
            Rotation::Heading180 => m = [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
            Rotation::Heading270 => m = [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
            Rotation::Altitude90 => m = [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            Rotation::Altitude90Heading90 => m = [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
            Rotation::Altitude90Heading180 => m = [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
            Rotation::Altitude90Heading270 => m = [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
            Rotation::Altitude270 => m = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            Rotation::Altitude270Heading90 => m = [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
            Rotation::Altitude270Heading180 => m = [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
            Rotation::Altitude270Heading270 => m = [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
            Rotation::Bank90 => m = [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
            Rotation::Bank90Heading90 => m = [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
            Rotation::Bank90Heading180 => m = [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
            Rotation::Bank90Heading270 => m = [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
            Rotation::Bank180 => m = [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
            Rotation::Bank180Heading90 => m = [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
            Rotation::Bank180Heading180 => m = [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
            Rotation::Bank180Heading270 => m = [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
            Rotation::Bank270 => m = [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
            Rotation::Bank270Heading90 => m = [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
            Rotation::Bank270Heading180 => m = [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
            Rotation::Bank270Heading270 => m = [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        }
        RotationMatrix(m)
    }
}

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Position {
    fn rotate(self, r: RotationMatrix) -> Self {
        Position {
            x: (r.0[0][0] * self.x) + (r.0[0][1] * self.y) + (r.0[0][2] * self.z),
            y: (r.0[1][0] * self.x) + (r.0[1][1] * self.y) + (r.0[1][2] * self.z),
            z: (r.0[2][0] * self.x) + (r.0[2][1] * self.y) + (r.0[2][2] * self.z),
        }
    }

    fn manhattan(self, rhs: Self) -> i32 {
        let d = self - rhs;
        d.x.abs() + d.y.abs() + d.z.abs()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Beacon {
    pos: Position,
}

impl FromStr for Beacon {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(",").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!("malformed input: {}", s).into());
        }
        Ok(Beacon {
            pos: Position {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
                z: parts[2].parse()?,
            },
        })
    }
}

#[derive(Clone, Debug)]
struct OverlapInfo {
    other_scanner_rot: Rotation,
    other_scanner_pos: Position,
    beacon_idx_mappings: HashMap<usize, usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    pos: Position,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn rotate(&mut self, r: Rotation) {
        let m = RotationMatrix::from_rot(r);
        for b in self.beacons.iter_mut() {
            b.pos = b.pos.rotate(m);
        }
    }

    fn set_pos(&mut self, p: Position) {
        self.pos = p;
        for b in self.beacons.iter_mut() {
            b.pos -= p;
        }
    }

    fn get_overlapping_beacons(&self, other: &Self) -> Option<OverlapInfo> {
        for r in [
            Rotation::None,
            Rotation::Heading90,
            Rotation::Heading180,
            Rotation::Heading270,
            Rotation::Altitude90,
            Rotation::Altitude90Heading90,
            Rotation::Altitude90Heading180,
            Rotation::Altitude90Heading270,
            Rotation::Altitude270,
            Rotation::Altitude270Heading90,
            Rotation::Altitude270Heading180,
            Rotation::Altitude270Heading270,
            Rotation::Bank90,
            Rotation::Bank90Heading90,
            Rotation::Bank90Heading180,
            Rotation::Bank90Heading270,
            Rotation::Bank180,
            Rotation::Bank180Heading90,
            Rotation::Bank180Heading180,
            Rotation::Bank180Heading270,
            Rotation::Bank270,
            Rotation::Bank270Heading90,
            Rotation::Bank270Heading180,
            Rotation::Bank270Heading270,
        ] {
            let m = RotationMatrix::from_rot(r);
            let mut differences = HashMap::new();
            for (i, a) in self.beacons.iter().enumerate() {
                for (j, b) in other.beacons.iter().enumerate() {
                    let difference = b.pos.rotate(m) - a.pos;
                    differences
                        .entry(difference)
                        .or_insert(HashSet::new())
                        .insert((i, j));
                }
            }
            match differences.iter().filter(|(_, v)| v.len() >= 12).next() {
                Some((p, v)) => {
                    let mut info = OverlapInfo {
                        other_scanner_rot: r,
                        other_scanner_pos: *p,
                        beacon_idx_mappings: HashMap::new(),
                    };
                    for &(k, v) in v.iter() {
                        info.beacon_idx_mappings.insert(k, v);
                    }
                    return Option::from(info);
                }
                None => (),
            }
        }
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Region {
    scanners: VecDeque<Scanner>,
}

impl FromStr for Region {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scanners = VecDeque::new();
        let mut lines = s.lines();
        let mut l;
        while {
            l = lines.next();
            l.is_some()
        } {
            let line = l.unwrap();
            if line.starts_with("---") {
                let mut beacons: Vec<Beacon> = Vec::new();
                while {
                    l = lines.next();
                    l.is_some()
                } {
                    let line = l.unwrap();
                    if line.is_empty() {
                        break;
                    }
                    beacons.push(line.parse()?);
                }
                scanners.push_back(Scanner {
                    pos: Default::default(),
                    beacons: beacons,
                });
            }
        }
        Ok(Region { scanners: scanners })
    }
}

impl Region {
    fn map_scanners(&mut self) {
        if self.scanners.len() <= 1 {
            return;
        }
        let mut todo: HashSet<usize> = (1..self.scanners.len()).collect();
        let mut done: HashSet<usize> = [0].into_iter().collect();
        let mut last_len = todo.len();
        while todo.len() > 0 {
            'outer: for &i in done.iter() {
                for &j in todo.iter() {
                    let overlap = self.scanners[i].get_overlapping_beacons(&self.scanners[j]);
                    if overlap.is_some() {
                        todo.remove(&j);
                        done.insert(j);
                        let o = overlap.unwrap();
                        self.scanners[j].rotate(o.other_scanner_rot);
                        self.scanners[j].set_pos(o.other_scanner_pos);
                        break 'outer;
                    }
                }
            }
            if todo.len() == last_len {
                panic!("no beacon could be mapped")
            }
            last_len = todo.len();
        }
    }

    fn get_all_beacons(&self) -> HashSet<Position> {
        let mut beacons: HashSet<Position> =
            HashSet::with_capacity(self.scanners.iter().map(|s| s.beacons.len()).sum());
        for s in self.scanners.iter() {
            for b in s.beacons.iter() {
                beacons.insert(b.pos);
            }
        }
        beacons
    }

    fn get_largest_manhattan_distance(&self) -> i32 {
        let mut largest_dist = 0;
        for i in 0..self.scanners.len() {
            for j in i..self.scanners.len() {
                if i != j {
                    largest_dist =
                        largest_dist.max(self.scanners[i].pos.manhattan(self.scanners[j].pos));
                }
            }
        }
        largest_dist
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut region: Region = std::fs::read_to_string(input)?.parse()?;
    region.map_scanners();
    Ok(region.get_all_beacons().len().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut region: Region = std::fs::read_to_string(input)?.parse()?;
    region.map_scanners();
    Ok(region.get_largest_manhattan_distance().to_string())
}
