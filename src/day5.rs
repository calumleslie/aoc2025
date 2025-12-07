use disjoint_sets::UnionFind;
use itertools::Itertools;
use std::collections::HashMap;
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

pub fn part2() -> Result<u64, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day5.part1.txt")?.read_to_string(&mut input)?;

    let database = Database::from_str(input.as_str())?;

    Ok(database.count_fresh_ingredients())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Database {
    fresh_ingredients: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Database {
    fn new(fresh_ingredients: Vec<RangeInclusive<u64>>, available_ingredients: Vec<u64>) -> Self {
        Database {
            fresh_ingredients: simplify_ranges(&fresh_ingredients),
            available_ingredients,
        }
    }

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

        Ok(Database::new(fresh, available))
    }

    fn fresh_available_ingredients(&self) -> impl Iterator<Item = u64> {
        self.available_ingredients
            .iter()
            .copied()
            .filter(|ingr| self.is_fresh(*ingr))
    }

    fn count_fresh_ingredients(&self) -> u64 {
        count_included(&self.fresh_ingredients)
    }

    fn is_fresh(&self, ingredient_id: u64) -> bool {
        self.fresh_ingredients
            .iter()
            .any(|range| range.contains(&ingredient_id))
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

fn simplify_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let overlap_pairs = ranges
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(l, r)| overlap(l, r))
        .collect::<Vec<_>>();

    // We're transitively joining all ranges that overlap at all and merging them, the union find
    // means we can do this in one "step". Do not ask me about time complexity, I did not think
    // about it.

    let range_to_index = ranges
        .iter()
        .enumerate()
        .map(|(i, r)| (r, i))
        .collect::<HashMap<_, _>>();

    let mut union_find = UnionFind::new(ranges.len());
    for (l, r) in overlap_pairs {
        union_find.union(range_to_index[l], range_to_index[r]);
    }

    ranges
        .iter()
        .map(|r| {
            let key = union_find.find(range_to_index[r]);
            (key, r)
        })
        .into_group_map()
        .values()
        .map(|ranges| merge(ranges))
        // Sort to make the results stable as I am too lazy to test better
        .sorted_by_key(|r| *r.start())
        .collect()
}

fn count_included(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges.iter().map(|r| (r.end() - r.start()) + 1).sum()
}

fn merge(ranges: &Vec<&RangeInclusive<u64>>) -> RangeInclusive<u64> {
    let start = ranges.iter().map(|range| range.start()).min().unwrap();
    let end = ranges.iter().map(|range| range.end()).max().unwrap();

    *start..=*end
}

fn overlap(l: &RangeInclusive<u64>, r: &RangeInclusive<u64>) -> bool {
    (l.start() <= r.end() && l.end() >= r.start()) || (r.start() <= l.end() && r.end() >= l.start())
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
                fresh_ingredients: vec!(3..=5, 10..=20),
                available_ingredients: vec!(1, 5, 8, 11, 17, 32),
            }
        )
    }

    #[test]
    fn fresh_available_ingredients_example() {
        let database = Database::from_str(EXAMPLE).unwrap();

        assert_eq!(
            database.fresh_available_ingredients().collect::<Vec<_>>(),
            vec![5, 11, 17]
        );
    }

    #[test]
    fn overlap_examples() {
        assert_reflexive_overlap(10..=14, 12..=18);
        assert_reflexive_overlap(10..=11, 11..=12);
        assert_reflexive_no_overlap(10..=11, 12..=13);
    }

    #[test]
    fn simplify_ranges_example() {
        let database = Database::from_str(EXAMPLE).unwrap();

        let mut result = simplify_ranges(&database.fresh_ingredients);
        result.sort_by_key(|r| *r.start());

        assert_eq!(result, vec![3..=5, 10..=20,]);

        assert_eq!(count_included(&result), 14);
    }

    fn assert_reflexive_overlap(l: RangeInclusive<u64>, r: RangeInclusive<u64>) {
        assert!(overlap(&l, &r));
        assert!(overlap(&r, &l));
    }

    fn assert_reflexive_no_overlap(l: RangeInclusive<u64>, r: RangeInclusive<u64>) {
        assert!(!overlap(&l, &r));
        assert!(!overlap(&r, &l));
    }
}
