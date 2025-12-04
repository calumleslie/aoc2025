use array2d::Array2D;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

pub fn part1() -> Result<usize, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day4.part1.txt")?.read_to_string(&mut input)?;

    let grid = Grid::from_str(input.as_str());

    Ok(grid.accessible_rolls().len())
}

pub fn part2() -> Result<usize, Box<dyn Error>> {
    let mut input = String::new();
    File::open("inputs/day4.part1.txt")?.read_to_string(&mut input)?;

    let mut grid = Grid::from_str(input.as_str());

    Ok(grid.remove_accessible_repeated())
}

// These have stupid names so they have the same number of characters and I can line them up
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Entry {
    Empt,
    Roll,
}

impl Entry {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empt),
            '@' => Some(Self::Roll),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid(Array2D<Entry>);

impl Grid {
    fn from_str(str: &str) -> Self {
        let n_rows = str.lines().count();
        let n_columns = str.lines().next().unwrap().len();

        let mut array = Array2D::filled_by_column_major(|| Entry::Empt, n_rows, n_columns);

        str.lines().enumerate().for_each(|(r, line)| {
            line.chars().enumerate().for_each(|(c, chr)| {
                array[(r, c)] = Entry::from_char(&chr).unwrap();
            });
        });

        Grid(array)
    }

    fn remove_accessible_repeated(&mut self) -> usize {
        let mut total_removed: usize = 0;

        loop {
            let removed = self.remove_accessible_once();
            total_removed += removed;

            if removed == 0 {
                return total_removed;
            }
        }
    }

    fn remove_accessible_once(&mut self) -> usize {
        let mut count: usize = 0;
        for loc in self.accessible_rolls() {
            count += 1;
            self.clear(loc);
        }
        count
    }

    fn clear(&mut self, loc: (usize, usize)) {
        self.0[loc] = Entry::Empt;
    }

    fn accessible_rolls(&self) -> Vec<(usize, usize)> {
        self.0
            .indices_row_major()
            .filter(|loc| self.0[*loc] == Entry::Roll)
            .filter(|loc| self.adjacent_rolls(*loc) < 4)
            .collect()
    }

    fn adjacent_rolls(&self, loc: (usize, usize)) -> usize {
        self.surrounding(loc)
            .filter(|loc| self.0[*loc] == Entry::Roll)
            .count()
    }

    fn surrounding(&self, loc: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let first_row = if loc.0 == 0 { 0 } else { loc.0 - 1 };
        let last_row = if loc.0 == self.0.num_rows() - 1 {
            loc.0
        } else {
            loc.0 + 1
        };
        let first_col = if loc.1 == 0 { 0 } else { loc.1 - 1 };
        let last_col = if loc.1 == self.0.num_columns() - 1 {
            loc.1
        } else {
            loc.1 + 1
        };

        (first_row..=last_row)
            .flat_map(move |r| (first_col..=last_col).map(move |c| (r, c)))
            .filter(move |l| *l != loc)
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    static EXAMPLE: &str = "..@@.@@@@.\n\
         @@@.@.@.@@\n\
         @@@@@.@.@@\n\
         @.@@@@..@.\n\
         @@.@@@@.@@\n\
         .@@@@@@@.@\n\
         .@.@.@.@@@\n\
         @.@@@.@@@@\n\
         .@@@@@@@@.\n\
         @.@.@@@.@.";

    #[test]
    fn parse_example() {
        let grid = Grid::from_str(EXAMPLE);

        use Entry::*;

        assert_eq!(
            grid,
            Grid(
                Array2D::from_rows(&[
                    vec![Empt, Empt, Roll, Roll, Empt, Roll, Roll, Roll, Roll, Empt],
                    vec![Roll, Roll, Roll, Empt, Roll, Empt, Roll, Empt, Roll, Roll],
                    vec![Roll, Roll, Roll, Roll, Roll, Empt, Roll, Empt, Roll, Roll],
                    vec![Roll, Empt, Roll, Roll, Roll, Roll, Empt, Empt, Roll, Empt],
                    vec![Roll, Roll, Empt, Roll, Roll, Roll, Roll, Empt, Roll, Roll],
                    vec![Empt, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Empt, Roll],
                    vec![Empt, Roll, Empt, Roll, Empt, Roll, Empt, Roll, Roll, Roll],
                    vec![Roll, Empt, Roll, Roll, Roll, Empt, Roll, Roll, Roll, Roll],
                    vec![Empt, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Empt],
                    vec![Roll, Empt, Roll, Empt, Roll, Roll, Roll, Empt, Roll, Empt],
                ])
                .unwrap()
            )
        );
    }

    #[test]
    fn part1_example() {
        let grid = Grid::from_str(EXAMPLE);

        assert_eq!(
            grid.accessible_rolls(),
            vec![
                (0, 2),
                (0, 3),
                (0, 5),
                (0, 6),
                (0, 8),
                (1, 0),
                (2, 6),
                (4, 0),
                (4, 9),
                (7, 0),
                (9, 0),
                (9, 2),
                (9, 8)
            ]
        );
    }

    #[test]
    fn part2_example() {
        let mut grid = Grid::from_str(EXAMPLE);

        assert_eq!(grid.remove_accessible_repeated(), 43);
    }
}
