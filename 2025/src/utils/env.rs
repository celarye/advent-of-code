use std::env;

#[cfg(feature = "env_file")]
use std::{fs, path::Path};

#[cfg(feature = "env_file")]
pub fn load_env_file(env_file_path: &Path) -> Result<(), ()> {
    let env_file_string = match fs::read_to_string(env_file_path) {
        Ok(env_file_string) => env_file_string,
        Err(err) => {
            eprintln!("An error occured while loading the env file: {err}");
            return Err(());
        }
    };

    let env_file_str_lines = env_file_string.split("\n");

    for (env_file_line_count, env_file_str_line) in env_file_str_lines.enumerate() {
        if env_file_str_line.is_empty() {
            continue;
        }

        let key_value: Vec<&str> = env_file_str_line.split("=").collect();

        if key_value.len() != 2 {
            eprintln!(
                "A line from the env file was ignored because of it not being a valid key=value pair, line {env_file_line_count}"
            );
            continue;
        }

        unsafe {
            env::set_var(key_value[0], key_value[1]);
        }
    }

    Ok(())
}

pub fn validate() -> Result<(), ()> {
    if let Err(err) = env::var("SESSION") {
        eprintln!("An error occured while loading the SESSION environment variable: {err}");
        return Err(());
    }

    Ok(())
}
