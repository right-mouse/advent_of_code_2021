use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::str::FromStr;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CaveType {
    Big,
    Small,
}

#[derive(Debug)]
struct Cave {
    typ: CaveType,
    connected_caves: Vec<String>,
}

#[derive(Debug)]
struct CaveGraph {
    caves: HashMap<String, Cave>,
}

impl FromStr for CaveGraph {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = CaveGraph {
            caves: HashMap::new(),
        };
        for l in s.lines() {
            let parts: Vec<&str> = l.split("-").collect();
            if parts.len() != 2 {
                return Err(format!("invalid connection: {}", l).into());
            }
            if !graph.caves.contains_key(parts[0]) {
                graph.caves.insert(
                    parts[0].to_string(),
                    Cave {
                        typ: if parts[0].chars().all(char::is_lowercase) {
                            CaveType::Small
                        } else {
                            CaveType::Big
                        },
                        connected_caves: Vec::new(),
                    },
                );
            }
            if !graph.caves.contains_key(parts[1]) {
                graph.caves.insert(
                    parts[1].to_string(),
                    Cave {
                        typ: if parts[1].chars().all(char::is_lowercase) {
                            CaveType::Small
                        } else {
                            CaveType::Big
                        },
                        connected_caves: Vec::new(),
                    },
                );
            }
            graph
                .caves
                .get_mut(parts[0])
                .unwrap()
                .connected_caves
                .push(parts[1].to_string());
            graph
                .caves
                .get_mut(parts[1])
                .unwrap()
                .connected_caves
                .push(parts[0].to_string());
        }
        Ok(graph)
    }
}

impl CaveGraph {
    fn expand_path(&self, paths: &mut Vec<Vec<String>>, path: Vec<String>) {
        for p in self.caves[path.last().unwrap()].connected_caves.iter() {
            if *p == "end" {
                let mut complete_path = path.clone();
                complete_path.push(p.clone());
                paths.push(complete_path);
            } else {
                if self.caves[p].typ == CaveType::Small && path.contains(p) {
                    // Path is a dead end since a small cave can't be visited twice.
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(p.clone());
                self.expand_path(paths, new_path);
            }
        }
    }

    fn expand_path_advanced(&self, paths: &mut Vec<Vec<String>>, path: Vec<String>) {
        for p in self.caves[path.last().unwrap()].connected_caves.iter() {
            if *p == "start" {
                // Can't go back to start.
                continue;
            } else if *p == "end" {
                let mut complete_path = path.clone();
                complete_path.push(p.clone());
                paths.push(complete_path);
            } else {
                let can_visit_small_cave_twice = has_unique_elements(
                    path.iter()
                        .filter(|&v| self.caves[v].typ == CaveType::Small),
                );
                if self.caves[p].typ == CaveType::Small
                    && !can_visit_small_cave_twice
                    && path.contains(p)
                {
                    // Path is a dead end since a small cave can't be visited twice more than once.
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(p.clone());
                self.expand_path_advanced(paths, new_path);
            }
        }
    }

    fn path_find(&self) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.expand_path(&mut paths, vec!["start".to_string()]);
        paths
    }

    fn path_find_advanced(&self) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.expand_path_advanced(&mut paths, vec!["start".to_string()]);
        paths
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let graph: CaveGraph = std::fs::read_to_string(input)?.parse()?;
    Ok(graph.path_find().len().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let graph: CaveGraph = std::fs::read_to_string(input)?.parse()?;
    Ok(graph.path_find_advanced().len().to_string())
}
