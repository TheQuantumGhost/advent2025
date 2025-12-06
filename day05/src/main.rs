fn main() -> std::io::Result<()> {
    let input = ();
    let simple_val = resolve_simple(&input);
    println!("Simple value: {}", simple_val);
    let complex_val = resolve_complex(&input);
    println!("Complex value: {}", complex_val);

    Ok(())
}

fn resolve_simple(input: &()) -> usize {
    0
}

fn resolve_complex(input: &()) -> usize {
    0
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    fn read_input() -> () {
        todo!()
    }

    #[test]
    fn simple_01() {
        let input = read_input();
        assert_eq!(0, resolve_simple(&input));
    }
    #[test]
    fn complex_01() {
        let input = read_input();
        assert_eq!(0, resolve_complex(&input));
    }
}
