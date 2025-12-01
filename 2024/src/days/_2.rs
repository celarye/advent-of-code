use crate::utils;

pub struct _2 {
    collection: Vec<Vec<i32>>,
}

pub fn init() -> Result<_2, ()> {
    match utils::http::request(2) {
        Ok(input) => {
            let mut _2 = _2 { collection: vec![] };

            for input_line in input.split("\n") {
                if input_line.len() == 0 {
                    continue;
                }

                let mut input_line_collection = vec![];
                for input in input_line.split(" ") {
                    match input.parse::<i32>() {
                        Ok(number) => input_line_collection.push(number),
                        Err(err) => {
                            eprintln!("An error occurred while parsing the input: {}", &err);
                            return Err(());
                        }
                    }
                }

                _2.collection.push(input_line_collection);
            }

            Ok(_2)
        }
        Err(()) => Err(()),
    }
}

impl _2 {
    fn part1(&self) -> String {
        let mut result = 0;

        for line_collection in self.collection.iter() {
            result += self.is_valid_line_collection(line_collection, None, 0);
        }

        result.to_string()
    }

    fn part2(&self) -> String {
        let mut result = 0;

        for line_collection in self.collection.iter() {
            // TODO: Fix the bug where is_valid_line_collection calls itself with a None p_invalid_number_index, correct the allowed mutation count and move the subtract with overflow check to the method call
            result += self.is_valid_line_collection(line_collection, None, 3);
        }

        result.to_string()
    }

    fn is_valid_line_collection(
        &self,
        line_collection: &Vec<i32>,
        mut p_invalid_number_index: Option<usize>,
        allowed_mut_count: usize,
    ) -> i32 {
        let mut is_valid_line_collection = true;

        let mut p_previous_number: Option<&i32> = None;
        let mut p_increase: Option<bool> = None;

        let mut i_line_collection = line_collection.clone();
        if let Some(invalid_number_index) = p_invalid_number_index {
            if !((invalid_number_index as i8 - allowed_mut_count as i8) < 0) {
                i_line_collection.remove(invalid_number_index - allowed_mut_count);
            }
        }

        for (index, number) in i_line_collection.iter().enumerate() {
            match p_previous_number {
                Some(previous_number) => {
                    let difference = previous_number - number;

                    if !(0 < difference.abs() && difference.abs() < 4) {
                        is_valid_line_collection = false;
                        if let None = p_invalid_number_index {
                            p_invalid_number_index = Some(index);
                        }
                        break;
                    }

                    p_previous_number = Some(number);

                    match p_increase {
                        Some(increase) => {
                            if increase != difference.is_positive() {
                                is_valid_line_collection = false;
                                if let None = p_invalid_number_index {
                                    p_invalid_number_index = Some(index);
                                }
                                break;
                            }
                        }
                        None => p_increase = Some(difference.is_positive()),
                    }
                }
                None => p_previous_number = Some(number),
            }
        }

        if is_valid_line_collection {
            return 1;
        } else {
            match allowed_mut_count {
                0 => 0,
                _ => self.is_valid_line_collection(
                    line_collection,
                    p_invalid_number_index,
                    allowed_mut_count - 1,
                ),
            }
        }
    }

    pub fn results(self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}
