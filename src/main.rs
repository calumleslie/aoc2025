mod day1;
mod day2;
mod day3;
mod day4;

use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct CliError {
    details: String,
}

impl CliError {
    fn new(msg: &str) -> CliError {
        CliError {
            details: msg.to_string(),
        }
    }

    fn from_string(msg: String) -> CliError {
        CliError { details: msg }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CliError {}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(CliError::new("No command specified").into());
    }

    match args[1].as_str() {
        "day1" => {
            println!("Day 1 part 1: {}", day1::part1()?);
            println!("Day 1 part 2: {}", day1::part2()?);
            Ok(())
        }
        "day2" => {
            println!("Day 2 part 1: {}", day2::part1()?);
            println!("Day 2 part 2: {}", day2::part2()?);
            Ok(())
        }
        "day3" => {
            println!("Day 3 part 1: {}", day3::part1()?);
            println!("Day 3 part 2: {}", day3::part2()?);
            Ok(())
        }
        "day4" => {
            println!("Day 4 part 1: {}", day4::part1()?);
            Ok(())
        }
        cmd => Err(CliError::from_string(format!("Unknown command: {}", cmd)).into()),
    }
}
