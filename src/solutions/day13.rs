use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grids = parse!(input);
    let ans: usize = grids
        .iter()
        .map(|g| find_v_mirror(g).unwrap_or(0) + find_h_mirror(g).unwrap_or(0) * 100)
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

fn find_v_mirror(grid: &Grid<Cell>) -> Option<usize> {
    let (cols, rows) = grid.size();

    'mirror: for mirror in 1..cols {
        for c in mirror..cols {
            for r in 0..rows {
                let reflection = (2 * mirror).checked_sub(c + 1).map(|c| Point::new(c, r));
                let Some(reflect_val) = reflection.and_then(|p| grid.get(p)) else {
                    return Some(mirror);
                };
                let val = grid.get(Point::new(c, r)).unwrap();
                if reflect_val != val {
                    continue 'mirror;
                }
            }
        }

        return Some(mirror);
    }

    None
}

fn find_h_mirror(grid: &Grid<Cell>) -> Option<usize> {
    let (cols, rows) = grid.size();

    'mirror: for mirror in 1..rows {
        for r in mirror..rows {
            for c in 0..cols {
                let reflection = (2 * mirror).checked_sub(r + 1).map(|r| Point::new(c, r));
                let Some(reflect_val) = reflection.and_then(|p| grid.get(p)) else {
                    return Some(mirror);
                };
                let val = grid.get(Point::new(c, r)).unwrap();
                if reflect_val != val {
                    continue 'mirror;
                }
            }
        }

        return Some(mirror);
    }

    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Ash,
    Rock,
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Grid<Cell>>> {
        let cell = alt((value(Cell::Ash, char('.')), value(Cell::Rock, char('#'))));
        let row = ws_line(many1(cell));
        let grid = many1(row).map(|rows| Grid::new(rows));
        let parser = separated_list1(multispace1, grid);
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "405")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "400")
    }
}
