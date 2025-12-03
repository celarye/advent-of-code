use crate::days::_3;

fn part1(input: &str) -> String {
    let mut sum = 0;

    for input_line in input.split('\n') {
        if input_line.is_empty() {
            continue;
        }

        let mut largest_numbers = vec![0; 2];

        for (il_index, char) in input_line.chars().enumerate() {
            if largest_numbers[1] == 9 {
                break;
            }

            let char_number = char
                .to_digit(10)
                .expect("the char should parse into valid u8") as u8;

            let mut is_match = false;

            for (ln_index, largest_number) in largest_numbers.iter_mut().enumerate() {
                if is_match {
                    *largest_number = 0;
                    continue;
                }

                if char_number > *largest_number
                    && (2 - ln_index + 1) <= (input_line.len() - il_index + 1)
                {
                    *largest_number = char_number;
                    is_match = true;
                }
            }
        }

        sum += largest_numbers
            .into_iter()
            .map(|c| c.to_string())
            .collect::<String>()
            .parse::<u16>()
            .expect("the combined numbers should parse into valid u64");
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for input_line in input.split('\n') {
        if input_line.is_empty() {
            continue;
        }

        let mut largest_numbers = vec![0; 12];

        for (il_index, char) in input_line.chars().enumerate() {
            if largest_numbers[11] == 9 {
                break;
            }

            let char_number = char
                .to_digit(10)
                .expect("the char should parse into valid u8") as u8;

            let mut is_match = false;

            for (ln_index, largest_number) in largest_numbers.iter_mut().enumerate() {
                if is_match {
                    *largest_number = 0;
                    continue;
                }

                if char_number > *largest_number
                    && (12 - ln_index + 1) <= (input_line.len() - il_index + 1)
                {
                    *largest_number = char_number;
                    is_match = true;
                }
            }
        }

        sum += largest_numbers
            .into_iter()
            .map(|c| c.to_string())
            .collect::<String>()
            .parse::<u64>()
            .expect("the combined numbers should parse into valid u64");
    }

    sum.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _3::part1(input),
        2 => _3::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_3, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(3, true).expect("there needs to be an example input file");

        assert_eq!("357", &_3::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(3, true).expect("there needs to be an example input file");

        assert_eq!("3121910778619", &_3::part2(&input))
    }
}
