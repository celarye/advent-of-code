use crate::days::_6;

fn part1(input: &str) -> String {
    let mut solution: u64 = 0;
    let mut numbers: Vec<Vec<u64>> = vec![];

    let mut input_lines = input.split('\n').rev().skip(1);

    let operation_symbols: Vec<&str> = input_lines
        .next()
        .expect("the input should contain a list of operation symbols")
        .split_whitespace()
        .collect();

    for input_line in input_lines {
        for (index, number_str) in input_line.split_whitespace().enumerate() {
            if numbers.get(index).is_none() {
                numbers.push(vec![]);
            }

            numbers
                .get_mut(index)
                .expect("numbers should contain the right amount of vecs")
                .push(
                    number_str
                        .parse::<u64>()
                        .expect("the number str should parse into valid u64"),
                );
        }
    }

    for (index, operation_symbol) in operation_symbols.iter().enumerate() {
        match *operation_symbol {
            "+" => {
                solution += numbers
                    .get(index)
                    .expect("operation symbols length should match the input lines - 1")
                    .iter()
                    .sum::<u64>();
            }
            "*" => {
                solution += numbers
                    .get(index)
                    .expect("operation symbols length should match the input lines - 1")
                    .iter()
                    .product::<u64>();
            }
            _ => unreachable!("the input should only have + and * operation symbols"),
        }
    }

    solution.to_string()
}

fn part2(input: &str) -> String {
    let mut solution: u64 = 0;
    let mut numbers: Vec<String> = vec![];

    let mut input_lines = input.split('\n').rev().skip(1);

    let operation_symbols = input_lines
        .next()
        .expect("the input should contain a list of operation symbols");

    let input_line_length = operation_symbols.len();

    for input_line in input_lines {
        for index in 0..input_line_length {
            if numbers.get(index).is_none() {
                numbers.push(String::new());
            }

            let number_str = input_line
                .chars()
                .nth(index)
                .expect("index on &str chars iterator should return a valid char");

            if number_str == ' ' {
                continue;
            }

            numbers
                .get_mut(index)
                .expect("numbers should contain the right amount of strings")
                .push(number_str);
        }
    }

    let mut column_solution = 0;
    let mut last_operation_symbol = ' ';

    for (index, number_string) in numbers.iter().enumerate() {
        let number_string = number_string.chars().rev().collect::<String>();

        if number_string.is_empty() {
            solution += column_solution;
            column_solution = 0;
            continue;
        }

        let number = number_string
            .parse::<u64>()
            .expect("the number string should parse into valid u64");

        let operation_symbol = match operation_symbols
            .get(index..=index)
            .expect("operation symbols str should have a valid char at the index")
        {
            "+" => {
                last_operation_symbol = '+';
                '+'
            }
            "*" => {
                last_operation_symbol = '*';
                '*'
            }
            _ => last_operation_symbol,
        };

        match operation_symbol {
            '+' => {
                column_solution += number;
            }
            '*' => {
                if column_solution == 0 {
                    column_solution += 1;
                }

                column_solution *= number;
            }
            _ => unreachable!("the operation symbol should either be a + or * char"),
        }
    }

    solution += column_solution;

    solution.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _6::part1(input),
        2 => _6::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_6, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(6, true).expect("there needs to be an example input file");

        assert_eq!("4277556", &_6::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(6, true).expect("there needs to be an example input file");

        assert_eq!("3263827", &_6::part2(&input))
    }
}
