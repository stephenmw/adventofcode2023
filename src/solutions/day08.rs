use ahash::AHashMap;

use crate::{solutions::prelude::*, utils::lcm};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (dir, nodes) = parse!(input);
    let node_map = AHashMap::from_iter(nodes.iter().map(|x| (x.id.as_str(), x)));

    let start = node_map.get("AAA").unwrap();
    let ans = moves_until_end(start, &node_map, &dir, |x| x == "ZZZ");

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (dir, nodes) = parse!(input);
    let node_map = AHashMap::from_iter(nodes.iter().map(|x| (x.id.as_str(), x)));

    let starts = nodes.iter().filter(|x| x.id.ends_with("A"));
    let cycle_lengths =
        starts.map(|s| moves_until_end(s, &node_map, &dir, |id| id.ends_with("Z")) as u64);
    let cycles_lcm = cycle_lengths
        .reduce(lcm)
        .ok_or_else(|| anyhow!("no starting nodes found"))?;

    Ok(cycles_lcm.to_string())
}

fn moves_until_end(
    start: &Node,
    nodes: &AHashMap<&str, &Node>,
    directions: &[Direction],
    predicate: impl Fn(&str) -> bool,
) -> usize {
    let mut cur = start;
    let mut count = 0;
    for &d in directions.iter().cycle() {
        if predicate(&cur.id) {
            break;
        }
        cur = nodes.get(cur.get(d)).unwrap();
        count += 1;
    }

    count
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn get(&self, d: Direction) -> &str {
        match d {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Vec<Node>)> {
        let dir = alt((
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        ));
        let dir_list = many1(dir);
        let node_id = || alphanumeric1.map(|x: &str| x.to_string());
        let node_value = delimited(
            char('('),
            separated_pair(node_id(), tag(", "), node_id()),
            char(')'),
        );
        let node = separated_pair(node_id(), tag(" = "), node_value)
            .map(|(id, (left, right))| Node { id, left, right });

        let nodes = many1(ws_line(node));
        let dir_line = ws_line(dir_list);

        let parser = separated_pair(dir_line, multispace1, nodes);
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    ";

    const EXAMPLE_INPUT2: &str = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "6")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT2).unwrap(), "6")
    }
}
