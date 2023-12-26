use rand::prelude::*;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let graph = parse!(input);
    loop {
        let ((a, b), conns) = karger(&graph);
        if conns <= 3 {
            return Ok((a * b).to_string());
        }
    }
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    Ok("Push the button".to_owned())
}

// Randomly determines a cut of the graph. Returns the size of two subgraphs
// and the number of connections between them.
fn karger(graph: &Graph) -> ((usize, usize), usize) {
    let mut edges = graph.edges.clone();
    let mut remaining_nodes = graph.num_nodes;
    let mut assignments = vec![None; graph.num_nodes];
    let mut next_id = 0;

    while remaining_nodes > 2 {
        let i = thread_rng().gen_range(0..edges.len());
        let (e1, e2) = edges[i];
        match (assignments[e1], assignments[e2]) {
            (Some(a), Some(b)) => {
                for assignment in assignments.iter_mut() {
                    if assignment == &Some(b) {
                        *assignment = Some(a);
                    }
                }
            }
            (Some(a), None) => assignments[e2] = Some(a),
            (None, Some(b)) => assignments[e1] = Some(b),
            (None, None) => {
                assignments[e1] = Some(next_id);
                assignments[e2] = Some(next_id);
                next_id += 1;
            }
        };

        edges.retain(|x| {
            let assign_a = assignments[x.0];
            let assign_b = assignments[x.1];

            assign_a.zip(assign_b).map(|(a, b)| a != b).unwrap_or(true)
        });

        remaining_nodes -= 1;
    }

    let a_count = assignments.iter().filter(|&&x| x == assignments[0]).count();
    let b_count = assignments.len() - a_count;
    ((a_count, b_count), edges.len())
}

struct Graph {
    edges: Vec<(usize, usize)>,
    num_nodes: usize,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;
    use crate::utils::IdAssigner;

    pub fn parse(input: &str) -> IResult<&str, Graph> {
        let node_id = alpha1;
        let line = separated_pair(node_id, tag(": "), separated_list1(space1, node_id));

        let graph = many1(ws_line(line)).map(|lines| {
            let mut id_assigner = IdAssigner::default();
            let mut edges = Vec::new();

            for (a, bs) in lines {
                for b in bs {
                    edges.push((
                        id_assigner.lookup_or_assign(a),
                        id_assigner.lookup_or_assign(b),
                    ));
                }
            }

            Graph {
                edges,
                num_nodes: id_assigner.next_id,
            }
        });

        ws_all_consuming(graph)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "54")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
