// TODO: Replace the Curl subprocess with a basic HTTP/2 request using std::io::net
// TODO: Prevent early fetching

use std::{env, fs, path::PathBuf, process::Command};

pub fn request(day: u8) -> Result<(), ()> {
    println!("Requesting the input of day {day}...");

    let p_request = Command::new("/usr/bin/curl")
        .arg("-H")
        .arg(format!(
            "Cookie: session={}",
            env::var("SESSION").expect("the SESSION env var should be available")
        ))
        .arg(format!("https://adventofcode.com/2025/day/{day}/input"))
        .output();

    match p_request {
        Ok(p_input_string) => match String::from_utf8(p_input_string.stdout) {
            Ok(input_string) => match input_string.as_str() {
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" => {
                    eprintln!(
                        "An error occurred while requesting the input: the provided session token is missing or invalid"
                    );
                    Err(())
                }
                "Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server time; the link will be enabled on the calendar the instant this puzzle becomes available.\n" =>
                {
                    eprintln!(
                        "An error occurred while requesting the input: the puzzle has not been released yet"
                    );
                    Err(())
                }
                _ => {
                    println!("Successfully requested the input, writing it to a file...");

                    if let Err(err) =
                        fs::write(PathBuf::from(format!("./inputs/{day}")), &input_string)
                    {
                        eprintln!(
                            "An error occured while trying to write the input to a file: {err}",
                        );
                    }

                    Ok(())
                }
            },
            Err(err) => {
                eprintln!("An error occurred while parsing the input into UTF-8: {err}",);
                Err(())
            }
        },
        Err(err) => {
            eprintln!("An error occurred while requesting the input: {err}");
            Err(())
        }
    }
}
