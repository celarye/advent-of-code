use std::{fs, path::PathBuf};

pub fn get_input(day: u8, example: bool) -> Result<String, ()> {
    let path = if example {
        println!("Loading the example input from its file...");

        PathBuf::from(format!("./example-inputs/{}", &day.to_string()))
    } else {
        println!("Loading the input from its file...");

        PathBuf::from(format!("./inputs/{}", &day.to_string()))
    };

    match fs::read_to_string(&path) {
        Ok(input) => Ok(input),
        Err(err) => {
            eprintln!("An error occured while loading the input file: {err}");
            Err(())
        }
    }
}
