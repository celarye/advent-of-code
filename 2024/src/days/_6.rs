use std::collections::HashMap;

use crate::utils;

#[derive(Clone)]
pub struct _6 {
    matrix: Vec<Vec<char>>,
    current_location: Vec<usize>,
    direction_change_locations: HashMap<Vec<i16>, u8>,
}

pub fn init() -> Result<_6, ()> {
    let mut _6 = _6 {
        matrix: vec![],
        current_location: vec![],
        direction_change_locations: HashMap::new(),
    };

    match utils::http::request(6) {
        Ok(input_string) => _6.parse_input(&input_string),
        Err(()) => Err(()),
    }
}

impl _6 {
    fn parse_input(mut self, string: &String) -> Result<_6, ()> {
        self.matrix.push(
            (0..(string.split("\n").next().unwrap_or("").len() + 2))
                .map(|_| '+')
                .collect(),
        );

        for (y_index, string_line) in string.split("\n").enumerate() {
            if string_line.len() == 0 {
                continue;
            }

            let mut matrix_line = vec!['+'];

            for (x_index, char) in string_line.chars().enumerate() {
                if char == '^' {
                    self.current_location = vec![y_index + 1, x_index + 1];
                }

                matrix_line.push(char);
            }

            matrix_line.push('+');

            self.matrix.push(matrix_line);
        }

        self.matrix
            .push((0..self.matrix[0].len()).map(|_| '+').collect());

        Ok(self)
    }

    fn part1(&mut self) -> String {
        match self.try_path() {
            Some(result) => result.to_string(),
            None => panic!("the initial try should always resolve to a result"),
        }
    }

    fn part2(&mut self) -> String {
        let mut result = 0;

        let original_start = self.current_location.clone();

        let original_matrix = self.matrix.clone();

        match self.try_path() {
            Some(_) => (),
            None => panic!("the initial try should always resolve to a result"),
        }

        let original_obstacles = self.direction_change_locations.clone();

        for (index_y, row) in self.matrix.clone().clone().iter().enumerate() {
            for (index_x, char) in row.iter().enumerate() {
                if char != &'X' || original_start == vec![index_y, index_x] {
                    continue;
                }

                self.matrix = original_matrix.clone();
                self.current_location = original_start.clone();
                self.direction_change_locations = original_obstacles.clone();

                self.matrix[index_y][index_x] = '#';

                match self.try_path() {
                    Some(_) => (),
                    None => result += 1,
                }
            }
        }

        result.to_string()
    }

    fn try_path(&mut self) -> Option<u16> {
        let mut result = 1;

        let mut direction: Vec<i16> = vec![-1, 0];

        loop {
            self.matrix[self.current_location[0]][self.current_location[1]] = 'X';

            match self.matrix[(self.current_location[0] as i16 + direction[0]) as usize]
                [(self.current_location[1] as i16 + direction[1]) as usize]
            {
                '+' => return Some(result),

                '#' => {
                    let direction_change_count = match self.direction_change_locations.get(&vec![
                        self.current_location[0] as i16 + direction[0],
                        self.current_location[1] as i16 + direction[1],
                        -direction[0],
                        -direction[1],
                    ]) {
                        Some(direction_change_count) => direction_change_count,
                        None => &0,
                    };

                    if direction_change_count == &3 {
                        return None;
                    } else {
                        self.direction_change_locations.insert(
                            vec![
                                self.current_location[0] as i16 + direction[0],
                                self.current_location[1] as i16 + direction[1],
                                -direction[0],
                                -direction[1],
                            ],
                            direction_change_count + 1,
                        );
                    }

                    match direction[..] {
                        [-1, 0] => direction = vec![0, 1],
                        [0, 1] => direction = vec![1, 0],
                        [1, 0] => direction = vec![0, -1],
                        _ => direction = vec![-1, 0],
                    }

                    if self.matrix[(self.current_location[0] as i16 + direction[0]) as usize]
                        [(self.current_location[1] as i16 + direction[1]) as usize]
                        == '#'
                    {
                        match direction[..] {
                            [-1, 0] => direction = vec![0, 1],
                            [0, 1] => direction = vec![1, 0],
                            [1, 0] => direction = vec![0, -1],
                            _ => direction = vec![-1, 0],
                        }
                    }
                }
                _ => (),
            }

            self.current_location = vec![
                (self.current_location[0] as i16 + direction[0]) as usize,
                (self.current_location[1] as i16 + direction[1]) as usize,
            ];

            if self.matrix[self.current_location[0]][self.current_location[1]] == '.' {
                result += 1
            }
        }
    }

    pub fn results(mut self) -> Vec<String> {
        let mut _self = self.clone();
        vec![self.part1(), _self.part2()]
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::_6;

    pub fn test_init() -> Result<_6, ()> {
        let mut _6 = _6 {
            matrix: vec![],
            current_location: vec![],
            direction_change_locations: HashMap::new(),
        };

        match fs::read_to_string("input_examples/day6.txt") {
            Ok(input_string) => _6.parse_input(&input_string),
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
        let mut _6 = test_init().unwrap();
        assert_eq!(41.to_string(), _6.part1())
    }

    #[test]
    fn part2_test() {
        let mut _6 = test_init().unwrap();
        assert_eq!(6.to_string(), _6.part2())
    }
}
