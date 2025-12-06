use crate::days::_5;

fn part1(input: &str) -> String {
    let mut count = 0;
    let mut ranges = vec![];

    let (ranges_str, items) = input
        .split_once("\n\n")
        .expect("the input should be splitable into ranges and items");

    for range_str in ranges_str.split('\n') {
        if range_str.is_empty() {
            continue;
        }

        let (number_one_str, number_two_str) = range_str
            .split_once('-')
            .expect("the input need to exist out of - seperated numbers");

        let number_one = number_one_str
            .parse::<u64>()
            .expect("the number one str should parse into valid u64");

        let number_two = number_two_str
            .trim_end_matches('\n')
            .parse::<u64>()
            .expect("the number two str should parse into valid u64");

        ranges.push((number_one, number_two));
    }

    for item in items.split('\n') {
        if item.is_empty() {
            continue;
        }

        let number = item
            .parse::<u64>()
            .expect("the number str should parse into valid u64");

        for range in &ranges {
            if range.0 <= number && number <= range.1 {
                count += 1;
                break;
            }
        }
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let mut count = 0;
    let mut ranges: Vec<(u64, u64)> = vec![];

    let (ranges_str, _) = input
        .split_once("\n\n")
        .expect("the input should be splitable into ranges and items");

    for range_str in ranges_str.split('\n') {
        if range_str.is_empty() {
            continue;
        }

        let (number_one_str, number_two_str) = range_str
            .split_once('-')
            .expect("the input need to exist out of - seperated numbers");

        let mut number_one = number_one_str
            .parse::<u64>()
            .expect("the number one str should parse into valid u64");

        let mut number_two = number_two_str
            .trim_end_matches('\n')
            .parse::<u64>()
            .expect("the number two str should parse into valid u64");

        let mut overlap = (false, false);

        for range in ranges.iter_mut() {
            if overlap == (true, true) {
                break;
            }

            if range.0 <= number_one && number_two <= range.1 {
                number_one = 0;
                number_two = 0;
                break;
            }

            if !overlap.0 && range.0 <= number_one && number_one <= range.1 {
                overlap.0 = true;
                number_one = range.0;
                *range = (0, 0);
                continue;
            }

            if !overlap.1 && range.0 <= number_two && number_two <= range.1 {
                overlap.1 = true;
                number_two = range.1;
                *range = (0, 0);
                continue;
            }

            if number_one <= range.0 && range.1 <= number_two {
                *range = (0, 0);
            }
        }

        ranges.push((number_one, number_two));
    }

    for range in ranges {
        if range == (0, 0) {
            continue;
        }

        count += range.1 - range.0 + 1;
    }

    count.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _5::part1(input),
        2 => _5::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_5, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(5, true).expect("there needs to be an example input file");

        assert_eq!("3", &_5::part1(&input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(5, true).expect("there needs to be an example input file");

        assert_eq!("14", &_5::part2(&input))
    }
}
