pub use crate::errors::ParseError;
pub use grid::Grid;
use std::{fs::read_to_string, str::FromStr};
pub use traits::FromChar;

mod errors;
pub mod parsers;
pub mod structures;
mod traits;

pub type IOResult<T> = std::io::Result<T>;

pub fn read_bulk<T>(path: &str) -> IOResult<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    read_to_string(path)?.parse().map_err(std::io::Error::other)
}

pub fn read_list<T>(path: &str) -> IOResult<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    read_to_string(path)?
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(std::io::Error::other)
}
pub fn read_comma_list<T>(path: &str) -> IOResult<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    read_to_string(path)?
        .split(',')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(std::io::Error::other)
}

pub fn read_list_str<T>(text: &str) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    text.split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<_>, _>>()
}

pub fn read_bundled<T>(path: &str) -> IOResult<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    read_to_string(path)?
        .split("\n\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim().parse::<T>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(std::io::Error::other)
}

pub fn read_grid<T>(path: &str) -> IOResult<Grid<T>>
where
    T: FromChar,
    <T as FromChar>::Err: std::error::Error + Sync + Send + 'static,
{
    let txt = read_to_string(path)?;
    let cols = txt.find('\n').unwrap();
    let vec = txt
        .split('\n')
        .filter(|&s| !s.is_empty())
        .flat_map(|s| s.chars())
        .map(|c| <T>::from_char(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(std::io::Error::other)?;
    Ok(Grid::from_vec(vec, cols))
}
