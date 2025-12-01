pub fn part1(_input: String) -> String {
    "Not Implemented".to_string()
}

pub fn part2(_input: String) -> String {
    "Not Implemented".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{days::_1, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(0, true).expect("there needs to be an example input file");

        assert_eq!("Part 1 Example Input Result".to_string(), _1::part1(input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(0, true).expect("there needs to be an example input file");

        assert_eq!("Part 2 Example Input Result".to_string(), _1::part2(input))
    }
}
