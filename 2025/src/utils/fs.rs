use std::{fs, path::PathBuf};

pub fn get_input(day: u8, example: bool) -> Result<String, ()> {
    let path = if example {
        PathBuf::from(format!("./example-inputs/{day}"))
    } else {
        PathBuf::from(format!("./inputs/{day}"))
    };

    match fs::read_to_string(&path) {
        Ok(input) => Ok(input),
        Err(err) => {
            eprintln!("An error occured while loading the input file: {err}");
            Err(())
        }
    }
}
