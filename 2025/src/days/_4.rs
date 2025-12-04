use crate::days::_4;

fn part1(input: &str) -> String {
    let mut count = 0;

    let rows: Vec<&str> = input.split("\n").collect();

    for (row, input_line) in rows.iter().enumerate() {
        for (column, char) in input_line.chars().enumerate() {
            if char != '@' {
                continue;
            }

            let mut neighbors = 0;

            let row_len = input_line.len() - 1;

            let column_range = if column == 0 {
                0..=1
            } else if column == row_len {
                row_len - 1..=row_len
            } else {
                column - 1..=column + 1
            };

            if row != 0 {
                neighbors += rows
                    .get(row - 1)
                    .expect("only existing rows should get accessed")
                    .get(column_range.clone())
                    .expect("only existing column ranges should get accessed")
                    .rmatches("@")
                    .count();
            }

            neighbors += rows
                .get(row)
                .expect("only existing rows should get accessed")
                .get(column_range.clone())
                .expect("only existing column ranges should get accessed")
                .rmatches("@")
                .count()
                - 1;

            if row < row_len {
                neighbors += rows
                    .get(row + 1)
                    .expect("only existing rows should get accessed")
                    .get(column_range)
                    .expect("only existing column ranges should get accessed")
                    .rmatches("@")
                    .count();
            }

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let mut count = 0;
    let mut removables = vec![];

    let mut rows = vec![];

    for row in input.split("\n") {
        rows.push(row.to_string());
    }

    loop {
        for (row, input_line) in rows.iter().enumerate() {
            for (column, char) in input_line.chars().enumerate() {
                if char != '@' {
                    continue;
                }

                let mut neighbors = 0;

                let row_len = input_line.len() - 1;

                let column_range = if column == 0 {
                    0..=1
                } else if column == row_len {
                    row_len - 1..=row_len
                } else {
                    column - 1..=column + 1
                };

                if row != 0 {
                    neighbors += rows
                        .get(row - 1)
                        .expect("only existing rows should get accessed")
                        .get(column_range.clone())
                        .expect("only existing column ranges should get accessed")
                        .rmatches("@")
                        .count();
                }

                neighbors += rows
                    .get(row)
                    .expect("only existing rows should get accessed")
                    .get(column_range.clone())
                    .expect("only existing column ranges should get accessed")
                    .rmatches("@")
                    .count()
                    - 1;

                if row < row_len {
                    neighbors += rows
                        .get(row + 1)
                        .expect("only existing rows should get accessed")
                        .get(column_range)
                        .expect("only existing column ranges should get accessed")
                        .rmatches("@")
                        .count();
                }

                if neighbors < 4 {
                    removables.push((row, column));
                    count += 1;
                }
            }
        }

        if removables.is_empty() {
            return count.to_string();
        }

        while let Some(removable) = removables.pop() {
            rows.get_mut(removable.0)
                .unwrap()
                .replace_range(removable.1..removable.1 + 1, ".");
        }
    }
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _4::part1(input),
        2 => _4::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_4, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(4, true).expect("there needs to be an example input file");

        assert_eq!("13", &_4::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(4, true).expect("there needs to be an example input file");

        assert_eq!("43", &_4::part2(&input))
    }
}
