use std::str::FromStr;
use utilities::{ParseError, read_list};

const DRUMM: i32 = 100;

enum Dir {
    Left,
    Right,
}

struct Line {
    dir: Dir,
    off: i32,
}

impl FromStr for Line {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().collect::<Vec<_>>()[0];
        let dir = match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => return Err(ParseError::WrongChar(c)),
        };
        let off = s[1..].parse()?;
        Ok(Line { dir, off })
    }
}

fn main() -> std::io::Result<()> {
    let input = read_list::<Line>("day01/input.txt")?;
    let simple_val = resolve_simple(&input);
    println!("Simple value: {}", simple_val);
    let complex_val = resolve_complex(&input);
    println!("Complex value: {}", complex_val);
    let complex_val_2 = resolve_complex_2(&input);
    println!("Complex value 2: {}", complex_val_2);

    Ok(())
}

fn resolve_simple(input: &[Line]) -> usize {
    let mut state = 50;
    let mut out = 0;
    for Line { dir, off } in input {
        match dir {
            Dir::Left => state -= *off,
            Dir::Right => state += *off,
        };
        state %= DRUMM;
        if state == 0 {
            out += 1;
        }
    }
    out
}

fn resolve_complex(input: &[Line]) -> usize {
    let mut state = 50;
    let mut out = 0;
    for Line { dir, off } in input {
        let mul = match dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };

        out += (1..=*off)
            .map(|off| (state + mul * off) % DRUMM)
            .filter(|i| *i == 0)
            .count();
        state += mul * *off;
    }
    out
}

fn resolve_complex_2(input: &[Line]) -> usize {
    let mut state = 50;
    let mut out = 0;

    for Line { dir, off } in input {
        out += (off / DRUMM) as usize;
        state += match dir {
            Dir::Left => {
                out += count_rest_rotation_neg(state, off % DRUMM);
                -*off
            }
            Dir::Right => {
                out += count_rest_rotation_pos(state, off % DRUMM);
                *off
            }
        };
        state = state.rem_euclid(DRUMM);
    }

    out
}

fn count_rest_rotation_pos(state: i32, off: i32) -> usize {
    if state + off >= DRUMM { 1 } else { 0 }
}
fn count_rest_rotation_neg(state: i32, off: i32) -> usize {
    if state - off <= 0 && state > 0 { 1 } else { 0 }
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    fn read_input() -> Vec<Line> {
        read_list("example_1.txt").unwrap()
    }

    #[test]
    fn simple_01() {
        let input = read_input();
        assert_eq!(3, resolve_simple(&input));
    }
    #[test]
    fn complex_01() {
        let input = read_input();
        assert_eq!(6, resolve_complex(&input));
    }
    #[test]
    fn complex_02() {
        let input = read_input();
        assert_eq!(6, resolve_complex_2(&input));
    }
}
