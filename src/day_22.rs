use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct RectangularPrism {
    x_min: i64,
    y_min: i64,
    z_min: i64,
    x_max: i64,
    y_max: i64,
    z_max: i64,
}

impl Debug for RectangularPrism {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max
        )
    }
}

impl RectangularPrism {
    fn volume(&self) -> u64 {
        (self.x_max - self.x_min + 1) as u64
            * (self.y_max - self.y_min + 1) as u64
            * (self.z_max - self.z_min + 1) as u64
    }

    fn intersection(&self, other: &RectangularPrism) -> Option<RectangularPrism> {
        let r = RectangularPrism {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min.max(other.y_min),
            y_max: self.y_max.min(other.y_max),
            z_min: self.z_min.max(other.z_min),
            z_max: self.z_max.min(other.z_max),
        };
        if r.x_min <= r.x_max && r.y_min <= r.y_max && r.z_min <= r.z_max {
            return Some(r);
        }
        None
    }

    fn split(&self, other: &RectangularPrism) -> VecDeque<RectangularPrism> {
        let mut split = VecDeque::new();
        let intersec = self.intersection(other);
        match intersec {
            Some(inter) => {
                let (
                    r1_y_min,
                    r1_y_max,
                    r1_z_min,
                    r1_z_max,
                    r2_x_min,
                    r2_x_max,
                    r2_z_min,
                    r2_z_max,
                    r3_x_min,
                    r3_x_max,
                    r3_y_min,
                    r3_y_max,
                );

                r1_y_min = other.y_min;
                r1_y_max = other.y_max;
                r1_z_min = other.z_min;
                r1_z_max = other.z_max;
                if other.x_max > inter.x_max {
                    split.push_back(RectangularPrism {
                        x_min: inter.x_max + 1,
                        x_max: other.x_max,
                        y_min: r1_y_min,
                        y_max: r1_y_max,
                        z_min: r1_z_min,
                        z_max: r1_z_max,
                    })
                }
                if other.x_min < inter.x_min {
                    split.push_back(RectangularPrism {
                        x_min: other.x_min,
                        x_max: inter.x_min - 1,
                        y_min: r1_y_min,
                        y_max: r1_y_max,
                        z_min: r1_z_min,
                        z_max: r1_z_max,
                    })
                }

                r2_x_min = inter.x_min;
                r2_x_max = inter.x_max;
                r2_z_min = other.z_min;
                r2_z_max = other.z_max;
                if other.y_max > inter.y_max {
                    split.push_back(RectangularPrism {
                        x_min: r2_x_min,
                        x_max: r2_x_max,
                        y_min: inter.y_max + 1,
                        y_max: other.y_max,
                        z_min: r2_z_min,
                        z_max: r2_z_max,
                    })
                }
                if other.y_min < inter.y_min {
                    split.push_back(RectangularPrism {
                        x_min: r2_x_min,
                        x_max: r2_x_max,
                        y_min: other.y_min,
                        y_max: inter.y_min - 1,
                        z_min: r2_z_min,
                        z_max: r2_z_max,
                    })
                }

                r3_x_min = inter.x_min;
                r3_x_max = inter.x_max;
                r3_y_min = inter.y_min;
                r3_y_max = inter.y_max;
                if other.z_max > inter.z_max {
                    split.push_back(RectangularPrism {
                        x_min: r3_x_min,
                        x_max: r3_x_max,
                        y_min: r3_y_min,
                        y_max: r3_y_max,
                        z_min: inter.z_max + 1,
                        z_max: other.z_max,
                    })
                }
                if other.z_min < inter.z_min {
                    split.push_back(RectangularPrism {
                        x_min: r3_x_min,
                        x_max: r3_x_max,
                        y_min: r3_y_min,
                        y_max: r3_y_max,
                        z_min: other.z_min,
                        z_max: inter.z_min - 1,
                    })
                }
            }
            None => {
                split.push_back(*other);
            }
        }
        split
    }
}

#[derive(Default)]
struct ReactorGrid {
    volumes: VecDeque<RectangularPrism>,
}

impl ReactorGrid {
    fn apply_step(&mut self, s: &Step) {
        let mut new_volumes = VecDeque::new();
        for v in self.volumes.iter() {
            new_volumes.append(&mut s.volume.split(v));
        }
        if s.on {
            new_volumes.push_back(s.volume);
        }
        self.volumes = new_volumes;
    }

    fn num_on(&self) -> u64 {
        self.volumes.iter().map(|v| v.volume()).sum()
    }
}

#[derive(Default)]
struct Step {
    on: bool,
    volume: RectangularPrism,
}

impl FromStr for Step {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_invalid_input = Err(format!("malformed input: {}", s).into());
        let mut step: Step = Default::default();
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return err_invalid_input;
        }

        step.on = parts[0] == "on";

        let sub_parts = parts[1].split(",").collect::<Vec<_>>();
        if sub_parts.len() != 3 {
            return err_invalid_input;
        }

        let x_parts = sub_parts[0].split("=").collect::<Vec<_>>();
        let y_parts = sub_parts[1].split("=").collect::<Vec<_>>();
        let z_parts = sub_parts[2].split("=").collect::<Vec<_>>();
        if x_parts.len() != 2
            || y_parts.len() != 2
            || z_parts.len() != 2
            || x_parts[0] != "x"
            || y_parts[0] != "y"
            || z_parts[0] != "z"
        {
            return err_invalid_input;
        }
        let x_bounds = x_parts[1].split("..").collect::<Vec<_>>();
        let y_bounds = y_parts[1].split("..").collect::<Vec<_>>();
        let z_bounds = z_parts[1].split("..").collect::<Vec<_>>();
        if x_bounds.len() != 2 || y_bounds.len() != 2 || z_bounds.len() != 2 {
            return err_invalid_input;
        }
        step.volume.x_min = x_bounds[0].parse()?;
        step.volume.x_max = x_bounds[1].parse()?;
        step.volume.y_min = y_bounds[0].parse()?;
        step.volume.y_max = y_bounds[1].parse()?;
        step.volume.z_min = z_bounds[0].parse()?;
        step.volume.z_max = z_bounds[1].parse()?;

        Ok(step)
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut r: ReactorGrid = Default::default();
    let steps = std::fs::read_to_string(input)?;
    for s in steps.lines() {
        let step: Step = s.parse()?;
        if step.volume.x_min < -50
            || step.volume.y_min < -50
            || step.volume.z_min < -50
            || step.volume.x_max > 50
            || step.volume.y_max > 50
            || step.volume.z_max > 50
        {
            continue;
        }
        r.apply_step(&step);
    }
    Ok(r.num_on().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut r: ReactorGrid = Default::default();
    let steps = std::fs::read_to_string(input)?;
    for s in steps.lines() {
        r.apply_step(&s.parse()?);
    }
    Ok(r.num_on().to_string())
}
