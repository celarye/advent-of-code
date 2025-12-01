// TODO: Replace the Curl subprocess with a basic HTTP/2 request using std::io::net
// TODO: Prevent early fetching
// TODO: Cache the response and use that instead of re-requesting it

use std::{env, process::Command};

pub fn request(day: u8) -> Result<String, ()> {
    println!("Requesting the input of day {}...", day);

    let p_request = Command::new("/usr/bin/curl")
        .arg("-H")
        .arg(format!(
            "Cookie: session={}",
            env::var("SESSION").unwrap_or_else(|_| "".to_string())
        ))
        .arg(format!("https://adventofcode.com/2025/day/{}/input", day))
        .output();

    match p_request {
        Ok(p_input_string) => match String::from_utf8(p_input_string.stdout) {
            Ok(input_string) => match input_string.as_str() {
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" => {
                    eprintln!("An error occurred while requesting the input: the provided session token is invalid");
                    Err(())
                }
                "Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server time; the link will be enabled on the calendar the instant this puzzle becomes available.\n" => {
                    eprintln!("An error occurred while requesting the input: the puzzle has not been released yet");
                    Err(())
                }
                _ => {
                    println!("Successfully requested the input");
                    Ok(input_string)
                }
            }
            Err(err) => {
                eprintln!("An error occurred while requesting the input: {}", &err);
                Err(())
            },
        },
        Err(err) => {
            eprintln!("An error occurred while requesting the input: {}", &err);
            Err(())
        },
    }
}
