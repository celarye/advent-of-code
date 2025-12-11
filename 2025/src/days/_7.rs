use crate::days::_7;

fn part1(input: &str) -> String {
    let mut sum = 0;
    let mut input_lines = input.lines();

    let first_input_line = input_lines.next().expect("the input should have lines");

    let input_line_length = first_input_line.len();

    let mut values = Vec::with_capacity(input_line_length);

    values.push(
        first_input_line
            .chars()
            .position(|c| c == 'S')
            .expect("the first input line should contain the character S"),
    );

    let mut skip = true;
    for input_line in input_lines {
        if skip {
            skip = false;
            continue;
        }

        let mut new_values = Vec::with_capacity(input_line_length);

        for value in values {
            if input_line.as_bytes()[value] as char == '^' {
                sum += 1;

                if !new_values.contains(&(value - 1)) {
                    new_values.push(value - 1);
                }

                if !new_values.contains(&(value + 1)) {
                    new_values.push(value + 1);
                }
            } else if !new_values.contains(&value) {
                new_values.push(value);
            }
        }

        skip = true;

        values = new_values;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 1;

    let mut input_lines = input.lines();

    let first_input_line = input_lines.next().expect("the input should have lines");

    let input_line_length = first_input_line.len();

    let mut values = vec![0; input_line_length];

    values.insert(
        first_input_line
            .chars()
            .position(|c| c == 'S')
            .expect("the first input line should contain the character S"),
        1,
    );

    let mut skip = true;
    for input_line in input_lines {
        if skip {
            skip = false;
            continue;
        }

        let mut new_values = vec![0u64; input_line_length];

        for (index, value) in values.iter().enumerate() {
            if *value == 0 {
                continue;
            }

            if input_line.as_bytes()[index] as char == '^' {
                sum += value;

                new_values[index - 1] += value;

                new_values[index + 1] += value;
            } else {
                new_values[index] += value;
            }
        }

        values = new_values;
    }

    sum.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _7::part1(input),
        2 => _7::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_7, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(7, true).expect("there needs to be an example input file");

        assert_eq!("21", &_7::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(7, true).expect("there needs to be an example input file");

        assert_eq!("40", &_7::part2(&input))
    }
}
