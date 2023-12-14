use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = parse!(input);
    let round_stones: Vec<_> = grid
        .iter_points()
        .filter(|&p| grid.get(p).unwrap() == &Cell::Round)
        .collect();

    for p in round_stones {
        let new_p = grid
            .iter_line(p, Direction::Down)
            .skip(1)
            .take_while(|(_, &v)| v == Cell::Empty)
            .map(|(p, _)| p)
            .last()
            .unwrap_or(p);

        *grid.get_mut(p).unwrap() = Cell::Empty;
        *grid.get_mut(new_p).unwrap() = Cell::Round;
    }

    let round_stones = grid
        .iter_points()
        .filter(|&p| grid.get(p).unwrap() == &Cell::Round);

    let (_, rows) = grid.size();
    let ans: usize = round_stones.map(|p| rows - p.y).sum();

    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
