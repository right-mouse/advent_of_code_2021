use std::error::Error;
use std::str::Chars;

fn hex_to_bin(s: &str) -> Result<String, String> {
    let mut b = String::with_capacity(s.len() * 4);
    for c in s.chars() {
        match c {
            '0' => b.push_str("0000"),
            '1' => b.push_str("0001"),
            '2' => b.push_str("0010"),
            '3' => b.push_str("0011"),
            '4' => b.push_str("0100"),
            '5' => b.push_str("0101"),
            '6' => b.push_str("0110"),
            '7' => b.push_str("0111"),
            '8' => b.push_str("1000"),
            '9' => b.push_str("1001"),
            'A' => b.push_str("1010"),
            'B' => b.push_str("1011"),
            'C' => b.push_str("1100"),
            'D' => b.push_str("1101"),
            'E' => b.push_str("1110"),
            'F' => b.push_str("1111"),
            _ => return Err(format!("unrecognized hex char: {}", c)),
        }
    }
    Ok(b)
}

fn next_n(buf: &mut Chars, tot: &mut usize, n: usize) -> Result<String, String> {
    let mut s = Vec::with_capacity(n);
    for _ in 0..n {
        s.push(
            buf.next()
                .ok_or::<String>("reached end of string".to_string())?,
        );
    }
    *tot += n;
    Ok(s.iter().collect())
}

#[derive(Debug)]
enum Payload {
    Literal(String),
    Packets(Vec<Packet>),
}

impl Default for Payload {
    fn default() -> Self {
        Payload::Literal(String::new())
    }
}

#[derive(Debug, Default)]
struct Packet {
    version: u8,
    type_id: u8,
    length_type_id: u8,
    length: usize,
    payload: Payload,
}

impl Packet {
    fn from_buf(buf: &mut Chars, tot: &mut usize) -> Result<Self, Box<dyn Error>> {
        let mut p: Packet = Default::default();
        p.version = u8::from_str_radix(&next_n(buf, tot, 3)?, 2)?;
        p.type_id = u8::from_str_radix(&next_n(buf, tot, 3)?, 2)?;
        match p.type_id {
            4 => {
                let mut literal = String::new();
                while next_n(buf, tot, 1)? == "1" {
                    literal.push_str(&next_n(buf, tot, 4)?);
                }
                literal.push_str(&next_n(buf, tot, 4)?);
                p.payload = Payload::Literal(literal);
            }
            _ => {
                p.length_type_id = u8::from_str_radix(&next_n(buf, tot, 1)?, 2)?;
                let len = if p.length_type_id == 0 { 15 } else { 11 };
                p.length = usize::from_str_radix(&next_n(buf, tot, len)?, 2)?;
                let mut packets = Vec::new();
                match p.length_type_id {
                    0 => {
                        let initial = tot.clone();
                        while (*tot - initial) < p.length {
                            packets.push(Packet::from_buf(buf, tot)?);
                        }
                    }
                    _ => {
                        for _ in 0..p.length {
                            packets.push(Packet::from_buf(buf, tot)?);
                        }
                    }
                }
                p.payload = Payload::Packets(packets);
            }
        }
        Ok(p)
    }

    fn version_total(&self) -> usize {
        let mut total = self.version as usize;
        match &self.payload {
            Payload::Packets(packets) => {
                for p in packets.iter() {
                    total += p.version_total();
                }
            }
            _ => (),
        }
        total
    }

    fn eval(&self) -> Result<usize, Box<dyn Error>> {
        match &self.payload {
            Payload::Literal(l) => {
                return Ok(usize::from_str_radix(l, 2)?);
            }
            Payload::Packets(packets) => match self.type_id {
                0 => {
                    let mut sum = 0;
                    for p in packets.iter() {
                        sum += p.eval()?;
                    }
                    return Ok(sum);
                }
                1 => {
                    let mut product = 1;
                    for p in packets.iter() {
                        product *= p.eval()?;
                    }
                    return Ok(product);
                }
                2 => {
                    let mut min = usize::MAX;
                    for p in packets.iter() {
                        min = min.min(p.eval()?);
                    }
                    return Ok(min);
                }
                3 => {
                    let mut max = 0;
                    for p in packets.iter() {
                        max = max.max(p.eval()?);
                    }
                    return Ok(max);
                }
                5 => {
                    return Ok(if packets[0].eval()? > packets[1].eval()? {
                        1
                    } else {
                        0
                    });
                }
                6 => {
                    return Ok(if packets[0].eval()? < packets[1].eval()? {
                        1
                    } else {
                        0
                    });
                }
                7 => {
                    return Ok(if packets[0].eval()? == packets[1].eval()? {
                        1
                    } else {
                        0
                    });
                }
                _ => {
                    return Err(format!("unrecognized packet type: {}", self.type_id).into());
                }
            },
        }
    }
}

pub fn prob_1(input: &str) -> Result<String, Box<dyn Error>> {
    let packet_str = hex_to_bin(&std::fs::read_to_string(input)?.trim())?;
    let mut t = 0;
    let packet = Packet::from_buf(&mut packet_str.chars(), &mut t)?;
    Ok(packet.version_total().to_string())
}

pub fn prob_2(input: &str) -> Result<String, Box<dyn Error>> {
    let packet_str = hex_to_bin(&std::fs::read_to_string(input)?.trim())?;
    let mut t = 0;
    let packet = Packet::from_buf(&mut packet_str.chars(), &mut t)?;
    Ok(packet.eval()?.to_string())
}
