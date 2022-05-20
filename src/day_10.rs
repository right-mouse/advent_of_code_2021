use std::error::Error;

fn reverse_paranthesis(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c,
    }
}

fn illegal_char_lut(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn autocomplete_char_lut(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn get_median(v: &Vec<u64>) -> u64 {
    let mid = v.len() / 2;
    v[mid]
}

struct NavigationSubsystemParser {}

impl NavigationSubsystemParser {
    fn check_corrupt(&self, s: &str) -> u64 {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let expected = stack.pop();
                    match expected {
                        Some(e) => {
                            if reverse_paranthesis(e) != c {
                                return illegal_char_lut(c);
                            }
                        }
                        None => return illegal_char_lut(c),
                    }
                }
                _ => (),
            }
        }
        0
    }

    fn check_incomplete(&self, s: &str) -> u64 {
        if self.check_corrupt(s) != 0 {
            return 0;
        }
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => (),
            }
        }
        let mut score: u64 = 0;
        for &c in stack.iter().rev() {
            score *= 5;
            score += autocomplete_char_lut(reverse_paranthesis(c));
        }
        score
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let nav_subsystem_cmds = std::fs::read_to_string(input)?;
    let parser = NavigationSubsystemParser {};
    let mut syntax_error_score: u64 = 0;
    for cmd in nav_subsystem_cmds.lines() {
        syntax_error_score += parser.check_corrupt(cmd);
    }
    Ok(syntax_error_score.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let nav_subsystem_cmds = std::fs::read_to_string(input)?;
    let parser = NavigationSubsystemParser {};
    let mut autocomplete_error_scores: Vec<u64> = Vec::new();
    for cmd in nav_subsystem_cmds.lines() {
        let s = parser.check_incomplete(cmd);
        if s != 0 {
            autocomplete_error_scores.push(s);
        }
    }
    autocomplete_error_scores.sort();
    Ok(get_median(&autocomplete_error_scores).to_string())
}
