use std::collections::hash_map::Entry;
use std::fmt::{Debug, Write};
use std::mem;

use ahash::AHashMap;

use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let mut rg = RotatingGrid::from(&grid);
    rg.slide_up();

    Ok(rg.score().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let mut rg = RotatingGrid::from(&grid);
    let mut seen = AHashMap::new();

    let mut i = 0;
    let (repeat_start, repeat_len) = 'l: loop {
        match seen.entry(rg.values.clone()) {
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

    let mut rg = RotatingGrid::from(&grid);
    let cycles = repeat_start + (1_000_000_000 - repeat_start) % repeat_len;
    for _ in 0..cycles {
        rg.cycle();
    }

    Ok(rg.score().to_string())
}

struct RotatingGrid {
    cols: usize,
    rows: usize,
    values: Vec<(Point, Cell)>,
}

impl RotatingGrid {
    fn cycle(&mut self) {
        for _ in 0..4 {
            self.slide_up();
            self.rotate_clockwise();
        }
    }

    fn rotate_clockwise(&mut self) {
        for (p, _) in self.values.iter_mut() {
            *p = Point::new(self.cols - p.y - 1, p.x);
        }
        mem::swap(&mut self.cols, &mut self.rows);
    }

    fn slide_up(&mut self) {
        self.values.sort_unstable();
        let mut vs = self.values.as_mut_slice();
        while let Some((first, _)) = vs.first() {
            let n = vs.iter().take_while(|(p, _)| p.x == first.x).count();
            let mut next_open = 0;
            for (p, v) in vs[..n].iter_mut() {
                if v == &Cell::Round {
                    p.y = next_open;
                }
                next_open = p.y + 1;
            }
            vs = &mut vs[n..];
        }
    }

    fn score(&self) -> usize {
        self.values
            .iter()
            .filter(|(_, v)| v == &Cell::Round)
            .map(|(p, _)| self.rows - p.y)
            .sum()
    }
}

impl Debug for RotatingGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = self.values.clone();
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
        let values = grid
            .iter_points()
            .filter_map(|p| Some((p, grid.get(p).copied()?)).filter(|(_, v)| v != &Cell::Empty))
            .collect();
        Self { cols, rows, values }
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
