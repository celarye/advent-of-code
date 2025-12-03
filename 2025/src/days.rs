mod _1;
mod _2;

use crate::utils::fs;

pub fn results(days: &[u8]) -> Result<Vec<[String; 2]>, ()> {
    let mut results = vec![];

    for day in days {
        let input = fs::get_input(*day, false)?;

        match day {
            1 => results.push([_1::part1(input.clone()), _1::part2(input)]),
            2 => results.push([_2::part1(input.clone()), _2::part2(input)]),
            _ => (),
        }
    }

    Ok(results)
}
