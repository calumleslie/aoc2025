use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use itertools::Itertools;

type IdRange = RangeInclusive<u64>;

pub fn part1() -> Result<u64, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day2.part1.txt")?.read_to_string(&mut input)?;
    let ranges = parse_ranges(input.as_str())?;

    Ok(ranges.iter()
        .map(|r| find_all_invalid_ids_with_exact_repetition(r, 2).sum::<u64>())
        .sum())
}

pub fn part2() -> Result<u64, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day2.part1.txt")?.read_to_string(&mut input)?;
    let ranges = parse_ranges(input.as_str())?;

    // The brute force version runs so fast I can leave this amazing parity check in! I don't regret
    // wasting my time at all, in case you were wondering!
    ranges.iter().for_each(|range| {
        assert_eq!(
            find_all_invalid_ids(range).sorted().collect::<Vec<_>>(),
            find_all_invalid_ids_brute_force(range).collect::<Vec<_>>(),
            "Range: {:?}", range)
    });

    Ok(ranges.iter()
        .map(|r| find_all_invalid_ids(r).sum::<u64>())
        .sum())
}

fn find_all_invalid_ids_brute_force(range: &IdRange) -> impl Iterator<Item = u64> {
    range.clone().filter(|i| contains_repetition(*i))
}

fn contains_repetition(i: u64) -> bool {
    let digits = i.ilog10() + 1;

    (1..=(digits / 2)).any(|pl| {
        let prefix = i / 10u64.pow(digits - pl);

        i == invalid_id(prefix, digits / pl)
    })
}

fn find_all_invalid_ids(range: &IdRange) -> impl Iterator<Item = u64> {
    let max_prefix_length = (range.end().ilog10() / 2) + 1;

    (1..=max_prefix_length)
        .flat_map(|pl| invalid_ids_with_prefix_length(range, pl))
        .unique()
}

fn find_all_invalid_ids_with_exact_repetition(range: &IdRange, repetitions: u32) -> impl Iterator<Item = u64> {
    // Having to do the max here makes me worried that I screwed this logic up
    let first_length = max(1, (range.start().ilog10() + 1) / repetitions);

    (first_length..)
        .flat_map(move |pl| invalid_ids_with_prefix_and_repetitions(pl, repetitions))
        .take_while(|id| range.end() >= id)
        .filter(|id| range.contains(id))
}

fn invalid_ids_with_prefix_and_repetitions(prefix_length: u32, repetitions: u32) -> impl Iterator<Item=u64> {
    let first_prefix = 10u64.pow(prefix_length - 1);

    // In the old version we went through some pains to calculate the right "first" prefix. We could
    // do that again but we might as well skip through them

    (first_prefix..)
        .take_while(move |prefix| prefix.ilog10() == prefix_length - 1)
        .map(move |prefix| invalid_id(prefix, repetitions))
}

fn invalid_ids_with_prefix_length(range: &IdRange, prefix_length: u32) -> impl Iterator<Item=u64> {
    // Repeated ranges of characters can only happen when the number of digits divide by len, and
    // there's only one for a given "prefix". We will generate them directly.
    let digits = range.start().ilog10() + 1;

    // We could do better here but lazy
    let first_repetitions = max(2, digits / prefix_length);

    // In the old version we went through some pains to calculate the right "first" prefix. We could
    // do that again but we might as well skip through them

    (first_repetitions..)
        .flat_map(move |r| invalid_ids_with_prefix_and_repetitions(prefix_length, r))
        .take_while(|id| range.end() >= id)
        .filter(|id| range.contains(&id))
}

fn invalid_id(prefix: u64, repetitions: u32) -> u64 {
    let length = prefix.ilog10() + 1;

    (0..repetitions)
        .map(|r| prefix * 10u64.pow(r * length))
        .sum()
}

fn parse_ranges(str: &str) -> Result<Vec<IdRange>, Box<dyn Error>> {
    str.split(",").map(parse_range).collect()
}

fn parse_range(str: &str) -> Result<IdRange, Box<dyn Error>> {
    let bits: Vec<u64> = str.split("-").map(u64::from_str).collect::<Result<Vec<_>, ParseIntError>>()?;

    if bits.len() != 2 {
        return Err("encountered range with more than two components".into());
    }

    Ok(bits[0]..=bits[1])
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    static EXAMPLES: &'static str = "11-22,\
        95-115,\
        998-1012,\
        1188511880-1188511890,\
        222220-222224,\
        1698522-1698528,\
        446443-446449,\
        38593856-38593862,\
        565653-565659,\
        824824821-824824827,\
        2121212118-2121212124";

    #[test]
    fn parse_ranges_simple() {
        assert_eq!(
            parse_ranges("11-22,95-115,998-1012,1188511880-1188511890").unwrap(),
            vec![11..=22, 95..=115, 998..=1012, 1188511880..=1188511890]);
    }

    #[test]
    fn invalid_id_various_lengths() {
        assert_eq!(invalid_id(21, 4), 21212121);
        assert_eq!(invalid_id(123, 4), 123123123123);
        assert_eq!(invalid_id(1234, 2), 12341234);
    }

    #[test]
    fn invalid_ids_with_prefix_length_works_as_expected() {
        fn collect_ids(range: &IdRange, prefix_length: u32) -> Vec<u64> {
            invalid_ids_with_prefix_length(range, prefix_length).collect()
        }

        assert_eq!(
            collect_ids(&(11..=44), 1),
            vec![11, 22, 33, 44]);

        assert_eq!(
            collect_ids(&(88..=1111), 1),
           vec![88, 99, 111, 222, 333, 444, 555, 666, 777, 888, 999, 1111]);

        assert_eq!(
            collect_ids(&(1000..=2000), 2),
            vec![1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717, 1818, 1919]);

        assert_eq!(
            collect_ids(&(1230_1230..=1234_1234), 4),
            vec![1230_1230, 1231_1231, 1232_1232, 1233_1233, 1234_1234]);
    }


    #[test]
    fn exactly_2_repetitions_examples() {
        let examples = parse_ranges(EXAMPLES).unwrap();

        let invalid_ids: Vec<Vec<u64>> = examples.iter()
            .map(|e| find_all_invalid_ids_with_exact_repetition(e, 2).collect())
            .collect();

        assert_eq!(invalid_ids,
            vec![
                vec![11, 22],
                vec![99],
                vec![1010],
                vec![1188511885],
                vec![222222],
                vec![],
                vec![446446],
                vec![38593859],
                vec![],
                vec![],
                vec![],
            ]);


        assert_eq!(examples.iter()
                       .map(|r| find_all_invalid_ids_with_exact_repetition(r, 2).sum::<u64>())
                       .sum::<u64>(), 1227775554);
    }

    #[test]
    fn all_repetitions_duplicates_returned_once() {
        assert_eq!(
            find_all_invalid_ids(&(1111..=1111)).collect::<Vec<u64>>(),
            vec![1111]);
    }

    #[test]
    fn all_repetitions_single_digits() {
        assert_eq!(
            find_all_invalid_ids(&(1..=20)).collect::<Vec<u64>>(),
            vec![11]);
    }

    #[test]
    fn all_repetitions_examples() {
        let examples = parse_ranges(EXAMPLES).unwrap();

        let invalid_ids: Vec<Vec<u64>> = examples.iter()
            .map(|e| find_all_invalid_ids(e).collect())
            .collect();

        assert_eq!(invalid_ids,
                   vec![
                       vec![11, 22],
                       vec![99, 111],
                       vec![999, 1010],
                       vec![1188511885],
                       vec![222222],
                       vec![],
                       vec![446446],
                       vec![38593859],
                       vec![565656],
                       vec![824824824],
                       vec![2121212121],
                   ]);

        assert_eq!(examples.iter()
                       .map(|r| find_all_invalid_ids(r).sum::<u64>())
                       .sum::<u64>(), 4174379265);
    }
}