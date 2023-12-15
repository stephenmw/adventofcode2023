use std::num::Wrapping;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let instructions = input.trim().split(",");
    let ans: usize = instructions.map(|x| hash(x) as usize).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let instructions = parse!(input);

    let mut map = ElfHashMap::default();

    for inst in instructions.iter() {
        match inst {
            Instruction::Add(l) => map.add(l.clone()),
            Instruction::Remove(id) => map.remove(id),
        }
    }

    let ans: usize = map
        .buckets
        .iter()
        .enumerate()
        .flat_map(|(i, b)| b.iter().enumerate().map(move |(j, l)| (i, j, l)))
        .map(|(i, j, l)| (i + 1) * (j + 1) * l.length)
        .sum();

    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct ElfHashMap {
    buckets: Vec<Vec<Lens>>,
}

impl ElfHashMap {
    fn add(&mut self, l: Lens) {
        let bucket = &mut self.buckets[hash(&l.id) as usize];
        if let Some(i) = bucket.iter().position(|a| a.id == l.id) {
            bucket[i] = l;
        } else {
            bucket.push(l);
        }
    }

    fn remove(&mut self, id: &str) {
        let bucket = &mut self.buckets[hash(id) as usize];
        let Some(i) = bucket.iter().position(|l| l.id == id) else {
            return;
        };
        bucket.remove(i);
    }
}

impl Default for ElfHashMap {
    fn default() -> Self {
        Self {
            buckets: vec![Vec::new(); 256],
        }
    }
}

#[derive(Clone, Debug)]
struct Lens {
    id: String,
    length: usize,
}

fn hash(input: &str) -> u8 {
    let mut cur = Wrapping(0);
    for c in input.bytes() {
        cur += c;
        cur *= 17;
    }

    cur.0
}

enum Instruction {
    Add(Lens),
    Remove(String),
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        let add_inst = separated_pair(alpha1, char('='), uint).map(|(id, length)| {
            Instruction::Add(Lens {
                id: id.to_string(),
                length,
            })
        });
        let rem_inst =
            terminated(alpha1, char('-')).map(|id: &str| Instruction::Remove(id.to_string()));
        let inst = alt((add_inst, rem_inst));
        let parser = separated_list1(char(','), inst);
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "1320")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "145")
    }
}
