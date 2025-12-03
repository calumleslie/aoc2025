use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn part1() -> Result<u64, Box<dyn Error>> {
    let banks = read_banks("inputs/day3.part1.txt")?;

    let result: u64 = banks.iter().map(|b| b.max_joltage(2)).sum();

    Ok(result)
}

pub fn part2() -> Result<u64, Box<dyn Error>> {
    let banks = read_banks("inputs/day3.part1.txt")?;

    let result: u64 = banks.iter().map(|b| b.max_joltage(12)).sum();

    Ok(result)
}

fn read_banks<P>(filename: P) -> io::Result<Vec<Bank>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    io::BufReader::new(file)
        .lines()
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
        let batteries = str
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        Bank::new(batteries)
    }

    fn max_joltage(&self, length: usize) -> u64 {
        max_joltage(&self.0, length)
    }
}

fn max_joltage(window: &[u32], length: usize) -> u64 {
    assert!(length > 0);

    // We need to have at least length digits left after we select one
    let top = *window[..window.len() - (length - 1)].iter().max().unwrap();
    let top_index = window.iter().position(|n| *n == top).unwrap();

    if length == 1 {
        top as u64
    } else {
        let suffix = max_joltage(&window[(top_index + 1)..], length - 1);

        (top as u64) * 10u64.pow((length as u32) - 1) + suffix
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    static EXAMPLES: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    pub fn max_joltage_examples() {
        let max_joltages = EXAMPLES.map(Bank::parse).map(|b| b.max_joltage(2)).to_vec();

        assert_eq!(max_joltages, vec![98, 89, 78, 92]);
    }

    #[test]
    pub fn max_joltage_12_examples() {
        let max_joltages = EXAMPLES
            .map(Bank::parse)
            .map(|b| b.max_joltage(12))
            .to_vec();

        assert_eq!(
            max_joltages,
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
    }

    #[test]
    pub fn parse_example() {
        assert_eq!(
            Bank::parse("987654321111111"),
            Bank::new(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
