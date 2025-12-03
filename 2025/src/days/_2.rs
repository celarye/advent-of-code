use crate::days::_2;

fn part1(input: &str) -> String {
    let mut sum = 0;

    for input_line in input.split(',') {
        let (number_one_str, number_two_str) = input_line
            .split_once('-')
            .expect("the input need to exist out of - seperated numbers");

        let number_one = number_one_str
            .parse::<u64>()
            .expect("the number one str should parse into valid u64");

        let number_two = number_two_str
            .trim_end_matches('\n')
            .parse::<u64>()
            .expect("the number two str should parse into valid u64");

        for number in number_one..=number_two {
            let number_string = number.to_string();

            if number_string.len() % 2 != 0 {
                continue;
            }

            let (number_section_one_string, number_section_two_string) =
                number_string.split_at(number_string.len() / 2);

            if number_section_one_string == number_section_two_string {
                sum += number;
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for input_line in input.split(',') {
        let (number_one_str, number_two_str) = input_line
            .split_once('-')
            .expect("the input need to exist out of - seperated numbers");

        let number_one = number_one_str
            .parse::<u64>()
            .expect("the number one str should parse into valid u64");

        let number_two = number_two_str
            .trim_end_matches('\n')
            .parse::<u64>()
            .expect("the number two str should parse into valid u64");

        for number in number_one..=number_two {
            let number_string = number.to_string();

            for split in 1..=(number_string.len() / 2) {
                if number_string.len() % split != 0 {
                    continue;
                }

                let mut is_match = true;

                let check_segment_str = &number_string[0..split];

                for segment in 1..(number_string.len() / split) {
                    if check_segment_str
                        != &number_string[(segment * split)..(segment * split + split)]
                    {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    sum += number;
                    break;
                }
            }
        }
    }

    sum.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _2::part1(input),
        2 => _2::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_2, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(2, true).expect("there needs to be an example input file");

        assert_eq!("1227775554", &_2::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(2, true).expect("there needs to be an example input file");

        assert_eq!("4174379265", &_2::part2(&input))
    }
}
