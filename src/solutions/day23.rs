use ahash::AHashSet;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;
use crate::utils::IdAssigner;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let graph = build_graph(&grid, true)?;
    longest_path(&graph).map(|x| x.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let graph = build_graph(&grid, false)?;
    longest_path(&graph).map(|x| x.to_string())
}

fn longest_path(graph: &Graph) -> anyhow::Result<usize> {
    fn rec(graph: &Graph, cur: usize, seen: &mut [bool]) -> Option<usize> {
        if cur == graph.end_id {
            return Some(0);
        }

        if seen[cur] {
            return None;
        }
        seen[cur] = true;

        let max_steps = graph.nodes[cur]
            .edges
            .iter()
            .filter_map(|edge| rec(graph, edge.to, seen).map(|x| x + edge.distance))
            .max();

        seen[cur] = false;

        max_steps
    }

    rec(
        graph,
        graph.start_id,
        vec![false; graph.nodes.len()].as_mut_slice(),
    )
    .ok_or_else(|| anyhow!("no solution"))
}

fn build_graph(grid: &Grid<Cell>, slippery: bool) -> anyhow::Result<Graph> {
    let (cols, rows) = grid.size();
    let start = (0..cols)
        .map(|c| Point::new(c, 0))
        .find(|p| grid.get(*p) == Some(&Cell::Empty))
        .ok_or_else(|| anyhow!("cannot find start"))?;
    let end = (0..cols)
        .map(|c| Point::new(c, rows - 1))
        .find(|p| grid.get(*p) == Some(&Cell::Empty))
        .ok_or_else(|| anyhow!("cannot find end"))?;

    let mut frontier = vec![start];
    let mut nodes = Vec::new();

    let mut id_assigner = IdAssigner::new();
    let start_id = id_assigner.lookup_or_assign(start);
    let end_id = id_assigner.lookup_or_assign(end);

    let mut seen_intersections = AHashSet::default();
    seen_intersections.insert(start);

    while let Some(intersection) = frontier.pop() {
        let id = id_assigner.lookup_or_assign(intersection);

        let edges = Direction::iter()
            .filter_map(|d| {
                let (to, distance) = walk_straight(grid, intersection, d, slippery)?;

                if seen_intersections.insert(to) {
                    frontier.push(to);
                }

                Some(Edge {
                    to: id_assigner.lookup_or_assign(to),
                    distance,
                })
            })
            .collect();

        if id >= nodes.len() {
            nodes.resize(id + 1, Node::default());
        }
        nodes[id] = Node { edges }
    }

    Ok(Graph {
        nodes,
        start_id,
        end_id,
    })
}

// Walk until reaching another intersection.
fn walk_straight(
    grid: &Grid<Cell>,
    start: Point,
    dir: Direction,
    slippery: bool,
) -> Option<(Point, usize)> {
    let (_, rows) = grid.size();

    let mut last = start;
    let mut cur = start.next(dir)?;
    let mut steps = 1;

    // Check first step is valid
    match grid.get(cur)? {
        Cell::Empty => (),
        Cell::Slope(d) => {
            if d != &dir && slippery {
                return None;
            }
        }
        Cell::Wall => return None,
    }

    loop {
        if cur.y == 0 || cur.y == rows - 1 {
            // We have reached a start/end node.
            return Some((cur, steps));
        }

        let mut next_places = Direction::iter()
            .map(|d| (d, cur.next(d).unwrap()))
            .filter(|(dir, p)| match grid.get(*p).unwrap() {
                Cell::Empty => true,
                Cell::Slope(d) => d == dir || !slippery,
                Cell::Wall => false,
            })
            .filter(|(_, p)| p != &last);

        let Some((_, next_loc)) = next_places.next() else {
            // We have reached a dead end. Dead end nodes can be discarded.
            return None;
        };

        if next_places.next().is_some() {
            // We have reached an intersection
            return Some((cur, steps));
        }

        last = cur;
        cur = next_loc;
        steps += 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph {
    nodes: Vec<Node>,

    start_id: usize,
    end_id: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Node {
    edges: Vec<Edge>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    to: usize,
    distance: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Slope(Direction),
    Wall,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            value(Cell::Empty, char('.')),
            value(Cell::Wall, char('#')),
            value(Cell::Slope(Direction::Left), char('<')),
            value(Cell::Slope(Direction::Right), char('>')),
            value(Cell::Slope(Direction::Up), char('v')),
            value(Cell::Slope(Direction::Down), char('^')),
        ));
        let row = ws_line(many1(cell));
        let grid = many1(row).map(|d| Grid::new(d));
        ws_all_consuming(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    ";

    #[test]
    fn walk_straight_test() {
        let grid = parser::parse(EXAMPLE_INPUT).finish().unwrap().1;
        let res = walk_straight(&grid, Point::new(1, 0), Direction::Up, true);
        assert_eq!(res, Some((Point::new(3, 5), 15)));
    }

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "94")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "154")
    }
}
