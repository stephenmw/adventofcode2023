use ahash::AHashMap;

use crate::{solutions::prelude::*, utils::lcm};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (dir, node_descs) = parse!(input);
    let nodes = build_nodes(&node_descs)?;

    let start = nodes
        .iter()
        .position(|x| x.id == "AAA")
        .ok_or_else(|| anyhow!("no starting node found"))?;
    let ans = moves_until_end(start, &nodes, &dir, |x| x == "ZZZ");

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (dir, node_descs) = parse!(input);
    let nodes = build_nodes(&node_descs)?;

    let starts = nodes
        .iter()
        .enumerate()
        .filter(|(_, x)| x.id.ends_with("A"));
    let cycle_lengths =
        starts.map(|(s, _)| moves_until_end(s, &nodes, &dir, |id| id.ends_with("Z")) as u64);
    let cycles_lcm = cycle_lengths
        .reduce(lcm)
        .ok_or_else(|| anyhow!("no starting nodes found"))?;

    Ok(cycles_lcm.to_string())
}

fn build_nodes(descs: &[NodeDesc]) -> Result<Vec<Node>, anyhow::Error> {
    let m: AHashMap<&str, usize> = descs
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.as_str(), i))
        .collect();

    descs
        .iter()
        .map(|d| {
            let left_id = d.left.as_str();
            let right_id = d.right.as_str();

            Ok(Node {
                id: &d.id,
                left: *m
                    .get(left_id)
                    .ok_or_else(|| anyhow!("node `{}` not declared", left_id))?,
                right: *m
                    .get(right_id)
                    .ok_or_else(|| anyhow!("node `{}` not declared", right_id))?,
            })
        })
        .collect()
}

fn moves_until_end(
    start: usize,
    nodes: &[Node],
    directions: &[Direction],
    predicate: impl Fn(&str) -> bool,
) -> usize {
    let mut cur = start;
    let mut count = 0;
    for &d in directions.iter().cycle() {
        if predicate(&nodes[cur].id) {
            break;
        }
        cur = nodes[cur].get(d);
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
struct NodeDesc {
    id: String,
    left: String,
    right: String,
}

#[derive(Clone, Debug)]
struct Node<'a> {
    id: &'a str,
    left: usize,
    right: usize,
}

impl<'a> Node<'a> {
    fn get(&self, d: Direction) -> usize {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Vec<NodeDesc>)> {
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
            .map(|(id, (left, right))| NodeDesc { id, left, right });

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
