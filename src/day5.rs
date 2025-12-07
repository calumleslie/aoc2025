use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn part1() -> Result<usize, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day5.part1.txt")?.read_to_string(&mut input)?;

    let database = Database::from_str(input.as_str())?;

    Ok(database.fresh_available_ingredients().count())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Database {
    fresh_ingredients: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        // We could presumably do better by taking the order into account
        let (fresh_lines, available_lines) = input
            .lines()
            .filter(|line| !line.is_empty())
            .partition::<Vec<_>, _>(|line| line.contains('-'));

        let fresh = fresh_lines
            .iter()
            .map(|l| parse_range(l))
            .collect::<Result<Vec<_>, _>>()?;
        let available = available_lines
            .iter()
            .map(|l| u64::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Database {
            fresh_ingredients: fresh,
            available_ingredients: available,
        })
    }

    fn fresh_available_ingredients(&self) -> impl Iterator<Item=u64> {
        self.available_ingredients.iter()
            .map(|ingr| *ingr)
            .filter(|ingr| self.is_fresh(*ingr))
    }

    fn is_fresh(&self, ingredient_id: u64) -> bool {
        self.fresh_ingredients.iter().any(|range| range.contains(&ingredient_id))
    }
}

fn parse_range(s: &str) -> Result<RangeInclusive<u64>, Box<dyn Error>> {
    let bits: Vec<u64> = s.split("-").map(u64::from_str).collect::<Result<_, _>>()?;
    if bits.len() != 2 {
        Err("Invalid number of fields".into())
    } else {
        Ok(bits[0]..=bits[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    static EXAMPLE: &str = "3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\n";

    #[test]
    fn parse_example() {
        assert_eq!(
            Database::from_str(EXAMPLE).unwrap(),
            Database {
                fresh_ingredients: vec!(3..=5, 10..=14, 16..=20, 12..=18),
                available_ingredients: vec!(1, 5, 8, 11, 17, 32),
            }
        )
    }

    #[test]
    fn fresh_available_ingredients_example() {
        let database = Database::from_str(EXAMPLE).unwrap();

        assert_eq!(database.fresh_available_ingredients().collect::<Vec<_>>(), vec![5, 11, 17]);
    }
}
