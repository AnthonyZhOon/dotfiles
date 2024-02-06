use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let table = _input.split_inclusive('\n').map(|s| s.trim_start().chars().collect_vec()).collect_vec();
    // Walk some pointers over this shit
    let mut total = 0;
    let mut i = 0;
    let (rows, cols) = (table.len(), table.first().unwrap().len() - 1);
    let max_len = rows * cols;
    while i < max_len {
        let row: &Vec<char> = table[i / cols].as_ref();
        let col = i % cols;
        let (prev_row, next_row) = ((i / cols).checked_sub(1).unwrap_or(cols + 1) , i / cols + 1);
        match row[col] {
            x if x.is_ascii_digit() => {
                let mut end = col + 1; 
                let mut is_valid = false;
                if col > 0  {
                    if prev_row <= cols && is_symbol(table[prev_row][col -1]) {
                        is_valid = true;
                    } 
                    if row[col-1] != '.' && !row[col-1].is_ascii_digit() {
                        is_valid = true;
                    }
                    if next_row < cols && is_symbol(table[next_row][col-1]){
                        is_valid = true
                    }
                }
                
                for j in col..cols {
                    if !row[j].is_ascii_digit() {
                        end = j;
                        break;
                    } 
                    if next_row < cols && is_symbol(table[next_row][j]) {
                        is_valid = true
                    }
                    if prev_row <= cols && is_symbol(table[prev_row][j]) {
                        is_valid = true
                    }
                } 
                if end > 0  {
                    if prev_row <= cols && is_symbol(table[prev_row][end]) {
                        is_valid = true;
                    } 
                    if next_row < cols && is_symbol(table[next_row][end]){
                        is_valid = true
                    }
                }
                if is_symbol(row[end]) {
                    is_valid = true;
                }
            let num = row[col..end].iter().collect::<String>().parse::<usize>().unwrap();
            if is_valid {total += row[col..end].iter().collect::<String>().parse::<usize>().unwrap()} else {
                println!("{num} false at {} {col}", i/cols)
            }
            i = i + (end - col);
            },
            _ => i += 1,
        }
    }; 
    Ok(format!("{total}"))
}

fn is_symbol(s: char) -> bool {
    match s {
        '.' => false,
        x if x.is_ascii_digit() => false,
        _ => true,
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
