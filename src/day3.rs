use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let banks = read_banks("inputs/day3.part1.txt")?;

    let result = banks.iter().map(Bank::max_joltage).sum();

    Ok(result)
}

fn read_banks<P>(filename: P) -> io::Result<Vec<Bank>> where P: AsRef<Path> {
    let file = File::open(filename)?;

    io::BufReader::new(file).lines()
        .map(|l| l.map(|t| Bank::parse(t.as_str())))
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Bank(Vec<u32>);

impl Bank {
    fn new(batteries: Vec<u32>) -> Bank {
        Bank(batteries)
    }

    fn parse(str: &str) -> Bank {
        let batteries = str.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        Bank::new(batteries)
    }

    fn max_joltage(&self) -> u32 {
        // First digit is the highest number, second digit is the highest number occurring after
        // that one. We cannot use the last digit as first
        let first = self.0[..self.0.len() - 1].iter().max().unwrap();
        let first_index = self.0.iter().position(|n| n == first).unwrap();
        let second = self.0[(first_index + 1)..].iter().max().unwrap();

        (first * 10) + second
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    static EXAMPLES: [&str; 4] = ["987654321111111", "811111111111119", "234234234234278", "818181911112111"];

    #[test]
    pub fn max_joltage_examples() {
        let max_joltages = EXAMPLES
            .map(Bank::parse)
            .map(|b| b.max_joltage())
            .to_vec();

        assert_eq!(max_joltages, vec![98, 89, 78, 92]);
    }

    #[test]
    pub fn parse_example() {
        assert_eq!(
            Bank::parse("987654321111111"),
            Bank::new(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]));
    }
}
