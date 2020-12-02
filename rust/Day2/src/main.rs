use std::{io::BufRead, time::Instant};

#[derive(Debug)]
struct PolicyWithPassword {
    pub policy: Policy,
    pub password: String,
}

#[derive(Debug)]
struct Policy {
    pub min: usize,
    pub max: usize,
    pub chr: u8,
}

impl PolicyWithPassword {
    pub fn check_1(&self) -> bool {
        let iter = self.password.bytes().filter(|c| *c == self.policy.chr);
        let count = iter.take(self.policy.max + 1).count();
        count >= self.policy.min && count <= self.policy.max
    }

    pub fn check_2(&self) -> bool {
        let min = self.password.bytes().nth(self.policy.min - 1).unwrap();
        let max = self.password.bytes().nth(self.policy.max - 1).unwrap();
        (min == self.policy.chr) ^ (max == self.policy.chr)
    }
}

#[cfg(test)]
const TEST_DATA: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";

#[cfg(test)]
#[test]
fn test1() {
    let mut reader = TEST_DATA.as_bytes();
    for i in 0..3 {
        let parsed = dbg!(parse_line(&mut reader)).unwrap();
        let correct = dbg!(parsed.check_1());

        match i {
            0 => assert!(correct && parsed.password == "abcde"),
            1 => assert!(!correct && parsed.password == "cdefg"),
            2 => assert!(correct && parsed.password == "ccccccccc"),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test2() {
    let mut reader = TEST_DATA.as_bytes();
    for i in 0..3 {
        let parsed = dbg!(parse_line(&mut reader)).unwrap();
        let correct = dbg!(parsed.check_2());

        match i {
            0 => assert!(correct),
            1 => assert!(!correct),
            2 => assert!(!correct),
            _ => panic!(),
        }
    }
}

fn main() {
    let time_total = Instant::now();

    {
        let time_reading = Instant::now();
        let passwords = read_input();
        println!("took {:?} to read input", time_reading.elapsed());

        let time_solving = Instant::now();
        let count_correct = passwords.iter().filter(|p| p.check_1()).count();
        println!("took {:?} to solve 1", time_solving.elapsed());
        println!("solution 1: {}", count_correct);

        let time_solving = Instant::now();
        let count_correct = passwords.iter().filter(|p| p.check_2()).count();
        println!("took {:?} to solve 2", time_solving.elapsed());
        println!("solution 2: {}", count_correct);
    }

    println!("took {:?} in total", time_total.elapsed());
}

fn read_input() -> Vec<PolicyWithPassword> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut passwords = Vec::with_capacity(1000);
    loop {
        if let Some(parsed) = parse_line(&mut input) {
            passwords.push(parsed);
        } else {
            break;
        }
    }
    passwords
}

fn parse_line(mut text: &mut impl BufRead) -> Option<PolicyWithPassword> {
    // parse int
    let mut buf = [0u8];
    let min = parse_u8(&mut text);
    let max = parse_u8(&mut text);
    let chr = {
        if let Ok(_) = text.read_exact(&mut buf) {
            Some(buf[0] as char)
        } else {
            None
        }
    }?;
    text.consume(2);
    let mut password = String::new();
    text.read_line(&mut password).unwrap();

    Some(PolicyWithPassword {
        policy: Policy {
            min: min as usize,
            max: max as usize,
            chr: chr as u8,
        },
        password: password[..password.len()-1].into(),
    })
}

// reads 1 extra char
fn parse_u8(text: &mut impl BufRead) -> u8 {
    // parse int
    let mut parsed_int = 0;
    loop {
        let mut buf = [0u8];
        let success = text.read_exact(&mut buf);
        if success.is_ok() {
            let x = buf[0];
            if (x as char).is_digit(10) {
                parsed_int *= 10;
                parsed_int += x - '0' as u8;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    parsed_int
}