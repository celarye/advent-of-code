use crate::days::_1;

fn part1(input: &str) -> String {
    let mut value = 50;
    let mut match_count = 0;

    for input_line in input.split('\n') {
        if input_line.is_empty() {
            continue;
        }

        let (direction, count_str) = input_line.split_at(1);

        let mut count = count_str
            .parse::<i32>()
            .expect("the count str should parse into valid i32");

        if direction == "L" {
            count *= -1;
        }

        value += count;

        if value % 100 == 0 {
            match_count += 1;
        }
    }

    match_count.to_string()
}

fn part2(input: &str) -> String {
    let mut value = 50;
    let mut match_count = 0;

    for input_line in input.split('\n') {
        if input_line.is_empty() {
            continue;
        }

        let (direction, count_str) = input_line.split_at(1);

        let mut count = count_str
            .parse::<i32>()
            .expect("the count str should parse into valid 32");

        if direction == "L" {
            count *= -1;
        }

        match_count += (count / 100).abs();

        if (value + (count % 100)) % 100 == 0
            || value % 100 != 0
                && (value / 100 != (value + (count % 100)) / 100
                    || value.is_positive() != (value + (count % 100)).is_positive())
        {
            match_count += 1;
        }

        value += count;
    }

    match_count.to_string()
}

pub fn result(part: u8, input: &str) -> String {
    match part {
        1 => _1::part1(input),
        2 => _1::part2(input),
        _ => unimplemented!("this day only has two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::_1, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(1, true).expect("there needs to be an example input file");

        assert_eq!("3", &_1::part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(1, true).expect("there needs to be an example input file");

        assert_eq!("6", &_1::part2(&input));
    }
}
