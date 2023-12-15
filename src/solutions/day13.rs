use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grids = parse!(input);
    let ans: usize = grids
        .iter()
        .map(|g| {
            let cg = CompressedGrid::try_from(g).unwrap();
            find_mirror(&cg.rows, 0) * 100 + find_mirror(&cg.columns, 0)
        })
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grids = parse!(input);
    let ans: usize = grids
        .iter()
        .map(|g| {
            let cg = CompressedGrid::try_from(g).unwrap();
            find_mirror(&cg.rows, 1) * 100 + find_mirror(&cg.columns, 1)
        })
        .sum();
    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct CompressedGrid {
    rows: Vec<u32>,
    columns: Vec<u32>,
}

impl TryFrom<&Grid<bool>> for CompressedGrid {
    type Error = anyhow::Error;

    fn try_from(grid: &Grid<bool>) -> Result<Self, Self::Error> {
        let (col_len, row_len) = grid.size();
        if col_len > 32 || row_len > 32 {
            bail!("grid too big to be compressed");
        }

        let rows = grid
            .cells
            .iter()
            .map(|r| r.iter().fold(0, |acc, &x| (acc << 1) + x as u32))
            .collect();

        let columns = (0..col_len)
            .map(|col| {
                (0..row_len)
                    .map_while(|row| grid.get(Point::new(col, row)))
                    .fold(0, |acc, &x| (acc << 1) + x as u32)
            })
            .collect();

        Ok(CompressedGrid { rows, columns })
    }
}

fn find_mirror(lines: &[u32], target_diff: u32) -> usize {
    (1..lines.len())
        .find(|&mirror| {
            let diffs: u32 = lines[..mirror]
                .iter()
                .rev()
                .zip(&lines[mirror..])
                .map(|(&a, &b)| (a ^ b).count_ones())
                .sum();
            diffs == target_diff
        })
        .unwrap_or(0)
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Grid<bool>>> {
        let cell = alt((value(false, char('.')), value(true, char('#'))));
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
