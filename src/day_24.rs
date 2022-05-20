use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

trait Instruction: Send + Sync {
    fn eval(&self, a: &mut i64, b: i64);
}

#[derive(Clone)]
struct Add {}

impl Instruction for Add {
    fn eval(&self, a: &mut i64, b: i64) {
        *a += b;
    }
}

#[derive(Clone)]
struct Mul {}

impl Instruction for Mul {
    fn eval(&self, a: &mut i64, b: i64) {
        *a *= b;
    }
}

#[derive(Clone)]
struct Div {}

impl Instruction for Div {
    fn eval(&self, a: &mut i64, b: i64) {
        *a /= b;
    }
}

#[derive(Clone)]
struct Mod {}

impl Instruction for Mod {
    fn eval(&self, a: &mut i64, b: i64) {
        *a %= b;
    }
}

#[derive(Clone)]
struct Eql {}

impl Instruction for Eql {
    fn eval(&self, a: &mut i64, b: i64) {
        *a = (*a == b) as i64;
    }
}

fn instruction_from_str(s: &str) -> Result<Arc<dyn Instruction>, Box<dyn Error>> {
    match s {
        "add" => return Ok(Arc::from(Add {})),
        "mul" => return Ok(Arc::from(Mul {})),
        "div" => return Ok(Arc::from(Div {})),
        "mod" => return Ok(Arc::from(Mod {})),
        "eql" => return Ok(Arc::from(Eql {})),
        _ => return Err(format!("invalid op: {}", s).into()),
    }
}

#[derive(Clone, Default)]
struct ALU {
    inputs: HashSet<String>,
    steps: Vec<(Arc<dyn Instruction + 'static>, String, String)>,
}

impl FromStr for ALU {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut alu: ALU = Default::default();
        for l in s.lines() {
            let parts = l.split(" ").collect::<Vec<_>>();
            match parts[0] {
                "inp" => {
                    if parts.len() != 2 {
                        return Err(format!("invalid line: {}", l).into());
                    }
                    alu.inputs.insert(parts[1].to_string());
                }
                _ => {
                    let ins = instruction_from_str(parts[0])?;
                    if parts.len() != 3 {
                        return Err(format!("invalid line: {}", l).into());
                    }
                    alu.steps
                        .push((ins, parts[1].to_string(), parts[2].to_string()));
                }
            }
        }
        Ok(alu)
    }
}

impl ALU {
    fn eval(
        &self,
        input: &HashMap<String, i64>,
        start_vars: &HashMap<String, i64>,
    ) -> Result<HashMap<String, i64>, String> {
        if input.len() != self.inputs.len() {
            return Err(format!(
                "expected {} inputs but got {}",
                self.inputs.len(),
                input.len()
            ));
        }
        let mut vars: HashMap<String, i64> = HashMap::new();
        for (name, val) in input.iter() {
            if !self.inputs.contains(name) {
                return Err(format!("{} is not a valid input", name));
            }
            vars.insert(name.clone(), *val);
        }
        for (name, val) in start_vars.iter() {
            vars.insert(name.clone(), *val);
        }
        for (step, inp1, inp2) in self.steps.iter() {
            let b = match inp2.parse::<i64>() {
                Ok(i) => i,
                Err(_) => *vars.get(inp2).unwrap_or(&0),
            };
            let a = vars.entry(inp1.clone()).or_insert(0);
            step.eval(a, b);
        }
        Ok(vars)
    }
}

fn parse_monad(s: &str) -> Result<Vec<ALU>, Box<dyn Error>> {
    let mut v = Vec::new();
    let mut l = s.lines();
    let mut line = "";
    let mut alu_lines = String::new();
    while {
        match l.next() {
            Some(st) => {
                line = st;
                true
            }
            None => false,
        }
    } {
        if line.starts_with("inp") {
            if !alu_lines.is_empty() {
                v.push(alu_lines.parse()?);
                alu_lines.clear();
            }
        } else {
            alu_lines.push('\n');
        }
        alu_lines.push_str(line);
    }
    if !alu_lines.is_empty() {
        v.push(alu_lines.parse()?);
    }
    Ok(v)
}

fn get_states(alu: &ALU, starting_z: i64) -> Result<Vec<(i64, i64)>, String> {
    let mut threads: Vec<JoinHandle<Result<(i64, i64), String>>> = Vec::new();
    for w in 1..=9 {
        let a = alu.clone();
        threads.push(thread::spawn(move || {
            let res = a.eval(
                &HashMap::from_iter([("w".to_string(), w)]),
                &HashMap::from_iter([("z".to_string(), starting_z)]),
            )?;
            Ok((w, *res.get("z").unwrap_or(&0)))
        }));
    }
    let mut states = Vec::with_capacity(9);
    for t in threads {
        states.push(t.join().unwrap()?);
    }
    Ok(states)
}

fn get_monad_num(monad: &Vec<ALU>, max: bool) -> Result<String, Box<dyn Error>> {
    let mut input_z: HashMap<i64, Vec<i64>> = HashMap::new();
    input_z.insert(0, Vec::new());
    for (i, alu) in monad.iter().enumerate() {
        println!(
            "Getting character {} ({} input z states)",
            i + 1,
            input_z.len()
        );
        let mut updated_input_z: HashMap<i64, Vec<i64>> = HashMap::new();
        for (in_z, inputs_so_far) in input_z.iter() {
            for (w, z) in get_states(alu, *in_z)?.into_iter() {
                let mut updated_inputs = inputs_so_far.clone();
                updated_inputs.push(w);
                if updated_input_z.contains_key(&z) {
                    let existing_str = updated_input_z
                        .get(&z)
                        .unwrap()
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<String>();
                    let new_str = updated_inputs
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<String>();
                    let to_insert = if max {
                        new_str > existing_str
                    } else {
                        new_str < existing_str
                    };
                    if to_insert {
                        updated_input_z.insert(z, updated_inputs);
                    }
                } else {
                    updated_input_z.insert(z, updated_inputs);
                }
            }
        }
        input_z = updated_input_z;
    }
    Ok(input_z
        .get(&0)
        .unwrap_or(&vec![0])
        .iter()
        .map(|c| c.to_string())
        .collect::<String>())
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let monad: Vec<ALU> = parse_monad(&std::fs::read_to_string(input)?)?;
    let model_number = get_monad_num(&monad, true)?;
    Ok(model_number.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let monad: Vec<ALU> = parse_monad(&std::fs::read_to_string(input)?)?;
    let model_number = get_monad_num(&monad, false)?;
    Ok(model_number.to_string())
}
