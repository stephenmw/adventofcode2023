use std::collections::hash_map::Entry;
use std::fmt::{Debug, Write};
use std::rc::Rc;

use ahash::AHashMap;

use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let mut rg = RotatingGrid::from(&grid);
    rg.slide_up(0);

    Ok(rg.score().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let mut rg = RotatingGrid::from(&grid);
    let mut history = Vec::new();
    let mut seen_locations = AHashMap::new();

    let mut i: usize = 0;
    let (repeat_start, repeat_len) = 'l: loop {
        let c = Rc::new(rg.round_rocks.clone());
        history.push(c.clone());
        match seen_locations.entry(c) {
            Entry::Occupied(e) => {
                break 'l (*e.get(), i - *e.get());
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        };

        rg.cycle();

        i += 1;
    };

    rg.round_rocks = (*history[repeat_start + (1_000_000_000 - repeat_start) % repeat_len]).clone();

    Ok(rg.score().to_string())
}

#[derive(Clone)]
struct RotatingGrid {
    cols: usize,
    rows: usize,
    round_rocks: Vec<Point>,
    square_rock_rotations: [Vec<Point>; 4],
}

impl RotatingGrid {
    fn new(cols: usize, rows: usize, values: &[(Point, Cell)]) -> Self {
        let round_rocks = values
            .iter()
            .filter_map(|&(p, v)| (v == Cell::Round).then_some(p))
            .collect();

        let mut square_rocks: Vec<_> = values
            .iter()
            .filter_map(|&(p, v)| (v == Cell::Square).then_some(p))
            .collect();
        square_rocks.sort_unstable();

        let mut square_rock_rotations = [square_rocks, Vec::new(), Vec::new(), Vec::new()];
        let mut c = cols;
        let mut r = rows;
        for i in 1..square_rock_rotations.len() {
            square_rock_rotations[i] = square_rock_rotations[i - 1].clone();
            rotate_clockwise(c, &mut square_rock_rotations[i]);
            square_rock_rotations[i].sort_unstable();
            std::mem::swap(&mut c, &mut r);
        }

        Self {
            cols,
            rows,
            round_rocks,
            square_rock_rotations,
        }
    }

    fn cycle(&mut self) {
        let mut cols = self.cols;
        let mut rows = self.rows;

        for i in 0..4 {
            self.slide_up(i);
            rotate_clockwise(cols, &mut self.round_rocks);
            std::mem::swap(&mut cols, &mut rows);
        }
    }

    fn slide_up(&mut self, orientation: usize) {
        self.round_rocks.sort_unstable();

        let mut round_rocks = self.round_rocks.iter_mut().peekable();
        let mut square_rocks = self.square_rock_rotations[orientation].iter().peekable();

        let mut cur_col = 0;
        let mut next_row = 0;

        loop {
            let next = match (round_rocks.peek(), square_rocks.peek()) {
                (Some(round), Some(square)) => {
                    if **round <= **square {
                        Rock::Round(round_rocks.next().unwrap())
                    } else {
                        Rock::Square(square_rocks.next().unwrap())
                    }
                }
                (Some(_), None) => Rock::Round(round_rocks.next().unwrap()),
                (None, Some(_)) => Rock::Square(square_rocks.next().unwrap()),
                (None, None) => break,
            };

            let p = next.point();
            if p.x != cur_col {
                cur_col = p.x;
                next_row = 0;
            }

            match next {
                Rock::Round(p) => {
                    p.y = next_row;
                    next_row += 1;
                }
                Rock::Square(p) => {
                    next_row = p.y + 1;
                }
            }
        }
    }

    fn score(&self) -> usize {
        self.round_rocks.iter().map(|p| (self.rows - p.y)).sum()
    }
}

impl Debug for RotatingGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::from_iter(
            self.round_rocks.iter().map(|&p| (p, Cell::Round)).chain(
                self.square_rock_rotations[0]
                    .iter()
                    .map(|&p| (p, Cell::Square)),
            ),
        );
        values.sort_unstable();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = values
                    .binary_search_by_key(&Point::new(col, row), |(p, _)| *p)
                    .map(|i| values[i].1)
                    .unwrap_or(Cell::Empty);
                f.write_char(match cell {
                    Cell::Empty => '.',
                    Cell::Square => '#',
                    Cell::Round => 'O',
                })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl From<&Grid<Cell>> for RotatingGrid {
    fn from(grid: &Grid<Cell>) -> Self {
        let (cols, rows) = grid.size();
        let values: Vec<_> = grid
            .iter_points()
            .filter_map(|p| Some((p, grid.get(p).copied()?)).filter(|(_, v)| v != &Cell::Empty))
            .collect();

        Self::new(cols, rows, &values)
    }
}

fn rotate_clockwise(cols: usize, points: &mut [Point]) {
    for p in points.iter_mut() {
        *p = Point::new(cols - p.y - 1, p.x);
    }
}

enum Rock<'a> {
    Square(&'a Point),
    Round(&'a mut Point),
}

impl<'a> Rock<'a> {
    fn point(&self) -> &Point {
        match self {
            Self::Square(p) => p,
            Self::Round(p) => p,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Round,
    Square,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            value(Cell::Empty, char('.')),
            value(Cell::Square, char('#')),
            value(Cell::Round, char('O')),
        ));
        let row = ws_line(many1(cell));
        let grid = many1(row).map(|rows| Grid::new(rows));
        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "136")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "64")
    }
}
