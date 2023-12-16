use std::collections::VecDeque;

use arrayvec::ArrayVec;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let ans = num_energized(&grid, Point::new(0, 0), Direction::Right);
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let (cols, rows) = grid.size();

    let left_starts = (0..rows).map(|r| (Point::new(0, r), Direction::Right));
    let right_starts = (0..rows).map(|r| (Point::new(cols - 1, r), Direction::Left));
    let top_starts = (0..cols).map(|c| (Point::new(c, 0), Direction::Up));
    let bottom_starts = (0..cols).map(|c| (Point::new(c, rows - 1), Direction::Down));
    let starts = left_starts
        .chain(right_starts)
        .chain(top_starts)
        .chain(bottom_starts);

    let ans = starts
        .map(|(p, d)| num_energized(&grid, p, d))
        .max()
        .ok_or_else(|| anyhow!("size zero grid?"))?;

    Ok(ans.to_string())
}

fn num_energized(grid: &Grid<Cell>, start: Point, start_dir: Direction) -> usize {
    let (cols, rows) = grid.size();
    let mut seen = Grid::new(vec![vec![DirectionSet::default(); cols]; rows]);
    let mut frontier = VecDeque::new();
    frontier.push_back((start, start_dir));

    while let Some((p, d)) = frontier.pop_front() {
        if !seen.get_mut(p).map(|x| x.set(d)).unwrap_or(false) {
            continue;
        }

        for new_d in grid.get(p).map(|x| x.refract(d)).unwrap_or(ArrayVec::new()) {
            let Some(new_p) = p.next(new_d) else {
                continue;
            };

            frontier.push_back((new_p, new_d));
        }
    }

    seen.cells
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| x.0 > 0)
        .count()
}

#[derive(Clone, Copy, Debug, Default)]
struct DirectionSet(u8);

impl DirectionSet {
    // Set bit. Return false if already set.
    fn set(&mut self, d: Direction) -> bool {
        let bit = match d {
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
            Direction::Down => 4,
        };

        let old = self.0;
        self.0 |= 1 << bit;
        old & (1 << bit) == 0
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Slash,
    BackSlash,
    VerticalSplit,
    HorizontalSplit,
}

impl Cell {
    fn refract(&self, d: Direction) -> ArrayVec<Direction, 2> {
        match self {
            Self::Empty => ArrayVec::from_iter([d]),
            Self::Slash => match d {
                Direction::Left => ArrayVec::from_iter([Direction::Up]),
                Direction::Up => ArrayVec::from_iter([Direction::Left]),
                Direction::Right => ArrayVec::from_iter([Direction::Down]),
                Direction::Down => ArrayVec::from_iter([Direction::Right]),
            },
            Self::BackSlash => match d {
                Direction::Left => ArrayVec::from_iter([Direction::Down]),
                Direction::Up => ArrayVec::from_iter([Direction::Right]),
                Direction::Right => ArrayVec::from_iter([Direction::Up]),
                Direction::Down => ArrayVec::from_iter([Direction::Left]),
            },
            Self::VerticalSplit => match d {
                Direction::Left | Direction::Right => {
                    ArrayVec::from_iter([Direction::Up, Direction::Down])
                }
                Direction::Down | Direction::Up => ArrayVec::from_iter([d]),
            },
            Self::HorizontalSplit => match d {
                Direction::Left | Direction::Right => ArrayVec::from_iter([d]),
                Direction::Down | Direction::Up => {
                    ArrayVec::from_iter([Direction::Left, Direction::Right])
                }
            },
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            value(Cell::Empty, char('.')),
            value(Cell::Slash, char('/')),
            value(Cell::BackSlash, char('\\')),
            value(Cell::VerticalSplit, char('|')),
            value(Cell::HorizontalSplit, char('-')),
        ));
        let row = ws_line(many1(cell));
        let grid = many1(row).map(|d| Grid::new(d));
        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "46")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "51")
    }
}
