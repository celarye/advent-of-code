use crate::utils;

pub struct Template {}

pub fn init() -> Result<Template, ()> {
    let mut template = Template {};

    match utils::http::request(0) {
        Ok(input_string) => template.parse_input(&input_string),
        Err(()) => Err(()),
    }
}

impl Template {
    fn parse_input(mut self, input_string: &String) -> Result<Template, ()> {
        Ok(self)
    }

    fn part1(&self) -> String {
        "No Result".to_string()
    }

    fn part2(&self) -> String {
        "No Result".to_string()
    }

    pub fn results(self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Template;

    pub fn test_init() -> Result<Template, ()> {
        let mut template = Template {};

        match fs::read_to_string("input_examples/template.txt") {
            Ok(input_string) => template.parse_input(&input_string),
            Err(err) => {
                eprintln!(
                    "An error occurred while loading the example input: {}",
                    &err
                );
                Err(())
            }
        }
    }

    #[test]
    fn part1_test() {
        let template = test_init().unwrap();
        assert_eq!("Part 1 Example Input Result".to_string(), template.part1())
    }

    #[test]
    fn part2_test() {
        let mut template = test_init().unwrap();
        assert_eq!("Part 2 Example Input Result".to_string(), template.part2())
    }
}
