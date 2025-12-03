pub fn part1(_input: String) -> String {
    String::from("Not Implemented")
}

pub fn part2(_input: String) -> String {
    String::from("Not Implemented")
}

#[cfg(test)]
mod tests {
    use crate::{days::template, utils::fs};

    #[test]
    fn part1_test() {
        let input = fs::get_input(0, true).expect("there needs to be an example input file");

        assert_eq!("Part 1 Example Input Result", &template::part1(input))
    }

    #[test]
    fn part2_test() {
        let input = fs::get_input(0, true).expect("there needs to be an example input file");

        assert_eq!("Part 2 Example Input Result", &template::part2(input))
    }
}
