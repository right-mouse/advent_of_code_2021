use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Eq, Debug)]
struct Node {
    path: Vec<(usize, usize)>,
    end: (usize, usize),
    g: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.pos() == other.pos()
    }

    fn ne(&self, other: &Node) -> bool {
        self.pos() != other.pos()
    }
}

impl Node {
    fn pos(&self) -> (usize, usize) {
        self.path[self.path.len() - 1]
    }

    fn h(&self) -> usize {
        let pos = self.pos();
        (self.end.0 - pos.0) + (self.end.1 - pos.1)
    }

    fn f(&self) -> usize {
        self.g + self.h()
    }
}

struct Cavern {
    risk_levels: Vec<Vec<u8>>,
}

impl FromStr for Cavern {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cavern {
            risk_levels: Vec::new(),
        };
        for l in s.lines() {
            c.risk_levels
                .push(l.chars().map(|c| c.to_string().parse().unwrap()).collect());
        }
        Ok(c)
    }
}

impl Cavern {
    fn expand_map_to_full(&mut self) {
        let orig_len = self.risk_levels.len();
        for i in 0..orig_len {
            self.risk_levels[i] = self.risk_levels[i].repeat(5);
        }
        self.risk_levels = self
            .risk_levels
            .clone()
            .into_iter()
            .cycle()
            .take(orig_len * 5)
            .collect();
        for i in 0..self.risk_levels.len() {
            for j in 0..self.risk_levels[i].len() {
                self.risk_levels[i][j] += (i / orig_len) as u8 + (j / orig_len) as u8;
                while self.risk_levels[i][j] > 9 {
                    self.risk_levels[i][j] -= 9;
                }
            }
        }
    }

    fn astar(&self, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
        let mut open_list = VecDeque::new();
        let mut closed_list = VecDeque::new();
        open_list.push_back(Node {
            path: vec![start],
            end: end,
            g: 0,
        });

        while open_list.len() > 0 {
            let idx = open_list
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.f().cmp(&b.f()))
                .map(|(index, _)| index)
                .unwrap();
            // Move node from open list to closed list.
            let node = open_list.remove(idx).unwrap();
            closed_list.push_back(node.clone());
            if node.pos() == end {
                return (node.path.clone(), node.g);
            }
            // Generate children.
            let mut children = Vec::new();
            for dir in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
                let ix = node.pos().0 as i32 + dir.0;
                let iy = node.pos().1 as i32 + dir.1;
                if ix < 0
                    || iy < 0
                    || ix >= (self.risk_levels.len() as i32)
                    || iy >= (self.risk_levels.len() as i32)
                {
                    continue;
                }
                let mut p = node.path.clone();
                p.push((ix as usize, iy as usize));
                children.push(Node {
                    path: p,
                    end: end,
                    g: node.g + self.risk_levels[ix as usize][iy as usize] as usize,
                });
            }
            // Loop through children.
            for child in children.iter() {
                if closed_list.contains(child) {
                    continue;
                }
                let mut append_child = true;
                let mut remove_idx: i32 = -1;
                for (i, o) in open_list.iter().enumerate() {
                    if *child == *o {
                        if child.g >= o.g {
                            append_child = false;
                        } else {
                            remove_idx = i as i32;
                        }
                        break;
                    }
                }
                if remove_idx != -1 {
                    open_list.remove(remove_idx as usize);
                }
                if append_child {
                    println!("{:?} - {}", child.pos(), child.h());
                    open_list.push_back(child.clone());
                }
            }
        }

        // Should never get here.
        return (Vec::new(), 0);
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let cavern: Cavern = std::fs::read_to_string(input)?.parse()?;
    let path = cavern.astar(
        (0, 0),
        (
            cavern.risk_levels.len() - 1,
            cavern.risk_levels[0].len() - 1,
        ),
    );
    Ok(path.1.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut cavern: Cavern = std::fs::read_to_string(input)?.parse()?;
    cavern.expand_map_to_full();
    let path = cavern.astar(
        (0, 0),
        (
            cavern.risk_levels.len() - 1,
            cavern.risk_levels[0].len() - 1,
        ),
    );
    Ok(path.1.to_string())
}
