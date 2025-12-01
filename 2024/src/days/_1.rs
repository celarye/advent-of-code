use std::{collections::HashMap, iter::zip};

use crate::utils;

pub struct _1 {
    collection_1: Vec<i32>,
    collection_2: Vec<i32>,
}

pub fn init() -> Result<_1, ()> {
    match utils::http::request(1) {
        Ok(input) => {
            let mut _1 = _1 {
                collection_1: vec![],
                collection_2: vec![],
            };

            for input_line in input.split("\n") {
                let inputs: Vec<&str> = input_line.split("   ").collect();
                if inputs.len() != 2 {
                    continue;
                }

                match inputs[0].parse::<i32>() {
                    Ok(number) => _1.collection_1.push(number),
                    Err(err) => {
                        eprintln!("An error occurred while parsing the input: {}", &err);
                        return Err(());
                    }
                }

                match inputs[1].parse::<i32>() {
                    Ok(number) => _1.collection_2.push(number),
                    Err(err) => {
                        eprintln!("An error occurred while parsing the input: {}", &err);
                        return Err(());
                    }
                }
            }

            Ok(_1)
        }
        Err(()) => Err(()),
    }
}

impl _1 {
    fn part1(&mut self) -> String {
        let mut result = 0;

        self.collection_1.sort();
        self.collection_2.sort();

        for (number_1, number_2) in zip(self.collection_1.iter(), self.collection_2.iter()) {
            result += (number_1 - number_2).abs()
        }

        return result.to_string();
    }

    fn part2(&self) -> String {
        let mut result = 0;
        let mut collection_2_map = HashMap::<i32, i32>::new();

        for number in self.collection_2.iter() {
            match collection_2_map.get(&number) {
                Some(collection_2_entry) => {
                    collection_2_map.insert(*number, collection_2_entry + 1)
                }
                None => collection_2_map.insert(*number, 1),
            };
        }

        for number in self.collection_1.iter() {
            let collection_1_entry = number;
            result += collection_1_entry
                * collection_2_map
                    .get(&collection_1_entry)
                    .unwrap_or_else(|| &0)
        }

        return result.to_string();
    }

    pub fn results(mut self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}
