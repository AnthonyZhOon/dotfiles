use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let table = _input
        .split_inclusive('\n')
        .map(|s| s.trim_start().chars().collect_vec())
        .collect_vec();
    // Walk some pointers over this shit
    let mut i = 0;
    let (rows, cols) = (table.len(), table.first().unwrap().len() - 1);
    let max_len = rows * cols;
    let mut gears = HashMap::<(usize, usize), (usize, usize)>::new();
    while i < max_len {
        let row: &Vec<char> = table[i / cols].as_ref();
        let col = i % cols;
        let (prev_row, next_row) = ((i / cols).checked_sub(1).unwrap_or(rows + 1), i / cols + 1);
        match row[col] {
            x if x.is_ascii_digit() => {
                let mut end = col;
                let mut found_at = Vec::<(usize, usize)>::new();
                if col > 0 {
                    if prev_row <= rows && if_gear(table[prev_row][col - 1]) {
                        found_at.push((prev_row, col - 1))
                    }
                    if row[col - 1] != '.' && !row[col - 1].is_ascii_digit() {
                        found_at.push((i / cols, col - 1))
                    }
                    if next_row < rows && if_gear(table[next_row][col - 1]) {
                        found_at.push((next_row, col - 1))
                    }
                }

                for j in col..cols {
                    if j < cols - 1 && next_row < rows && if_gear(table[next_row][j]) {
                        found_at.push((next_row, j))
                    }
                    if j < cols - 1 && prev_row <= rows && if_gear(table[prev_row][j]) {
                        found_at.push((prev_row, j))
                    }
                    if !row[j].is_ascii_digit() {
                        break;
                    }
                    end = j + 1;
                }

                // if end > col && end < cols - 1 {
                //     if prev_row <= rows && if_gear(table[prev_row][end]) {
                //         found_at.push((prev_row, end))
                //     }
                //     if next_row < rows && if_gear(table[next_row][end]) {
                //         found_at.push((next_row, end))
                //     }
                // }
                if end < cols - 1 && if_gear(row[end]) {
                    found_at.push((i / cols, end))
                }

                if !found_at.is_empty() {
                    for (x, y) in found_at {
                        match gears.get_mut(&(x, y)) {
                            None => {
                                let num = row[col..end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap();
                                gears.insert((x, y), (1, num));
                            }
                            Some((count @ 0..=1, power)) => {
                                let num = row[col..end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap();
                                *count += 1;
                                *power *= num;
                            }
                            _ => (),
                        }
                    }
                }
                i = i + (end - col);
            }
            _ => i += 1,
        }
    }
    let total: usize = gears
        .values()
        .filter_map(|&(x, ratio)| if x == 2 { Some(ratio) } else { None })
        .sum();
    Ok(total.to_string())
}

fn if_gear(s: char) -> bool {
    match s {
        '*' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }

}
