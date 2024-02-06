use std::ops::Index;
use std::str::Bytes;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    // BFS from the start, maintain a list of visited in the current frontier dist, remember the highest place visited twice in a frontier dist
    let grid: Vec<_> = _input.split("\n").collect();
    let (start_row, start_column) = grid
        .iter()
        .enumerate()
        .map(|(row, &line)| (row, line.find("S")))
        .find(|(row, pos)| pos.is_some())
        .unwrap();
    let start_column = start_column.unwrap();
    let grid  = grid.into_iter().map(|line| line.as_bytes().to_owned()).collect_vec();
    let (left, right) = [
        (start_row + 1, start_column),
        (start_row.saturating_sub(1), start_column),
        (start_row, start_column + 1),
        (start_row, start_column.saturating_sub(1)),
    ]
    .iter()
    .inspect(|x| println!("{x:?}"))
    .map(|&dir| expand(&grid, dir))
    .inspect(|x| println!("{x:?}"))
    .filter_map(|x| Some(x.into_iter().filter(|&y|y==Some((start_row,start_column)))))
    // .inspect(|x| println!("{x:?}"))a
    .take(2)
    .collect_tuple()
    .expect("Failed to make tuple");
    Ok(1.to_string())
}

fn expand(grid: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> [Option<(usize, usize)>; 2]

{
    match grid.index(row).index(col) {
        b'|' => [if col < grid[0].len() {Some((row, col + 1))} else {None}, if col > 0 {Some((row, col - 1))} else {None}],
        b'-' => [if row < grid.len() {Some((row + 1, col))} else {None}, if row > 0 {Some((row - 1, col))} else {None}],
        b'L' => [if col < grid[0].len() {Some((row, col + 1))} else {None}, if row > 0 {Some((row - 1, col))} else {None}],
        b'J' => [if col > 0 {Some((row, col - 1))} else {None}, if row > 0 {Some((row - 1, col))} else {None}],
        b'7' => [if col > 0 {Some((row, col - 1))} else {None}, if row < grid.len() {Some((row + 1, col))} else {None}],
        b'F' => [if col < grid[0].len() {Some((row, col + 1))} else {None}, if row < grid.len() {Some((row + 1, col))} else {None}],
        b'.' => [None, None],
        b'S' => panic!("Cannot expand the start"),
        _ => panic!("Unknown symbol? Did you expand newline"),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
