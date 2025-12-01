use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;
use regex::Regex;

// Maybe this would be better with unsigned arithmetic but I do not trust myself with it

// This is waaaay more accuracy than we need but hey, in for a penny
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Dial(i64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Turn(i64);

impl Dial {
    fn new() -> Dial {
        Dial(50)
    }

    fn direction(&self) -> i64 {
        self.0
    }

    fn of(direction: i64) -> Dial {
        assert!(direction >= 0 && direction < 100, "invalid direction: {}", direction);
        Dial(direction)
    }

    fn apply(&self, turn: Turn) -> Dial {
        let amount_effective = turn.0.abs() % 100;

        let effective_turn = match turn.0.signum() {
            0 => 0,
            -1 => 100 - amount_effective,
            1 => amount_effective,
            _ => unreachable!(),
        };

        let result = (self.0 + effective_turn) % 100;

        Dial::of(result)
    }
}

impl Turn {
    fn parse(s: &str) -> Turn {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([LR])([0-9]+)$").unwrap());

        let captures = RE.captures(s).unwrap();

        let dir = if "L" == &captures[1] { -1 } else { 1 };
        let amount = i64::from_str(&captures[2]).unwrap();

        Turn(dir * amount)
    }
}

fn count_zeroes(initial: Dial, turns: Vec<Turn>) -> i64 {
    let mut dial = initial;
    let mut count: i64 = 0;

    for turn in turns {
        dial = dial.apply(turn);

        if dial.direction() == 0 {
            count += 1
        }
    }

    count
}

pub fn part1() -> Result<i64, Box<dyn Error>> {
    let turns = read_turns("inputs/day1.part1.txt")?;

    Ok(count_zeroes(Dial::new(), turns))
}

fn read_turns<P>(filename: P) -> io::Result<Vec<Turn>> where P: AsRef<Path> {
    let file = File::open(filename)?;

    let mut result: Vec<Turn> = vec![];

    for line in io::BufReader::new(file).lines() {
        result.push(Turn::parse(&line?));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    static EXAMPLES: &'static str = "L68
                                     L30
                                     R48
                                     L5
                                     R60
                                     L55
                                     L1
                                     L99
                                     R14
                                     L82";

    #[test]
    fn dial_simple_examples() {
        assert_eq!(Dial::of(50).apply(Turn(50)), Dial::of(0));
        assert_eq!(Dial::of(50).apply(Turn(0)), Dial::of(50));
        assert_eq!(Dial::of(50).apply(Turn(-100)), Dial::of(50));
        assert_eq!(Dial::of(50).apply(Turn(-50)), Dial::of(0));
        assert_eq!(Dial::of(50).apply(Turn(-100_000_000)), Dial::of(50));
    }

    #[test]
    fn turn_parse_examples() {
        let turns: Vec<Turn> = EXAMPLES.lines().map(|l| Turn::parse(l.trim())).collect();

        assert_eq!(turns, vec![
            Turn(-68),
            Turn(-30),
            Turn(48),
            Turn(-5),
            Turn(60),
            Turn(-55),
            Turn(-1),
            Turn(-99),
            Turn(14),
            Turn(-82)]);
    }

    #[test]
    fn dial_turn_using_examples() {
        let mut dial = Dial::new();
        let turns: Vec<Turn> = EXAMPLES.lines().map(|l| Turn::parse(l.trim())).collect();

        for turn in turns {
            dial = dial.apply(turn);
        }

        assert_eq!(dial, Dial::of(32));
    }

    #[test]
    fn count_zeroes_using_examples() {
        let dial = Dial::new();
        let turns: Vec<Turn> = EXAMPLES.lines().map(|l| Turn::parse(l.trim())).collect();

        assert_eq!(count_zeroes(dial, turns), 3);
    }
}