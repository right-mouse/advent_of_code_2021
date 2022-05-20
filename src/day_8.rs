use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Default for Segment {
    fn default() -> Self {
        Segment::A
    }
}

impl FromStr for Segment {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("malformed segment: {}", s).into());
        }
        return Ok(match s.chars().next().unwrap() {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => return Err(format!("invalid segment: {}", s).into()),
        });
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Segments(HashSet<Segment>);

impl FromStr for Segments {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m: HashSet<Segment> = HashSet::new();
        for c in s.chars() {
            m.insert(c.to_string().parse()?);
        }
        Ok(Segments(m))
    }
}

impl fmt::Display for Segments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str;
        let set = &self.0;
        if set.len() == 7 {
            s = "8";
        } else if set.len() == 2 && set.contains(&Segment::C) && set.contains(&Segment::F) {
            s = "1";
        } else if set.len() == 3
            && set.contains(&Segment::A)
            && set.contains(&Segment::C)
            && set.contains(&Segment::F)
        {
            s = "7";
        } else if set.len() == 4
            && set.contains(&Segment::B)
            && set.contains(&Segment::C)
            && set.contains(&Segment::D)
            && set.contains(&Segment::F)
        {
            s = "4";
        } else if set.len() == 5
            && set.contains(&Segment::A)
            && set.contains(&Segment::C)
            && set.contains(&Segment::D)
            && set.contains(&Segment::E)
            && set.contains(&Segment::G)
        {
            s = "2";
        } else if set.len() == 5
            && set.contains(&Segment::A)
            && set.contains(&Segment::C)
            && set.contains(&Segment::D)
            && set.contains(&Segment::F)
            && set.contains(&Segment::G)
        {
            s = "3";
        } else if set.len() == 5
            && set.contains(&Segment::A)
            && set.contains(&Segment::B)
            && set.contains(&Segment::D)
            && set.contains(&Segment::F)
            && set.contains(&Segment::G)
        {
            s = "5";
        } else if set.len() == 6
            && set.contains(&Segment::A)
            && set.contains(&Segment::B)
            && set.contains(&Segment::C)
            && set.contains(&Segment::E)
            && set.contains(&Segment::F)
            && set.contains(&Segment::G)
        {
            s = "0";
        } else if set.len() == 6
            && set.contains(&Segment::A)
            && set.contains(&Segment::B)
            && set.contains(&Segment::D)
            && set.contains(&Segment::E)
            && set.contains(&Segment::F)
            && set.contains(&Segment::G)
        {
            s = "6";
        } else if set.len() == 6
            && set.contains(&Segment::A)
            && set.contains(&Segment::B)
            && set.contains(&Segment::C)
            && set.contains(&Segment::D)
            && set.contains(&Segment::F)
            && set.contains(&Segment::G)
        {
            s = "9";
        } else {
            s = "invalid";
        }
        write!(f, "{}", s)
    }
}

impl Segments {
    fn is_one(&self) -> bool {
        if self.0.len() == 2 {
            return true;
        }
        false
    }

    fn is_four(&self) -> bool {
        if self.0.len() == 4 {
            return true;
        }
        false
    }

    fn is_seven(&self) -> bool {
        if self.0.len() == 3 {
            return true;
        }
        false
    }

    fn is_eight(&self) -> bool {
        if self.0.len() == 7 {
            return true;
        }
        false
    }

    fn replace(&self, mappings: HashMap<Segment, Segment>) -> Segments {
        let mut s = HashSet::with_capacity(self.0.len());
        for v in self.0.iter() {
            s.insert(mappings[v]);
        }
        Segments(s)
    }
}

#[derive(Debug, Default)]
struct SevenSegmentDisplay {
    signal_patterns: [Segments; 10],
    output_values: [Segments; 4],
}

impl FromStr for SevenSegmentDisplay {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("|").map(|n| n.trim()).collect();
        if parts.len() != 2 {
            return Err(format!("malformed input: {}", s).into());
        }
        let signal_patterns: Vec<&str> = parts[0].split(" ").map(|n| n.trim()).collect();
        if signal_patterns.len() != 10 {
            return Err(format!(
                "expected 10 signal patterns but got {}",
                signal_patterns.len()
            )
            .into());
        }
        let output_values: Vec<&str> = parts[1].split(" ").map(|n| n.trim()).collect();
        if output_values.len() != 4 {
            return Err(format!("expected 4 output values but got {}", output_values.len()).into());
        }
        let mut ssd = SevenSegmentDisplay {
            signal_patterns: Default::default(),
            output_values: Default::default(),
        };
        for (i, sp) in signal_patterns.into_iter().enumerate() {
            ssd.signal_patterns[i] = sp.to_string().parse()?;
        }
        for (i, ov) in output_values.into_iter().enumerate() {
            ssd.output_values[i] = ov.to_string().parse()?;
        }
        Ok(ssd)
    }
}

impl SevenSegmentDisplay {
    fn unique_vals(&self) -> Vec<Segments> {
        self.output_values
            .iter()
            .filter(|v| v.is_one() || v.is_four() || v.is_seven() || v.is_eight())
            .map(|v| v.clone())
            .collect()
    }

    fn create_pattern_mappings(&self) -> HashMap<Segment, Segment> {
        let mut mappings: HashMap<Segment, Segment> = HashMap::new();
        // Initialize groupings.
        let one = self
            .signal_patterns
            .iter()
            .filter(|p| p.is_one())
            .next()
            .unwrap();
        let four = self
            .signal_patterns
            .iter()
            .filter(|p| p.is_four())
            .next()
            .unwrap();
        let seven = self
            .signal_patterns
            .iter()
            .filter(|p| p.is_seven())
            .next()
            .unwrap();
        let eight = self
            .signal_patterns
            .iter()
            .filter(|p| p.is_eight())
            .next()
            .unwrap();
        let five_segment_values: Vec<HashSet<Segment>> = self
            .signal_patterns
            .iter()
            .map(|p| p.0.clone())
            .filter(|p| p.len() == 5)
            .collect();
        let six_segment_values: Vec<HashSet<Segment>> = self
            .signal_patterns
            .iter()
            .map(|p| p.0.clone())
            .filter(|p| p.len() == 6)
            .collect();
        // One has c and f; seven has a, c and f - find a.
        let mut set_a = seven.0.clone();
        for p in one.0.iter() {
            set_a.remove(p);
        }
        let mapping_a = *set_a.iter().next().unwrap();
        mappings.insert(mapping_a, Segment::A);
        // The two values in four that aren't in one are b and d.
        let mut set_b_and_d = four.0.clone();
        for p in one.0.iter() {
            set_b_and_d.remove(p);
        }
        // The six segment values have b in all 3 of them and d in two of them.
        let mut mapping_b: Segment = Default::default();
        let mut mapping_d: Segment = Default::default();
        for p in set_b_and_d.iter() {
            let count = six_segment_values.iter().filter(|v| v.contains(p)).count();
            if count == 2 {
                mapping_d = *p;
                mappings.insert(mapping_d, Segment::D);
            } else if count == 3 {
                mapping_b = *p;
                mappings.insert(mapping_b, Segment::B);
            }
        }
        // The five segment values all have a, d and g.
        let mut set_five_segment_values: HashSet<Segment> = HashSet::new();
        for fs in five_segment_values.iter() {
            set_five_segment_values.extend(fs.clone());
        }
        set_five_segment_values.remove(&mapping_a);
        set_five_segment_values.remove(&mapping_d);
        let mut mapping_g: Segment = Default::default();
        for p in set_five_segment_values.iter() {
            if five_segment_values.iter().filter(|v| v.contains(p)).count() == 3 {
                mapping_g = *p;
                mappings.insert(mapping_g, Segment::G);
                break;
            }
        }
        // The six segment values all have a, b, f and g.
        let mut set_six_segment_values: HashSet<Segment> = HashSet::new();
        for ss in six_segment_values.iter() {
            set_six_segment_values.extend(ss.clone());
        }
        set_six_segment_values.remove(&mapping_a);
        set_six_segment_values.remove(&mapping_b);
        set_six_segment_values.remove(&mapping_d);
        set_six_segment_values.remove(&mapping_g);
        let mut mapping_f: Segment = Default::default();
        for p in set_six_segment_values.iter() {
            let count = six_segment_values.iter().filter(|v| v.contains(p)).count();
            if count == 3 {
                mapping_f = *p;
                mappings.insert(mapping_f, Segment::F);
            }
        }
        // Use one to get c.
        let mut set_c = one.0.clone();
        set_c.remove(&mapping_f);
        let mapping_c = *set_c.iter().next().unwrap();
        mappings.insert(mapping_c, Segment::C);
        // Use eight to get e.
        let mut set_e = eight.0.clone();
        set_e.remove(&mapping_a);
        set_e.remove(&mapping_b);
        set_e.remove(&mapping_c);
        set_e.remove(&mapping_d);
        set_e.remove(&mapping_f);
        set_e.remove(&mapping_g);
        let mapping_e = *set_e.iter().next().unwrap();
        mappings.insert(mapping_e, Segment::E);
        // Mappings should be complete now.
        mappings
    }

    fn get_output(&self) -> Result<u32, std::num::ParseIntError> {
        let m = self.create_pattern_mappings();
        let mut val = String::new();
        for s in self.output_values.iter() {
            let sr = s.replace(m.clone());
            val = format!("{}{}", val, sr);
        }
        val.parse()
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut ssd: Vec<SevenSegmentDisplay> = Vec::new();
    for line in reader.lines() {
        ssd.push(line?.parse()?);
    }
    let num_unique: usize = ssd.into_iter().map(|v| v.unique_vals().len()).sum();
    Ok(num_unique.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut ssd: Vec<SevenSegmentDisplay> = Vec::new();
    for line in reader.lines() {
        let s: SevenSegmentDisplay = line?.parse()?;
        ssd.push(s);
    }
    let mut sum: u32 = 0;
    for i in ssd.into_iter().map(|v| v.get_output()) {
        sum += i?;
    }
    Ok(sum.to_string())
}
