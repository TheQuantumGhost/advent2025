use derive_from_char::FromChar;
use utilities::{Grid, read_grid};

#[derive(FromChar, Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    #[c = '.']
    Empty,
    #[c = '@']
    Roll,
}
impl Cell {
    fn is_roll(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Roll => true,
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = read_grid("day04/input.txt")?;
    let simple_val = resolve_simple(&input);
    println!("Simple value: {}", simple_val);
    let complex_val = resolve_complex(&input);
    println!("Complex value: {}", complex_val);

    Ok(())
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn resolve_simple(input: &Grid<Cell>) -> usize {
    input
        .indexed_iter()
        .filter(|((row, col), _)| is_accessible(input, *row, *col))
        .count()
}

fn is_accessible(grid: &Grid<Cell>, row: usize, col: usize) -> bool {
    if grid.get(row, col).is_some_and(|cell| cell.is_roll()) {
        NEIGHBORS
            .iter()
            .filter_map(|(row_delta, col_delta)| {
                grid.get(row as isize + row_delta, col as isize + col_delta)
            })
            .filter(|cell| cell.is_roll())
            .count()
            < 4
    } else {
        false
    }
}

fn iterate(grid: &mut Grid<Cell>) -> usize {
    let tmp_grid = grid.clone();
    grid.indexed_iter_mut()
        .filter(|((row, col), _)| is_accessible(&tmp_grid, *row, *col))
        .map(|(_, cell)| *cell = Cell::Empty)
        .count()
}

fn resolve_complex(input: &Grid<Cell>) -> usize {
    let mut grid = input.clone();
    let mut out = 0;
    let mut delta = iterate(&mut grid);

    while delta > 0 {
        out += delta;
        delta = iterate(&mut grid);
    }

    out
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    fn read_input() -> Grid<Cell> {
        read_grid("example.txt").unwrap()
    }

    #[test]
    fn simple_01() {
        let input = read_input();
        assert_eq!(13, resolve_simple(&input));
    }
    #[test]
    fn complex_01() {
        let input = read_input();
        assert_eq!(43, resolve_complex(&input));
    }
}
