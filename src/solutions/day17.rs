use std::collections::VecDeque;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

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

    let mut seen = vec![false; WalkState::max_int_state(cols, rows, max_dir)];
    let mut frontier = RollingPriorityQueue::default();

    let start = WalkState {
        loc: Point::new(0, 0),
        dir: Direction::Right,
        straight_count: 0,
    };

    for state in start.iter_next(0, 1) {
        frontier.push(0, state);
    }

    while let Some((cost, state)) = frontier.pop() {
        let Some(&additional_cost) = grid.get(state.loc) else {
            continue;
        };

        let new_cost = cost + additional_cost as usize;

        if state.loc == target_point && state.straight_count >= min_dir {
            return new_cost;
        }

        for next_state in state.iter_next(min_dir, max_dir) {
            let Some(s_index) = next_state.as_int(cols, rows, max_dir) else {
                continue;
            };
            if !seen[s_index] {
                seen[s_index] = true;
                frontier.push(new_cost, next_state);
            }
        }
    }

    unreachable!("no solution");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    fn as_int(&self, cols: usize, rows: usize, max_dir: u8) -> Option<usize> {
        let max_dir = max_dir as usize + 1;

        (self.loc.x < cols && self.loc.y < rows).then(|| {
            self.loc.x * rows * max_dir * 4
                + self.loc.y * max_dir * 4
                + self.straight_count as usize * 4
                + self.dir as usize
        })
    }

    fn max_int_state(cols: usize, rows: usize, max_dir: u8) -> usize {
        let max_dir = max_dir as usize + 1;
        cols * rows * max_dir * 4
    }
}

#[derive(Clone, Debug)]
struct RollingPriorityQueue<V> {
    queue: VecDeque<Vec<V>>,
    min_key: usize,
}

impl<V> RollingPriorityQueue<V> {
    fn push(&mut self, k: usize, v: V) {
        let i = k
            .checked_sub(self.min_key)
            .expect("RollingPriorityQueue: priority must be same or greater than minimum");
        if i >= self.queue.len() {
            self.queue.resize_with(i + 1, Vec::new);
        }
        self.queue[i].push(v);
    }

    fn pop(&mut self) -> Option<(usize, V)> {
        let i = self.queue.iter().position(|x| !x.is_empty())?;
        let k = i + self.min_key;
        let v = self.queue[i].pop()?;

        self.min_key += i;
        self.queue.rotate_left(i);

        Some((k, v))
    }
}

impl<V> Default for RollingPriorityQueue<V> {
    fn default() -> Self {
        RollingPriorityQueue {
            queue: VecDeque::default(),
            min_key: 0,
        }
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
