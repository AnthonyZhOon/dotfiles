use std::ops::Index;

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
        .find(|(_, pos)| pos.is_some())
        .unwrap();
    let start_column = start_column.unwrap();
    let grid = grid
        .into_iter()
        .map(|line| line.as_bytes().to_owned())
        .collect_vec();
    let (mut left, mut right) = [
        (start_row + 1, start_column),
        (start_row.saturating_sub(1), start_column),
        (start_row, start_column + 1),
        (start_row, start_column.saturating_sub(1)),
    ]
    .iter()
    .filter_map(|&x| {
        if expand(&grid, x)
            .into_iter()
            .any(|y| y == Some((start_row, start_column)))
        {
            Some(x)
        } else {
            None
        }
    })
    // .inspect(|x| println!("{x:?}"))
    .collect_tuple()
    .expect("Failed to make tuple");
    let (mut prev_left, mut prev_right) = ((start_row, start_column), (start_row, start_column));
    let mut dist = 1;
    while left != right {
        dist += 1;
        (left, prev_left) = (expand(&grid, left)
            .iter()
            .find(|&x| x.is_some_and(|x| x != prev_left))
            .unwrap()
            .unwrap(), left);
        (right, prev_right) = (expand(&grid, right)
            .iter()
            .find(|&x| x.is_some_and(|x| x != prev_right))
            .unwrap()
            .unwrap(), right);
    }
    Ok(dist.to_string())
}

fn expand(grid: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> [Option<(usize, usize)>; 2] {
    match grid.index(row).index(col) {
        b'|' => [
            if row < grid.len() - 1 {
                Some((row + 1, col))
            } else {
                None
            },
            if row > 0 { Some((row - 1, col)) } else { None },
        ],
        b'-' => [
            if col < grid[0].len() - 1 {
                Some((row, col + 1))
            } else {
                None
            },
            if col > 0 { Some((row, col - 1)) } else { None },
        ],
        b'L' => [
            if col < grid[0].len() - 1 {
                Some((row, col + 1))
            } else {
                None
            },
            if row > 0 { Some((row - 1, col)) } else { None },
        ],
        b'J' => [
            if col > 0 { Some((row, col - 1)) } else { None },
            if row > 0 { Some((row - 1, col)) } else { None },
        ],
        b'7' => [
            if col > 0 { Some((row, col - 1)) } else { None },
            if row < grid.len() - 1 {
                Some((row + 1, col))
            } else {
                None
            },
        ],
        b'F' => [
            if col < grid[0].len() - 1 {
                Some((row, col + 1))
            } else {
                None
            },
            if row < grid.len() - 1 {
                Some((row + 1, col))
            } else {
                None
            },
        ],
        b'.' | b'S' => [None, None],
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
        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[test]
    fn test_small() -> miette::Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
