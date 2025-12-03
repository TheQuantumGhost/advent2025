use std::str::FromStr;
use utilities::{ParseError, read_list};

mod generic;

#[derive(Debug, PartialEq)]
struct Bank {
    joltages: Vec<u32>,
}

impl FromStr for Bank {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .map(|joltages| Bank { joltages })
            .ok_or(Self::Err::Misc)
    }
}

fn main() -> std::io::Result<()> {
    let input = read_list("day03/input.txt")?;
    let simple_val = resolve_simple(&input);
    println!("Simple value: {}", simple_val);
    let complex_val = resolve_complex(&input);
    println!("Complex value: {}", complex_val);

    Ok(())
}

fn resolve_simple(input: &[Bank]) -> usize {
    input.iter().map(|bank| calc_bank(&bank.joltages)).sum()
}

fn find_maximum(bank: &[u32]) -> (u32, usize) {
    let mut val = 0;
    let mut index = 0;
    for (n_index, &n_val) in bank.iter().enumerate() {
        if n_val > val {
            val = n_val;
            index = n_index
        }
    }
    (val, index)
}

fn calc_bank(bank: &[u32]) -> usize {
    let (tens, tens_index) = find_maximum(&bank[0..bank.len() - 1]);
    let (units, _) = find_maximum(&bank[tens_index + 1..]);
    (tens * 10 + units) as usize
}

fn calc_bank_rec(bank: &[u32], digits_left: usize) -> usize {
    if digits_left == 0 {
        return 0;
    }
    let (val, index) = find_maximum(&bank[..bank.len() - digits_left + 1]);
    let rest = calc_bank_rec(&bank[index + 1..], digits_left - 1);

    val as usize * 10usize.pow(digits_left as u32 - 1) + rest
}

fn resolve_complex(input: &[Bank]) -> usize {
    input
        .iter()
        .map(|bank| calc_bank_rec(&bank.joltages, 12))
        .sum()
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    fn read_input() -> Vec<Bank> {
        read_list("example.txt").unwrap()
    }

    #[test]
    fn bank_parsing() {
        assert_eq!(
            Ok(Bank {
                joltages: vec![0, 1, 2, 3]
            }),
            Bank::from_str("0123")
        );
        assert_eq!(Err(ParseError::Misc), Bank::from_str("012c"));
    }

    #[test]
    fn individual_banks() {
        assert_eq!(
            98,
            calc_bank(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])
        );
        assert_eq!(
            89,
            calc_bank(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9])
        );
        assert_eq!(
            78,
            calc_bank(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8])
        );
        assert_eq!(
            92,
            calc_bank(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1])
        );

        assert_eq!(65, calc_bank(&[6, 4, 5, 2]));
    }
    #[test]
    fn individual_banks_rec() {
        assert_eq!(
            98,
            calc_bank_rec(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2)
        );
        assert_eq!(
            89,
            calc_bank_rec(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2)
        );
        assert_eq!(
            78,
            calc_bank_rec(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2)
        );
        assert_eq!(
            92,
            calc_bank_rec(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2)
        );

        assert_eq!(65, calc_bank_rec(&[6, 4, 5, 2], 2));
    }

    #[test]
    fn simple_01() {
        let input = read_input();
        assert_eq!(357, resolve_simple(&input));
    }
    #[test]
    fn complex_01() {
        let input = read_input();
        assert_eq!(3121910778619, resolve_complex(&input));
    }
}
