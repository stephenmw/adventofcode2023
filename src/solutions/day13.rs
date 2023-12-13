use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grids = parse!(input);
    let ans: usize = grids
        .iter()
        .map(|g| find_v_mirror(g, 0).unwrap_or(0) + find_h_mirror(g, 0).unwrap_or(0) * 100)
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grids = parse!(input);
    let ans: usize = grids
        .iter()
        .map(|g| find_v_mirror(g, 1).unwrap_or(0) + find_h_mirror(g, 1).unwrap_or(0) * 100)
        .sum();
    Ok(ans.to_string())
}

fn find_v_mirror(grid: &Grid<Cell>, num_smudges: usize) -> Option<usize> {
    fn reflect(p: Point, mirror: usize) -> Point {
        let col = 2 * mirror - p.x - 1;
        Point::new(col, p.y)
    }

    let (cols, rows) = grid.size();

    (1..cols).find(|&mirror| {
        let left_points = (0..mirror)
            .rev()
            .flat_map(|c| (0..rows).map(move |r| Point::new(c, r)));
        let diffs = left_points
            .map_while(|p| Some(grid.get(p).unwrap() == grid.get(reflect(p, mirror))?))
            .filter(|&x| x == false)
            .count();
        diffs == num_smudges
    })
}

fn find_h_mirror(grid: &Grid<Cell>, num_smudges: usize) -> Option<usize> {
    fn reflect(p: Point, mirror: usize) -> Point {
        let row = 2 * mirror - p.y - 1;
        Point::new(p.x, row)
    }

    let (cols, rows) = grid.size();

    (1..rows).find(|&mirror| {
        let top_points = (0..mirror)
            .rev()
            .flat_map(|r| (0..cols).map(move |c| Point::new(c, r)));
        let diffs = top_points
            .map_while(|p| Some(grid.get(p).unwrap() == grid.get(reflect(p, mirror))?))
            .filter(|&x| x == false)
            .count();
        diffs == num_smudges
    })
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
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "400")
    }
}
