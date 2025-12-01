use std::collections::HashMap;

use crate::utils;

pub struct _5 {
    section_1: HashMap<u16, Vec<u16>>,
    section_2: Vec<Vec<u16>>,
}

pub fn init() -> Result<_5, ()> {
    let mut _5 = _5 {
        section_1: HashMap::new(),
        section_2: vec![],
    };

    match utils::http::request(5) {
        Ok(input_string) => _5.parse_input(&input_string),
        Err(()) => Err(()),
    }
}

impl _5 {
    fn parse_input(mut self, string: &String) -> Result<_5, ()> {
        let sections = string.split("\n\n").collect::<Vec<&str>>();

        for section_line in sections[0].split("\n") {
            let mut numbers = vec![];

            for number_str in section_line.split("|") {
                if number_str.len() == 0 {
                    continue;
                }

                match number_str.parse::<u16>() {
                    Ok(number) => numbers.push(number),
                    Err(err) => {
                        eprintln!("An error occurred while parsing the input: {}", &err);
                        return Err(());
                    }
                }
            }

            match self.section_1.get(&numbers[0]) {
                Some(entries) => {
                    let mut new_entries = entries.clone();
                    new_entries.push(numbers[1]);

                    self.section_1.insert(numbers[0], new_entries);
                }
                None => {
                    self.section_1.insert(numbers[0], vec![numbers[1]]);
                }
            };
        }

        for section_line in sections[1].split("\n") {
            let mut values = vec![];

            for value in section_line.split(",") {
                if value.len() == 0 {
                    continue;
                }

                match value.parse::<u16>() {
                    Ok(number) => values.push(number),
                    Err(err) => {
                        eprintln!("An error occurred while parsing the input: {}", &err);
                        return Err(());
                    }
                }
            }

            self.section_2.push(values);
        }

        Ok(self)
    }

    fn part1(&self) -> String {
        let mut result = 0;

        for array in self.section_2.iter() {
            if array.len() == 0 {
                continue;
            }

            if self.is_valid(&array).0 {
                result += array.get(array.len() / 2).unwrap();
            }
        }

        result.to_string()
    }

    fn part2(&mut self) -> String {
        let mut result = 0;

        for array in self.section_2.iter() {
            if array.len() == 0 {
                continue;
            }

            let (is_valid, p_bad_index, p_number_index, p_number) = self.is_valid(&array);

            if !is_valid {
                let mut c_array = array.clone();

                c_array.remove(p_bad_index.unwrap());
                c_array.insert(p_number_index.unwrap(), p_number.unwrap());

                loop {
                    let (c_is_valid, c_p_bad_index, c_p_number_index, c_p_number) =
                        self.is_valid(&c_array);

                    if c_is_valid {
                        break;
                    }

                    c_array.remove(c_p_bad_index.unwrap());
                    c_array.insert(c_p_number_index.unwrap(), c_p_number.unwrap());
                }

                result += c_array.get(c_array.len() / 2).unwrap();
            }
        }

        result.to_string()
    }

    fn is_valid(&self, array: &Vec<u16>) -> (bool, Option<usize>, Option<usize>, Option<u16>) {
        let mut i_array = array.clone();

        for (index, number) in array.iter().enumerate().rev() {
            i_array.remove(index);

            for compared_number in self.section_1.get(number).unwrap_or(&vec![]).iter() {
                for (i_index, i_number) in i_array.iter().enumerate() {
                    if i_number == compared_number {
                        return (false, Some(i_index), Some(index), Some(*compared_number));
                    }
                }
            }
        }

        (true, None, None, None)
    }

    pub fn results(mut self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::_5;

    pub fn test_init() -> Result<_5, ()> {
        let mut _5 = _5 {
            section_1: HashMap::new(),
            section_2: vec![],
        };

        match fs::read_to_string("input_examples/day5.txt") {
            Ok(input_string) => _5.parse_input(&input_string),
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
        let _5 = test_init().unwrap();
        assert_eq!(143.to_string(), _5.part1())
    }

    #[test]
    fn part2_test() {
        let mut _5 = test_init().unwrap();
        assert_eq!(123.to_string(), _5.part2())
    }
}
