use std::ops::Neg;

use crate::utils;

pub struct _4 {
    _2d_array: Vec<Vec<char>>,
    part1_start_indexes: Vec<Vec<usize>>,
    part2_start_indexes: Vec<Vec<usize>>,
}

pub fn init() -> Result<_4, ()> {
    let mut _4 = _4 {
        _2d_array: vec![],
        part1_start_indexes: vec![],
        part2_start_indexes: vec![],
    };

    match utils::http::request(4) {
        Ok(input) => {
            for _ in 0..3 {
                _4._2d_array
                    .push((0..146).map(|_| '+').collect::<Vec<char>>());
            }

            for (index1, input_line) in input.split("\n").enumerate() {
                if input_line.len() == 0 {
                    continue;
                }

                let mut input_line_collection = vec!['+', '+', '+'];
                for (index2, char) in input_line.chars().enumerate() {
                    match char {
                        'X' => _4.part1_start_indexes.push(vec![index1 + 3, index2 + 3]),
                        'A' => _4.part2_start_indexes.push(vec![index1 + 3, index2 + 3]),
                        _ => (),
                    }

                    input_line_collection.push(char);
                }
                input_line_collection.append(&mut vec!['+', '+', '+']);

                _4._2d_array.push(input_line_collection);
            }

            for _ in 0..3 {
                _4._2d_array
                    .push((0..146).map(|_| '+').collect::<Vec<char>>());
            }

            Ok(_4)
        }
        Err(()) => Err(()),
    }
}

impl _4 {
    fn part1(&self) -> String {
        let mut result = 0;

        let check_range: Vec<i16> = vec![-1, 0, 1];

        for start_index in self.part1_start_indexes.iter() {
            for c_index_x in check_range.iter() {
                for c_index_y in check_range.iter() {
                    if self._2d_array[(start_index[0] as i16 + c_index_x) as usize]
                        [(start_index[1] as i16 + c_index_y) as usize]
                        != 'M'
                    {
                        continue;
                    }

                    if self._2d_array[(start_index[0] as i16 + c_index_x * 2) as usize]
                        [(start_index[1] as i16 + c_index_y * 2) as usize]
                        != 'A'
                    {
                        continue;
                    }

                    if self._2d_array[(start_index[0] as i16 + c_index_x * 3) as usize]
                        [(start_index[1] as i16 + c_index_y * 3) as usize]
                        != 'S'
                    {
                        continue;
                    }

                    result += 1;
                }
            }
        }

        result.to_string()
    }

    fn part2(&self) -> String {
        let mut result = 0;

        let diagonals: Vec<Vec<i16>> = vec![vec![-1, 1], vec![1, 1]];

        for start_index in self.part2_start_indexes.iter() {
            let mut valid = true;

            for c_indexes in diagonals.iter() {
                let char_1 = self._2d_array[(start_index[0] as i16 + c_indexes[0]) as usize]
                    [(start_index[1] as i16 + c_indexes[1]) as usize];

                let char_2 = self._2d_array[(start_index[0] as i16 + c_indexes[0].neg()) as usize]
                    [(start_index[1] as i16 + c_indexes[1].neg()) as usize];

                if 'M' == char_1 && 'S' == char_2 || 'M' == char_2 && 'S' == char_1 {
                    continue;
                }

                valid = false;
                break;
            }

            if valid {
                result += 1;
            }
        }

        result.to_string()
    }

    pub fn results(self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}
