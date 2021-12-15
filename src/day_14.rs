use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::error::Error;
use std::str::FromStr;

struct Polymer {
    first: char,
    last: char,
    chain_pairs: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

impl FromStr for Polymer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Polymer {
            chain_pairs: HashMap::new(),
            rules: HashMap::new(),
            first: Default::default(),
            last: Default::default(),
        };
        let mut lines = s.lines();
        match lines.next() {
            Some(l) => {
                let c: Vec<char> = l.chars().collect();
                p.first = *c.first().unwrap();
                p.last = *c.last().unwrap();
                for i in 0..(c.len() - 1) {
                    *p.chain_pairs.entry((c[i], c[i + 1])).or_default() += 1;
                }
            }
            None => return Err("invalid first line".into()),
        };
        lines.next(); // Advance iterator once more to skip blank line.
        let mut l: Option<&str>;
        while {
            l = lines.next();
            l.is_some()
        } {
            let parts = l.unwrap().split(" -> ").collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(format!("malformed line: {}", l.unwrap()).into());
            }
            let sub_parts = parts[0].chars().collect::<Vec<_>>();
            if sub_parts.len() != 2 || parts[1].chars().count() != 1 {
                return Err(format!("malformed line: {}", l.unwrap()).into());
            }
            p.rules.insert(
                (sub_parts[0], sub_parts[1]),
                parts[1].chars().next().unwrap(),
            );
        }
        Ok(p)
    }
}

impl Polymer {
    fn advance_step(&mut self) {
        let mut to_inc = Vec::new();
        let mut to_dec = Vec::new();
        for (&k, &v) in self.chain_pairs.iter() {
            if self.rules.contains_key(&k) {
                let m = self.rules[&k];
                to_inc.push(((k.0, m), v));
                to_inc.push(((m, k.1), v));
                to_dec.push((k, v));
            }
        }
        for &(v, n) in to_inc.iter() {
            *self.chain_pairs.entry(v).or_default() += n;
        }
        for &(v, n) in to_dec.iter() {
            *self.chain_pairs.entry(v).or_default() -= n;
        }
    }

    fn advance_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.advance_step();
        }
    }

    fn freq_count(&self) -> Vec<(usize, char)> {
        let mut map: HashMap<char, usize> = HashMap::new();
        for (&k, &v) in self.chain_pairs.iter() {
            *map.entry(k.0).or_default() += v;
            *map.entry(k.1).or_default() += v;
        }

        let mut heap = BinaryHeap::with_capacity(map.len());
        for (k, count) in map.into_iter() {
            let mut c = count / 2;
            if k == self.first || k == self.last {
                c += 1;
            }
            heap.push(Reverse((c, k)));
        }
        heap.into_sorted_vec().into_iter().map(|r| r.0).collect()
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut polymer: Polymer = std::fs::read_to_string(input)?.parse()?;
    polymer.advance_steps(10);
    let counts: BTreeSet<usize> = polymer.freq_count().iter().map(|&f| f.0).collect();
    Ok((counts.iter().next_back().unwrap() - counts.iter().next().unwrap()).to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut polymer: Polymer = std::fs::read_to_string(input)?.parse()?;
    polymer.advance_steps(40);
    let counts: BTreeSet<usize> = polymer.freq_count().iter().map(|&f| f.0).collect();
    Ok((counts.iter().next_back().unwrap() - counts.iter().next().unwrap()).to_string())
}
