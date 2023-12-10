use std::collections::VecDeque;

use ahash::AHashSet;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let (start, start_dirs) = find_start(&grid)?;
    if start_dirs.len() != 2 {
        bail!("start must connect to 2 points");
    }

    let mut w1 = Walker::new(&grid, start, start_dirs[0]);
    let mut w2 = Walker::new(&grid, start, start_dirs[1]);
    w1.step()?;
    w2.step()?;

    let mut count = 1;
    while w1.loc != w2.loc {
        w1.step()?;
        w2.step()?;
        count += 1;
    }

    Ok(count.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let (start, start_dirs) = find_start(&grid)?;
    if start_dirs.len() != 2 {
        bail!("start must connect to 2 points");
    }

    let mut loop_points = AHashSet::new();
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();
    let mut w = Walker::new(&grid, start, start_dirs[0]);

    loop {
        let prev_d = w.dir;
        w.step()?;
        loop_points.insert(w.loc);
        left_side.push(w.loc.next(prev_d.rotate_left()));
        right_side.push(w.loc.next(prev_d.rotate_right()));
        left_side.push(w.loc.next(w.dir.rotate_left()));
        right_side.push(w.loc.next(w.dir.rotate_right()));

        if w.loc == start {
            break;
        }
    }

    let eval_side = |xs: Vec<Option<Point>>| -> Option<usize> {
        let set: Option<AHashSet<Point>> = xs
            .into_iter()
            .filter(|p| match p {
                Some(x) => !loop_points.contains(x),
                None => true,
            })
            .collect();
        set.and_then(|s| fill(&s, &loop_points))
    };

    let left_ans = eval_side(left_side);
    let right_ans = eval_side(right_side);

    let ans = left_ans
        .xor(right_ans)
        .ok_or_else(|| anyhow!("one and only one side may be inside the loop"))?;

    Ok(ans.to_string())
}

fn find_start(grid: &Grid<Cell>) -> anyhow::Result<(Point, Vec<Direction>)> {
    let start = grid
        .iter_points()
        .find(|p| grid.get(*p) == Some(&Cell::Start))
        .ok_or(anyhow!("no start in grid"))?;

    let dirs = Direction::iter().filter(|&d| {
        start
            .next(d)
            .and_then(|p| grid.get(p))
            .map(|c| c.directions().contains(&d.opposite()))
            .unwrap_or(false)
    });

    Ok((start, dirs.collect()))
}

struct Walker<'a> {
    grid: &'a Grid<Cell>,
    loc: Point,
    dir: Direction,
    start_loc: Point,
    start_dir: Direction,
}

impl<'a> Walker<'a> {
    fn new(grid: &'a Grid<Cell>, loc: Point, dir: Direction) -> Self {
        Self {
            grid,
            loc,
            dir,
            start_loc: loc,
            start_dir: dir,
        }
    }

    fn step(&mut self) -> anyhow::Result<()> {
        // take step
        self.loc = self
            .loc
            .next(self.dir)
            .ok_or_else(|| anyhow!("walked to invalid point"))?;

        // determine next direction
        self.dir = if self.loc == self.start_loc {
            self.start_dir
        } else {
            let cell = self
                .grid
                .get(self.loc)
                .ok_or_else(|| anyhow!("walked off map"))?;

            cell.directions()
                .iter()
                .copied()
                .find(|&d| d != self.dir.opposite())
                .ok_or_else(|| anyhow!("point has no next direction"))?
        };

        Ok(())
    }
}

fn fill(start: &AHashSet<Point>, bounds: &AHashSet<Point>) -> Option<usize> {
    let mut frontier: VecDeque<Point> = start.iter().copied().collect();
    let mut seen: AHashSet<Point> = start.iter().copied().collect();

    while let Some(p) = frontier.pop_front() {
        for d in Direction::iter() {
            let Some(n) = p.next(d) else {
                // if we move off edge, we are not within the loop
                return None;
            };

            if !bounds.contains(&n) {
                if seen.insert(n) {
                    frontier.push_back(n);
                }
            }
        }
    }

    Some(seen.len())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Cell {
    fn directions(&self) -> &'static [Direction] {
        match self {
            Self::NS => &[Direction::Down, Direction::Up],
            Self::EW => &[Direction::Right, Direction::Left],
            Self::NE => &[Direction::Down, Direction::Right],
            Self::NW => &[Direction::Down, Direction::Left],
            Self::SW => &[Direction::Up, Direction::Left],
            Self::SE => &[Direction::Up, Direction::Right],
            Self::Ground => &[],
            Self::Start => &[],
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NS),
            '-' => Ok(Self::EW),
            'L' => Ok(Self::NE),
            'J' => Ok(Self::NW),
            '7' => Ok(Self::SW),
            'F' => Ok(Self::SE),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(anyhow!("unknown cell")),
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = one_of("|-LJ7F.S").map(|x| Cell::try_from(x).unwrap());
        let row = many1(cell);
        let graph = many1(ws_line(row)).map(Grid::new);
        ws_all_consuming(graph)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "8")
    }

    #[test]
    fn problem2_test() {
        let input = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(problem2(input).unwrap(), "10")
    }
}
