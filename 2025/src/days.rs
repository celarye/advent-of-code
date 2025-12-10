mod _1;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;
mod _7;

use std::time::Instant;

use crate::utils::fs;

pub fn results(days: &[u8]) -> Result<(), ()> {
    let global_start = Instant::now();

    for day in days {
        println!("\nDay {day}:\n");

        println!("Loading the input from its file...\n");
        let input = fs::get_input(*day, false)?;

        for part in 1u8..=2 {
            print!("  Part {part}: ");

            let start = Instant::now();

            match day {
                1 => print!("{}", _1::result(part, &input)),
                2 => print!("{}", _2::result(part, &input)),
                3 => print!("{}", _3::result(part, &input)),
                4 => print!("{}", _4::result(part, &input)),
                5 => print!("{}", _5::result(part, &input)),
                6 => print!("{}", _6::result(part, &input)),
                7 => print!("{}", _7::result(part, &input)),
                _ => unimplemented!("this day is not yet available"),
            }

            println!(" [Elapsed Time: {}μs]", start.elapsed().as_micros());
        }
    }

    println!(
        "\nTotal Elapsed Time: {}ms",
        global_start.elapsed().as_millis()
    );

    Ok(())
}
