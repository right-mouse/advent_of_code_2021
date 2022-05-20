use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct DiracDice {
    p1_pos: u64,
    p2_pos: u64,
    p1_points: u64,
    p2_points: u64,
}

impl FromStr for DiracDice {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.lines().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err("invalid input".into());
        }
        Ok(DiracDice {
            p1_pos: parts[0].chars().last().unwrap().to_string().parse()?,
            p2_pos: parts[1].chars().last().unwrap().to_string().parse()?,
            p1_points: 0,
            p2_points: 0,
        })
    }
}

impl DiracDice {
    fn roll_p1(&mut self, roll: u64) {
        self.p1_pos += roll;
        if self.p1_pos > 10 {
            self.p1_pos = ((self.p1_pos - 1) % 10) + 1;
        }
        self.p1_points += self.p1_pos as u64;
    }

    fn roll_p2(&mut self, roll: u64) {
        self.p2_pos += roll;
        if self.p2_pos > 10 {
            self.p2_pos = ((self.p2_pos - 1) % 10) + 1;
        }
        self.p2_points += self.p2_pos as u64;
    }
}

struct DeterministicDice {
    next_roll: u64,
    num_rolls: u64,
}

impl Default for DeterministicDice {
    fn default() -> Self {
        DeterministicDice {
            next_roll: 1,
            num_rolls: 0,
        }
    }
}

impl DeterministicDice {
    fn roll(&mut self) -> u64 {
        let r = self.next_roll;
        self.num_rolls += 1;
        self.next_roll += 1;
        if self.next_roll > 100 {
            self.next_roll = 1;
        }
        r
    }
}

fn roll_quantum_die(wins: &mut [u64; 2], d: &DiracDice, player: bool, num: u64) {
    if player {
        if d.p1_points >= 21 {
            wins[0] += num;
            return;
        }
    } else {
        if d.p2_points >= 21 {
            wins[1] += num;
            return;
        }
    }

    let mut d1 = d.clone();
    let mut d2 = d.clone();
    let mut d3 = d.clone();
    let mut d4 = d.clone();
    let mut d5 = d.clone();
    let mut d6 = d.clone();
    let mut d7 = d.clone();

    if player {
        d1.roll_p2(3);
        d2.roll_p2(4);
        d3.roll_p2(5);
        d4.roll_p2(6);
        d5.roll_p2(7);
        d6.roll_p2(8);
        d7.roll_p2(9);
    } else {
        d1.roll_p1(3);
        d2.roll_p1(4);
        d3.roll_p1(5);
        d4.roll_p1(6);
        d5.roll_p1(7);
        d6.roll_p1(8);
        d7.roll_p1(9);
    }

    roll_quantum_die(wins, &d1, !player, 1 * num); // Sum 3 occurs 1 time.
    roll_quantum_die(wins, &d2, !player, 3 * num); // Sum 3 occurs 3 times.
    roll_quantum_die(wins, &d3, !player, 6 * num); // Sum 3 occurs 6 times.
    roll_quantum_die(wins, &d4, !player, 7 * num); // Sum 3 occurs 7 times.
    roll_quantum_die(wins, &d5, !player, 6 * num); // Sum 3 occurs 6 times.
    roll_quantum_die(wins, &d6, !player, 3 * num); // Sum 3 occurs 3 times.
    roll_quantum_die(wins, &d7, !player, 1 * num); // Sum 3 occurs 1 time.
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut d: DiracDice = std::fs::read_to_string(input)?.parse()?;
    let mut die: DeterministicDice = Default::default();
    let val;
    loop {
        d.roll_p1(die.roll() + die.roll() + die.roll());
        if d.p1_points >= 1000 {
            val = d.p2_points * die.num_rolls;
            break;
        }
        d.roll_p2(die.roll() + die.roll() + die.roll());
        if d.p2_points >= 1000 {
            val = d.p1_points * die.num_rolls;
            break;
        }
    }
    Ok(val.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let d: DiracDice = std::fs::read_to_string(input)?.parse()?;
    let mut wins = [0; 2];
    roll_quantum_die(&mut wins, &d, false, 1);
    Ok(wins[0].max(wins[1]).to_string())
}
