use crate::utils;

pub struct _3 {
    input: String,
}

pub fn init() -> Result<_3, ()> {
    match utils::http::request(3) {
        Ok(input) => Ok(_3 {
            input: input.replace("\n", ""),
        }),
        Err(()) => Err(()),
    }
}

impl _3 {
    fn part1(&self) -> String {
        self.sum_of_multiplications(false).to_string()
    }

    fn part2(&self) -> String {
        self.sum_of_multiplications(true).to_string()
    }

    fn sum_of_multiplications(&self, allow_exclusion: bool) -> u32 {
        let mut total_sum = 0;
        let mut p_sum_values = vec![];
        let mut p_number_string = "".to_string();
        let mut p_sum_string = "".to_string();
        let mut include = true;
        let mut p_include_string = "".to_string();

        for char in self.input.chars() {
            if p_sum_string == "mul("
                && ((char == ',' && p_sum_values.len() == 0)
                    || (char == ')' && p_sum_values.len() == 1))
            {
                match p_number_string.parse::<u32>() {
                    Ok(number) => {
                        if number < 1000 {
                            p_sum_values.push(number);
                            p_number_string.clear();

                            if p_sum_values.len() == 2 {
                                if include || !allow_exclusion {
                                    total_sum += u32::from(p_sum_values[0] * p_sum_values[1]);
                                }

                                p_sum_values.clear();
                                p_sum_string.clear();
                            }
                        }
                    }
                    Err(_) => {
                        p_sum_values.clear();
                        p_sum_string.clear();
                    }
                }

                p_number_string.clear();

                continue;
            } else if p_sum_string == "mul(" {
                match char.to_string().parse::<u8>() {
                    Ok(_) => {
                        p_number_string.push(char);
                    }
                    Err(_) => {
                        p_sum_values.clear();
                        p_number_string.clear();
                        p_sum_string.clear();
                    }
                }
            } else {
                let mut i_sum_string = p_sum_string.clone();
                i_sum_string.push(char);

                if "mul(".starts_with(&i_sum_string) {
                    p_sum_string.push(char);
                } else {
                    p_sum_string.clear();
                }

                if allow_exclusion {
                    let mut i_include_string = p_include_string.clone();
                    i_include_string.push(char);

                    if "do()".starts_with(&i_include_string)
                        || "don't()".starts_with(&i_include_string)
                    {
                        p_include_string.push(char);

                        match p_include_string.as_str() {
                            "do()" => {
                                include = true;
                                p_include_string.clear();
                            }
                            "don't()" => {
                                include = false;
                                p_include_string.clear();
                            }
                            _ => (),
                        }
                    } else {
                        p_include_string.clear();
                        continue;
                    }
                }
            }
        }

        total_sum
    }

    pub fn results(self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}
