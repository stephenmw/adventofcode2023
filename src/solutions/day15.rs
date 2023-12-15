use std::{collections::hash_map::Entry, num::Wrapping};

use ahash::AHashMap;

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
            Instruction::Add(l) => map.add(&l.id, l.length),
            Instruction::Remove(id) => map.remove(id),
        }
    }

    let ans: usize = map
        .buckets
        .iter()
        .enumerate()
        .flat_map(|(i, b)| b.values().enumerate().map(move |(j, v)| (i, j, v)))
        .map(|(i, j, v)| (i + 1) * (j + 1) * v)
        .sum();

    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct ElfHashMap<'a> {
    buckets: Vec<OrderedMap<'a>>,
}

impl<'a> ElfHashMap<'a> {
    fn add(&mut self, k: &'a str, v: usize) {
        let bucket = &mut self.buckets[hash(k) as usize];
        bucket.insert(k, v);
    }

    fn remove(&mut self, k: &'a str) {
        let bucket = &mut self.buckets[hash(k) as usize];
        bucket.remove(k);
    }
}

impl<'a> Default for ElfHashMap<'a> {
    fn default() -> Self {
        Self {
            buckets: vec![OrderedMap::default(); 256],
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

#[derive(Clone, Debug)]
enum Instruction {
    Add(Lens),
    Remove(String),
}

#[derive(Clone, Debug, Default)]
struct OrderedMap<'a> {
    m: AHashMap<&'a str, usize>,
    l: Vec<OrderedMapElem<'a>>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<'a> OrderedMap<'a> {
    fn insert(&mut self, k: &'a str, v: usize) {
        match self.m.entry(k) {
            Entry::Occupied(e) => self.l[*e.get()].value = v,
            Entry::Vacant(e) => {
                let i = self.l.len();
                let elem = OrderedMapElem {
                    key: k,
                    value: v,
                    prev: self.tail,
                    next: None,
                };
                self.l.push(elem);
                e.insert(i);

                if let Some(tail) = self.tail {
                    self.l[tail].next = Some(i);
                }

                self.tail = Some(i);
                if self.head.is_none() {
                    self.head = Some(i);
                }
            }
        };
    }

    fn remove(&mut self, k: &'a str) {
        let Some(i) = self.m.remove(k) else {
            return;
        };

        if let Some(prev) = self.l[i].prev {
            self.l[prev].next = self.l[i].next;
        } else {
            self.head = self.l[i].next;
        }

        if let Some(next) = self.l[i].next {
            self.l[next].prev = self.l[i].prev;
        } else {
            self.tail = self.l[i].prev
        }

        self.l.swap_remove(i);

        if i < self.l.len() {
            if let Some(prev) = self.l[i].prev {
                self.l[prev].next = Some(i);
            } else {
                self.head = Some(i);
            }

            if let Some(next) = self.l[i].next {
                self.l[next].prev = Some(i);
            } else {
                self.tail = Some(i);
            }

            self.m.insert(self.l[i].key, i);
        }
    }

    fn values(&'a self) -> OrderedMapValuesIterator<'a> {
        OrderedMapValuesIterator {
            m: &self,
            next_index: self.head,
        }
    }
}

struct OrderedMapValuesIterator<'a> {
    m: &'a OrderedMap<'a>,
    next_index: Option<usize>,
}

impl<'a> Iterator for OrderedMapValuesIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.next_index?;
        self.next_index = self.m.l[i].next;
        Some(self.m.l[i].value)
    }
}

#[derive(Clone, Debug)]
struct OrderedMapElem<'a> {
    key: &'a str,
    value: usize,
    prev: Option<usize>,
    next: Option<usize>,
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
