use std::cmp::Reverse;
use std::collections::BinaryHeap;

use ahash::HashSet;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;
use crate::utils::HeapElement;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    Ok(solve(&grid, 0, 3).to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    Ok(solve(&grid, 4, 10).to_string())
}

fn solve(grid: &Grid<u8>, min_dir: u8, max_dir: u8) -> usize {
    let (cols, rows) = grid.size();
    let target_point = Point::new(cols - 1, rows - 1);

    let mut seen = HashSet::default();
    let mut frontier = BinaryHeap::new();

    let start = WalkState {
        loc: Point::new(0, 0),
        dir: Direction::Right,
        straight_count: 0,
    };

    for state in start.iter_next(0, 1) {
        frontier.push(Reverse(HeapElement::from((0usize, state))));
    }

    while let Some(elem) = frontier.pop().map(|x| x.0) {
        let Some(&additional_cost) = grid.get(elem.value.loc) else {
            continue;
        };

        let new_cost = elem.key + additional_cost as usize;

        if elem.value.loc == target_point && elem.value.straight_count >= min_dir {
            return new_cost;
        }

        for next_state in elem.value.iter_next(min_dir, max_dir) {
            if seen.insert(next_state.clone()) {
                frontier.push(Reverse((new_cost, next_state).into()));
            }
        }
    }

    unreachable!("no solution");
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct WalkState {
    loc: Point,
    dir: Direction,
    straight_count: u8,
}

impl WalkState {
    fn iter_next(&self, min_dir: u8, max_dir: u8) -> impl Iterator<Item = Self> + '_ {
        let dirs = [
            (self.straight_count >= min_dir).then(|| self.dir.rotate_left()),
            (self.straight_count < max_dir).then_some(self.dir),
            (self.straight_count >= min_dir).then(|| self.dir.rotate_right()),
        ];

        dirs.into_iter().filter_map(|d| {
            let d = d?;
            Some(WalkState {
                loc: self.loc.next(d)?,
                dir: d,
                straight_count: if d == self.dir {
                    self.straight_count + 1
                } else {
                    1
                },
            })
        })
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<u8>> {
        let cell = one_of("0123456789").map(|x| x.to_digit(10).unwrap() as u8);
        let row = ws_line(many1(cell));
        let grid = many1(row).map(|d| Grid::new(d));
        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "102")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "94")
    }
}
