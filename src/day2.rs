use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

type IdRange = RangeInclusive<u64>;

pub fn part1() -> Result<u64, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day2.part1.txt")?.read_to_string(&mut input)?;
    let ranges = parse_ranges(input.as_str())?;

    Ok(sum_vec_vecs(ranges.iter().map(find_invalid_ids).collect()))
}

fn sum_vec_vecs(vecs: Vec<Vec<u64>>) -> u64 {
    vecs.iter().map(|ids| ids.iter().sum::<u64>()).sum()
}

fn find_invalid_ids(range: &IdRange) -> Vec<u64> {
    // Repeated ranges of characters can only happen when the number of digits are even, and there's
    // only one for a given "prefix" (first half). We will generate them directly.

    let start = *range.start();
    let digits = start.ilog10() + 1;

    let mut start_prefix = if digits % 2 != 0 {
        10u64.pow(digits / 2)
    } else {
        start / (10u64.pow(digits / 2))
    };

    // Make sure the first value is in the range in a very non-hacky, very trustworthy way
    if !range.contains(&invalid_id(start_prefix)) {
        start_prefix += 1;
    }

    (start_prefix..)
        .map(invalid_id)
        .take_while(|id| range.contains(&id))
        .collect()
}

fn invalid_id(prefix: u64) -> u64 {
    (prefix * 10u64.pow(prefix.ilog10() + 1)) + prefix
}

fn parse_ranges(str: &str) -> Result<Vec<IdRange>, Box<dyn Error>> {
    str.split(",").map(parse_range).collect()
}

fn parse_range(str: &str) -> Result<IdRange, Box<dyn Error>> {
    println!("parsing {}", str);
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
    fn count_invalid_ids_examples() {
        let examples = parse_ranges(EXAMPLES).unwrap();


        // 11-22 has two invalid IDs, 11 and 22.
        // 95-115 has one invalid ID, 99.
        // 998-1012 has one invalid ID, 1010.
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        // 222220-222224 has one invalid ID, 222222.
        // 1698522-1698528 contains no invalid IDs.
        //     446443-446449 has one invalid ID, 446446.
        // 38593856-38593862 has one invalid ID, 38593859.

        let invalid_ids: Vec<Vec<u64>> = examples.iter().map(find_invalid_ids).collect();

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

        assert_eq!(sum_vec_vecs(invalid_ids), 1227775554);
    }
}