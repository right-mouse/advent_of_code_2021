use std::error::Error;
use std::str::FromStr;
use std::thread;

const HALLWAY_LOCS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

#[derive(Copy, Clone, Eq, PartialEq)]
enum Room {
    Room1,
    Room2,
    Room3,
    Room4,
}

impl Room {
    fn expected_amphipod(self) -> Amphipod {
        match self {
            Room::Room1 => Amphipod::Amber,
            Room::Room2 => Amphipod::Bronze,
            Room::Room3 => Amphipod::Copper,
            Room::Room4 => Amphipod::Desert,
        }
    }

    fn pos(self) -> usize {
        match self {
            Room::Room1 => 2,
            Room::Room2 => 4,
            Room::Room3 => 6,
            Room::Room4 => 8,
        }
    }

    fn dist(self, room_pos: usize, hallway_pos: usize) -> usize {
        let mut d = room_pos + 1;
        d += ((hallway_pos as i64) - (self.pos() as i64)).abs() as usize;
        d
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl FromStr for Amphipod {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::Amber),
            "B" => Ok(Amphipod::Bronze),
            "C" => Ok(Amphipod::Copper),
            "D" => Ok(Amphipod::Desert),
            _ => Err(format!("invalid amphipod: {}", s).into()),
        }
    }
}

impl Amphipod {
    fn expected_room(self) -> Room {
        match self {
            Amphipod::Amber => Room::Room1,
            Amphipod::Bronze => Room::Room2,
            Amphipod::Copper => Room::Room3,
            Amphipod::Desert => Room::Room4,
        }
    }

    fn energy(self, steps: usize) -> usize {
        match self {
            Amphipod::Amber => steps,
            Amphipod::Bronze => steps * 10,
            Amphipod::Copper => steps * 100,
            Amphipod::Desert => steps * 1000,
        }
    }
}

#[derive(Copy, Clone)]
struct Burrow {
    room_1: [Option<Amphipod>; 2],
    room_2: [Option<Amphipod>; 2],
    room_3: [Option<Amphipod>; 2],
    room_4: [Option<Amphipod>; 2],
    hallway: [Option<Amphipod>; 11],
    energy_spent: usize,
}

#[derive(Copy, Clone)]
struct BurrowExpanded {
    room_1: [Option<Amphipod>; 4],
    room_2: [Option<Amphipod>; 4],
    room_3: [Option<Amphipod>; 4],
    room_4: [Option<Amphipod>; 4],
    hallway: [Option<Amphipod>; 11],
    energy_spent: usize,
}

impl FromStr for Burrow {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_invalid_input = Err("invalid input".into());
        let mut lines = s.lines();
        if lines.clone().count() != 5 {
            return err_invalid_input;
        }
        // Skip first 2 lines.
        lines.next();
        lines.next();
        let top_row = lines
            .next()
            .unwrap()
            .split("#")
            .filter_map(|p| {
                let q = p.trim();
                if q.is_empty() {
                    None
                } else {
                    Some(q)
                }
            })
            .collect::<Vec<_>>();
        if top_row.len() != 4 {
            return err_invalid_input;
        }
        let bot_row = lines
            .next()
            .unwrap()
            .split("#")
            .filter_map(|p| {
                let q = p.trim();
                if q.is_empty() {
                    None
                } else {
                    Some(q)
                }
            })
            .collect::<Vec<_>>();
        if bot_row.len() != 4 {
            return err_invalid_input;
        }
        Ok(Burrow {
            room_1: [Some(top_row[0].parse()?), Some(bot_row[0].parse()?)],
            room_2: [Some(top_row[1].parse()?), Some(bot_row[1].parse()?)],
            room_3: [Some(top_row[2].parse()?), Some(bot_row[2].parse()?)],
            room_4: [Some(top_row[3].parse()?), Some(bot_row[3].parse()?)],
            hallway: Default::default(),
            energy_spent: Default::default(),
        })
    }
}

impl Burrow {
    fn to_expanded(&self) -> BurrowExpanded {
        BurrowExpanded {
            room_1: [
                self.room_1[0],
                Some(Amphipod::Desert),
                Some(Amphipod::Desert),
                self.room_1[1],
            ],
            room_2: [
                self.room_2[0],
                Some(Amphipod::Copper),
                Some(Amphipod::Bronze),
                self.room_2[1],
            ],
            room_3: [
                self.room_3[0],
                Some(Amphipod::Bronze),
                Some(Amphipod::Amber),
                self.room_3[1],
            ],
            room_4: [
                self.room_4[0],
                Some(Amphipod::Amber),
                Some(Amphipod::Copper),
                self.room_4[1],
            ],
            hallway: self.hallway,
            energy_spent: self.energy_spent,
        }
    }

    fn get_room(&self, room: Room) -> &[Option<Amphipod>; 2] {
        match room {
            Room::Room1 => &self.room_1,
            Room::Room2 => &self.room_2,
            Room::Room3 => &self.room_3,
            Room::Room4 => &self.room_4,
        }
    }
}

impl BurrowExpanded {
    fn get_room(&self, room: Room) -> &[Option<Amphipod>; 4] {
        match room {
            Room::Room1 => &self.room_1,
            Room::Room2 => &self.room_2,
            Room::Room3 => &self.room_3,
            Room::Room4 => &self.room_4,
        }
    }
}

fn move_amphipod(
    burrow: Burrow,
    from_hallway: bool,
    room: Room,
    old_pos: usize,
    new_pos: usize,
) -> Option<Burrow> {
    let mut new = burrow.clone();
    let am: Amphipod;
    let ro = match room {
        Room::Room1 => &mut new.room_1,
        Room::Room2 => &mut new.room_2,
        Room::Room3 => &mut new.room_3,
        Room::Room4 => &mut new.room_4,
    };
    let room_pos = room.pos();
    if from_hallway {
        am = new.hallway[old_pos].unwrap();
        // Check if there is a clear path.
        if old_pos < room_pos {
            for i in (old_pos + 1)..=room_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        } else {
            for i in room_pos..old_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        }
        for i in 0..=new_pos {
            if ro[i].is_some() {
                return None;
            }
        }
        // Move the amphipod.
        ro[new_pos] = Some(am);
        new.hallway[old_pos] = None;
        new.energy_spent += am.energy(room.dist(new_pos, old_pos));
    } else {
        am = ro[old_pos].unwrap();
        // Check if there is a clear path.
        if old_pos > 0 {
            for i in 0..old_pos {
                if ro[i].is_some() {
                    return None;
                }
            }
        }
        if new_pos < room_pos {
            for i in new_pos..=room_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        } else {
            for i in room_pos..=new_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        }
        // Move the amphipod.
        new.hallway[new_pos] = Some(am);
        ro[old_pos] = None;
        new.energy_spent += am.energy(room.dist(old_pos, new_pos));
    }
    Some(new)
}

fn move_amphipod_expanded(
    burrow: BurrowExpanded,
    from_hallway: bool,
    room: Room,
    old_pos: usize,
    new_pos: usize,
) -> Option<BurrowExpanded> {
    let mut new = burrow.clone();
    let am: Amphipod;
    let ro = match room {
        Room::Room1 => &mut new.room_1,
        Room::Room2 => &mut new.room_2,
        Room::Room3 => &mut new.room_3,
        Room::Room4 => &mut new.room_4,
    };
    let room_pos = room.pos();
    if from_hallway {
        am = new.hallway[old_pos].unwrap();
        // Check if there is a clear path.
        if old_pos < room_pos {
            for i in (old_pos + 1)..=room_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        } else {
            for i in room_pos..old_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        }
        for i in 0..=new_pos {
            if ro[i].is_some() {
                return None;
            }
        }
        // Move the amphipod.
        ro[new_pos] = Some(am);
        new.hallway[old_pos] = None;
        new.energy_spent += am.energy(room.dist(new_pos, old_pos));
    } else {
        am = ro[old_pos].unwrap();
        // Check if there is a clear path.
        if old_pos > 0 {
            for i in 0..old_pos {
                if ro[i].is_some() {
                    return None;
                }
            }
        }
        if new_pos < room_pos {
            for i in new_pos..=room_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        } else {
            for i in room_pos..=new_pos {
                if new.hallway[i].is_some() {
                    return None;
                }
            }
        }
        // Move the amphipod.
        new.hallway[new_pos] = Some(am);
        ro[old_pos] = None;
        new.energy_spent += am.energy(room.dist(old_pos, new_pos));
    }
    Some(new)
}

fn rearrange(burrow: Burrow, min_energy: &mut usize) {
    if burrow.energy_spent >= *min_energy {
        return;
    }
    // Check if already arranged.
    let mut already_arranged = true;
    for r in [Room::Room1, Room::Room2, Room::Room3, Room::Room4] {
        let ro = burrow.get_room(r);
        for a in ro.iter() {
            if a.is_none() || a.unwrap().expected_room() != r {
                already_arranged = false;
                break;
            }
        }
        if already_arranged == false {
            break;
        }
    }
    if already_arranged {
        *min_energy = (*min_energy).min(burrow.energy_spent);
        return;
    }
    // Check rooms.
    for r in [Room::Room1, Room::Room2, Room::Room3, Room::Room4] {
        let ro = burrow.get_room(r);
        for i in 0..=1 {
            if ro[i].is_some() {
                let am = ro[i].unwrap();
                let try_move: bool;
                if i == 1 {
                    try_move = am != r.expected_amphipod();
                } else {
                    try_move = (am != r.expected_amphipod())
                        || (ro[1].is_some() && ro[1].unwrap() != r.expected_amphipod());
                }
                if try_move {
                    for j in HALLWAY_LOCS {
                        match move_amphipod(burrow, false, r, i, j) {
                            Some(new_burrow) => rearrange(new_burrow, min_energy),
                            None => (),
                        }
                    }
                }
            }
        }
    }
    // Check hallway.
    for i in HALLWAY_LOCS {
        if burrow.hallway[i].is_some() {
            let am = burrow.hallway[i].unwrap();
            let r = am.expected_room();
            let ro = burrow.get_room(r);
            let j = if ro[1].is_some() && ro[1].unwrap() == r.expected_amphipod() {
                0
            } else {
                1
            };
            match move_amphipod(burrow, true, r, i, j) {
                Some(new_burrow) => rearrange(new_burrow, min_energy),
                None => (),
            }
        }
    }
}

fn rearrange_expanded(burrow: BurrowExpanded, min_energy: &mut usize) {
    if burrow.energy_spent >= *min_energy {
        return;
    }
    // Check if already arranged.
    let mut already_arranged = true;
    for r in [Room::Room1, Room::Room2, Room::Room3, Room::Room4] {
        let ro = burrow.get_room(r);
        for a in ro.iter() {
            if a.is_none() || a.unwrap().expected_room() != r {
                already_arranged = false;
                break;
            }
        }
        if already_arranged == false {
            break;
        }
    }
    if already_arranged {
        *min_energy = (*min_energy).min(burrow.energy_spent);
        return;
    }
    // Check rooms.
    for r in [Room::Room1, Room::Room2, Room::Room3, Room::Room4] {
        let ro = burrow.get_room(r);
        for i in 0..=3 {
            if ro[i].is_some() {
                let am = ro[i].unwrap();
                let try_move: bool;
                if i == 3 {
                    try_move = am != r.expected_amphipod();
                } else if i == 2 {
                    try_move = (am != r.expected_amphipod())
                        || (ro[3].is_some() && ro[3].unwrap() != r.expected_amphipod());
                } else if i == 1 {
                    try_move = (am != r.expected_amphipod())
                        || (ro[3].is_some() && ro[3].unwrap() != r.expected_amphipod())
                        || (ro[2].is_some() && ro[2].unwrap() != r.expected_amphipod());
                } else {
                    try_move = (am != r.expected_amphipod())
                        || (ro[3].is_some() && ro[3].unwrap() != r.expected_amphipod())
                        || (ro[2].is_some() && ro[2].unwrap() != r.expected_amphipod())
                        || (ro[1].is_some() && ro[1].unwrap() != r.expected_amphipod());
                }
                if try_move {
                    for j in HALLWAY_LOCS {
                        match move_amphipod_expanded(burrow, false, r, i, j) {
                            Some(new_burrow) => rearrange_expanded(new_burrow, min_energy),
                            None => (),
                        }
                    }
                }
            }
        }
    }
    // Check hallway.
    for i in HALLWAY_LOCS {
        if burrow.hallway[i].is_some() {
            let am = burrow.hallway[i].unwrap();
            let r = am.expected_room();
            let ro = burrow.get_room(r);
            let j = if ro[3].is_some() && ro[3].unwrap() == r.expected_amphipod() {
                if ro[2].is_some() && ro[2].unwrap() == r.expected_amphipod() {
                    if ro[1].is_some() && ro[1].unwrap() == r.expected_amphipod() {
                        0
                    } else {
                        1
                    }
                } else {
                    2
                }
            } else {
                3
            };
            match move_amphipod_expanded(burrow, true, r, i, j) {
                Some(new_burrow) => rearrange_expanded(new_burrow, min_energy),
                None => (),
            }
        }
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let burrow: Burrow = std::fs::read_to_string(input)?.parse()?;
    let mut min_energy = usize::MAX;
    rearrange(burrow, &mut min_energy);
    Ok(min_energy.to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let burrow: BurrowExpanded = std::fs::read_to_string(input)?
        .parse::<Burrow>()?
        .to_expanded();
    let mut min_energy = usize::MAX;
    rearrange_expanded(burrow, &mut min_energy);
    Ok(min_energy.to_string())
}
