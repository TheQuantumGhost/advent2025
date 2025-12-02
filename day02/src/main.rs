use std::{collections::HashMap, str::FromStr};

use utilities::{ParseError, read_comma_list};

struct Range {
    first: usize,
    last: usize,
}
impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.trim().split('-').collect::<Vec<_>>();

        if segments.len() != 2 {
            return Err(Self::Err::MismatchedLength {
                expect: 2,
                got: segments.len(),
            });
        };
        let first = segments[0].parse()?;
        let last = segments[1].parse()?;
        Ok(Self { first, last })
    }
}

fn main() -> std::io::Result<()> {
    let input = read_comma_list("day02/input.txt")?;
    let simple_val = resolve_simple(&input);
    println!("Simple value: {}", simple_val);
    let complex_val = resolve_complex(&input);
    println!("Complex value: {}", complex_val);

    Ok(())
}

// Returns 11 for length 2, 101 for 4 and so on
fn div_for_len(len: u32) -> usize {
    1 + 10usize.pow(len / 2)
}

fn is_invalid_id(id: usize) -> bool {
    let l = id.ilog10() + 1;
    if !l.is_multiple_of(2) {
        return false;
    }
    id.is_multiple_of(div_for_len(l))
}

fn resolve_simple(input: &[Range]) -> usize {
    input
        .iter()
        .flat_map(|Range { first, last }| *first..=*last)
        .filter(|id| is_invalid_id(*id))
        .sum()
}

fn mk_divisor(p: u32, q: u32) -> usize {
    let mut out = 1;
    for _ in 1..p {
        out *= 10usize.pow(q);
        out += 1;
    }
    out
}

fn gen_divisors(len: u32) -> Vec<usize> {
    (1..=len)
        .filter(|n| *n >= 2 && len.is_multiple_of(*n))
        .map(|p| mk_divisor(p, len / p))
        .collect()
}

fn get_divisors(divs: &mut HashMap<u32, Vec<usize>>, len: u32) -> &[usize] {
    //if !divs.contains_key(&len) {
    //    divs.insert(len, gen_divisors(len));
    //}
    divs.entry(len).or_insert_with(|| gen_divisors(len));

    divs.get(&len).unwrap()
}

fn is_invalid_id_complex(divs: &mut HashMap<u32, Vec<usize>>, id: usize) -> bool {
    let len = id.ilog10() + 1;
    get_divisors(divs, len)
        .iter()
        .any(|d| id.is_multiple_of(*d))
}

fn resolve_complex(input: &[Range]) -> usize {
    let mut divs = HashMap::new();
    input
        .iter()
        .flat_map(|Range { first, last }| *first..=*last)
        .filter(|id| is_invalid_id_complex(&mut divs, *id))
        .sum()
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    fn read_input() -> Vec<Range> {
        read_comma_list("example.txt").unwrap()
    }

    #[test]
    fn val_tests() {
        assert!(!is_invalid_id(10));
        assert!(!is_invalid_id(9));
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(38593859));
    }
    #[test]
    fn simple_01() {
        let input = read_input();
        assert_eq!(1227775554, resolve_simple(&input));
    }
    #[test]
    fn complex_01() {
        let input = read_input();
        for i in 2..=3 {
            for j in 1..=3 {
                println!("{i}, {j}, {}", mk_divisor(i, j));
            }
        }
        println!("{:?}", gen_divisors(6));
        assert_eq!(4174379265, resolve_complex(&input));
    }
}
