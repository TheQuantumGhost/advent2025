use std::str::FromStr;
use utilities::{ParseError, read_list};

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
        state %= 100;
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
            .map(|off| (state + mul * off) % 100)
            .filter(|i| *i == 0)
            .count();
        state += mul * *off;
    }
    out
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
}
