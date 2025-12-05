use grid::Grid;

pub enum BorderType<T> {
    Wrapping,
    Capped,
    Padd(T),
}

pub fn convolution<T>(kernel: Grid<T>, input: &Grid<T>, border: BorderType<T>) -> Grid<T> {
    todo!()
}

type Accessor<T> = FnMut(&Grid<T>, isize, isize) -> &T;
fn wrapping_accessor<T>(grid: &Grid<T>, row: isize, col: isize) -> T
where
    T: Copy,
{
    let (height, width) = grid.size();
    grid.get(
        row.rem_euclid(height as isize),
        col.rem_euclid(width as isize),
    )
    .copied()
    .expect("With the remainder, we should always be in bounds")
}
fn mk_padding<T>(t: T) -> Accessor<T>
where
    T: Copy,
{
    move |grid, row, col| {
        let a = grid.get(row, col).copied().unwrap_or(t);
        a
    }
}

fn calc_kernel<T>(
    kernel: Grid<T>,
    accessor: Accessor<T>,
    grid: &Grid<T>,
    row: usize,
    col: usize,
) -> T
where
    T: std::ops::Mul<Output = T> + std::iter::Sum + Copy,
{
    let (height, width) = kernel.size();
    kernel
        .indexed_into_iter()
        .map(|((k_row, k_col), c)| {
            c * *accessor(
                grid,
                (row + k_row) as isize - height as isize / 2,
                (col + k_col) as isize - width as isize / 2,
            )
        })
        .sum()
}
