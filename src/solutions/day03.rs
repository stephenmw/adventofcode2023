use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

use ahash::HashMap;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let parts = locate_parts(&grid);
    let ans: usize = parts
        .iter()
        .filter(|part| {
            let digit_points = part.points();
            let adj_points = digit_points.flat_map(|p| p.iter_adjacent8());
            let mut adj_chars = adj_points.filter_map(|p| grid.get(p));
            adj_chars.any(|c| is_symbol(*c))
        })
        .map(|part| part.num)
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let parts = locate_parts(&grid);

    let part_locs: HashMap<_, _> = parts
        .iter()
        .flat_map(|p| p.points().zip(std::iter::repeat(p)))
        .collect();

    let gear_locs = grid
        .iter_points()
        .filter(|p| *grid.get(*p).unwrap_or(&b'.') == b'*');

    let gear_adj_parts = gear_locs.map(|loc| {
        let mut adj_nums: Vec<_> = loc
            .iter_adjacent8()
            .filter_map(|p| part_locs.get(&p))
            .collect();
        adj_nums.sort_unstable();
        adj_nums.dedup();
        adj_nums
    });

    let true_gears = gear_adj_parts.filter(|x| x.len() == 2);

    let ans: usize = true_gears.map(|x| x[0].num * x[1].num).sum();
    Ok(ans.to_string())
}

fn locate_parts(g: &Grid<u8>) -> Vec<PartNum> {
    let mut cur = PartNum::default();
    let mut ret = Vec::new();

    for (i, r) in g.cells.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if let Some(d) = (c as char).to_digit(10) {
                if cur.num == 0 {
                    cur.row = i;
                    cur.col_start = j;
                }

                cur.num = cur.num * 10 + d as usize;
                cur.col_end = j
            } else {
                if cur.num != 0 {
                    ret.push(cur);
                    cur = PartNum::default();
                }
            }
        }

        if cur.num != 0 {
            ret.push(cur);
            cur = PartNum::default();
        }
    }

    ret
}

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct PartNum {
    num: usize,
    row: usize,
    col_start: usize,
    col_end: usize,
}

impl PartNum {
    fn points(&self) -> impl Iterator<Item = Point> {
        let p = *self;
        (p.col_start..=p.col_end).map(move |col| Point::new(col, p.row))
    }
}

mod parser {
    use nom::bytes::complete::take_while1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<u8>> {
        let row = ws_line(take_while1(|c: char| !c.is_ascii_whitespace()))
            .map(|x| x.as_bytes().to_owned());
        let rows = many1(row);
        let grid = map(rows, |d| Grid::new(d));
        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "4361")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "467835")
    }
}
