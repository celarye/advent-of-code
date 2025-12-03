// TODO:
// - Cli parameters: Day range, cache
// - Benchmark support

use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

mod days;
mod utils;

use utils::{env, http};

fn main() -> ExitCode {
    if let Ok(()) = run() {
        println!("\nExiting the program");
        ExitCode::from(0)
    } else {
        eprintln!("Exiting the program");
        ExitCode::from(1)
    }
}

fn run() -> Result<(), ()> {
    let days = vec![1, 2];

    #[cfg(feature = "env_file")]
    {
        println!("Loading all environment variables from the env file...");
        env::load_env_file(&PathBuf::from("../.env"))?;
    }

    println!("Validating the environment variables...");
    env::validate()?;

    println!("Getting all required inputs...");
    get_inputs(&days, true)?;

    println!("Getting all results...");
    days::results(&days)?;
    Ok(())
}

fn get_inputs(days: &[u8], cache: bool) -> Result<(), ()> {
    let inputs_directory = PathBuf::from("./inputs");

    for day in days {
        if cache {
            match fs::exists(inputs_directory.join(Path::new(&day.to_string()))) {
                Ok(exists) => {
                    if exists {
                        continue;
                    }
                }
                Err(err) => {
                    eprintln!(
                        "An error occured while checking the existence of a cached version of the input of day {day}: {err}"
                    );
                    return Err(());
                }
            }
        }

        http::request(*day)?;
    }

    Ok(())
}
