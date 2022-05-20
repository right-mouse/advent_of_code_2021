use std::any::Any;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Debug};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SnailfishTraversePath {
    LEFT,
    RIGHT,
}

struct SnailfishNumberPtr<'a> {
    val: &'a mut SnailfishNumber,
    path: VecDeque<SnailfishTraversePath>,
    depth: u8,
}

trait SnailfishValue: Debug {
    fn is_number(&self) -> bool;
    fn magnitude(&self) -> u64;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_numbers(&mut self) -> Vec<SnailfishNumberPtr>;
}

impl Clone for Box<dyn SnailfishValue> {
    fn clone(&self) -> Self {
        let val = self.as_any();
        if self.is_number() {
            return Box::new(val.downcast_ref::<SnailfishNumber>().unwrap().clone());
        } else {
            let pair = val.downcast_ref::<SnailfishPair>().unwrap();
            return Box::new(SnailfishPair {
                left: pair.left.clone(),
                right: pair.right.clone(),
            });
        }
    }
}

impl Add for Box<dyn SnailfishValue> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut p = SnailfishPair {
            left: self.clone(),
            right: rhs,
        };
        p.reduce();
        Box::new(p)
    }
}

impl AddAssign for Box<dyn SnailfishValue> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

#[derive(Copy, Clone, Default)]
struct SnailfishNumber {
    num: u8,
}

impl SnailfishValue for SnailfishNumber {
    fn is_number(&self) -> bool {
        true
    }

    fn magnitude(&self) -> u64 {
        self.num as u64
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_numbers(&mut self) -> Vec<SnailfishNumberPtr> {
        vec![SnailfishNumberPtr {
            val: self,
            path: VecDeque::new(),
            depth: 0,
        }]
    }
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[derive(Clone)]
struct SnailfishPair {
    left: Box<dyn SnailfishValue>,
    right: Box<dyn SnailfishValue>,
}

impl SnailfishValue for SnailfishPair {
    fn is_number(&self) -> bool {
        false
    }

    fn magnitude(&self) -> u64 {
        (3 * self.left.magnitude()) + (2 * self.right.magnitude())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_numbers(&mut self) -> Vec<SnailfishNumberPtr> {
        let mut v1 = self.left.get_numbers();
        for i in 0..v1.len() {
            v1[i].path.push_front(SnailfishTraversePath::LEFT);
            v1[i].depth += 1;
        }
        let mut v2 = self.right.get_numbers();
        for i in 0..v2.len() {
            v2[i].path.push_front(SnailfishTraversePath::RIGHT);
            v2[i].depth += 1;
        }
        v1.append(&mut v2);
        v1
    }
}

impl FromStr for SnailfishPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_invalid_input = Err("invalid input".into());
        let left: Box<dyn SnailfishValue>;
        let right: Box<dyn SnailfishValue>;
        let mut chars = s.chars();
        if chars.next().unwrap() != '[' {
            return err_invalid_input;
        }
        let c = chars.next().unwrap();
        if c != '[' {
            let v = c.to_string().parse::<u8>()?;
            left = Box::new(SnailfishNumber { num: v });
        } else {
            let mut bracket_count = 1;
            let mut substr = c.to_string();
            while bracket_count > 0 {
                let c = chars.next().unwrap();
                substr.push(c);
                match c {
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => (),
                }
            }
            left = Box::new(substr.parse::<SnailfishPair>()?);
        }
        if chars.next().unwrap() != ',' {
            return err_invalid_input;
        }
        let c = chars.next().unwrap();
        if c != '[' {
            let v = c.to_string().parse::<u8>()?;
            right = Box::new(SnailfishNumber { num: v });
        } else {
            let mut bracket_count = 1;
            let mut substr = c.to_string();
            while bracket_count > 0 {
                let c = chars.next().unwrap();
                substr.push(c);
                match c {
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => (),
                }
            }
            right = Box::new(substr.parse::<SnailfishPair>()?);
        }
        Ok(SnailfishPair {
            left: left,
            right: right,
        })
    }
}

impl Debug for SnailfishPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

impl SnailfishPair {
    fn get(&mut self, path: &VecDeque<SnailfishTraversePath>) -> &mut dyn SnailfishValue {
        let mut v: &mut dyn SnailfishValue = self;
        for p in path.iter() {
            match p {
                SnailfishTraversePath::LEFT => {
                    v = &mut *v.as_any_mut().downcast_mut::<SnailfishPair>().unwrap().left
                }
                SnailfishTraversePath::RIGHT => {
                    v = &mut *v
                        .as_any_mut()
                        .downcast_mut::<SnailfishPair>()
                        .unwrap()
                        .right
                }
            }
        }
        v
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        let mut nums = self.get_numbers();
        for i in 0..(nums.len() - 1) {
            if nums[i].depth > 4 {
                let mut p1 = nums[i].path.clone();
                p1.pop_back(); // Remove last element to get parent.
                let mut p2 = nums[i + 1].path.clone();
                p2.pop_back(); // Remove last element to get parent.
                if p1 == p2 {
                    if i > 0 {
                        nums[i - 1].val.num += nums[i].val.num;
                    }
                    if i + 2 < nums.len() {
                        nums[i + 2].val.num += nums[i + 1].val.num;
                    }
                    let pos = p1.pop_back().unwrap();
                    let parent = self
                        .get(&p1)
                        .as_any_mut()
                        .downcast_mut::<SnailfishPair>()
                        .unwrap();
                    match pos {
                        SnailfishTraversePath::LEFT => {
                            parent.left = Box::new(SnailfishNumber { num: 0 });
                        }
                        SnailfishTraversePath::RIGHT => {
                            parent.right = Box::new(SnailfishNumber { num: 0 });
                        }
                    }
                    return true;
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        let nums = self.get_numbers();
        for n in nums.iter() {
            if n.val.num > 9 {
                let left = n.val.num / 2;
                let right = n.val.num - left;
                let mut p = n.path.clone();
                let pos = p.pop_back().unwrap();
                let parent = self
                    .get(&p)
                    .as_any_mut()
                    .downcast_mut::<SnailfishPair>()
                    .unwrap();
                match pos {
                    SnailfishTraversePath::LEFT => {
                        parent.left = Box::new(SnailfishPair {
                            left: Box::new(SnailfishNumber { num: left }),
                            right: Box::new(SnailfishNumber { num: right }),
                        });
                    }
                    SnailfishTraversePath::RIGHT => {
                        parent.right = Box::new(SnailfishPair {
                            left: Box::new(SnailfishNumber { num: left }),
                            right: Box::new(SnailfishNumber { num: right }),
                        });
                    }
                }
                return true;
            }
        }
        false
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let list = std::fs::read_to_string(input)?;
    let mut lines = list.lines();
    let mut sum: Box<dyn SnailfishValue> =
        Box::new(lines.next().unwrap().parse::<SnailfishPair>()?);
    let mut next;
    while {
        next = lines.next();
        next.is_some()
    } {
        let pair: Box<dyn SnailfishValue> = Box::new(next.unwrap().parse::<SnailfishPair>()?);
        sum += pair;
    }
    Ok(sum.magnitude().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let list = std::fs::read_to_string(input)?;
    let pairs: Vec<_> = list
        .lines()
        .map(|l| l.parse::<SnailfishPair>().unwrap())
        .collect();
    let mut max_magnitude = 0;
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i != j {
                let p1: Box<dyn SnailfishValue> = Box::new(pairs[i].clone());
                let p2: Box<dyn SnailfishValue> = Box::new(pairs[j].clone());
                max_magnitude = max_magnitude.max((p1 + p2).magnitude());
            }
        }
    }
    Ok(max_magnitude.to_string())
}
