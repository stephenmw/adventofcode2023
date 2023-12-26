use std::collections::hash_map::Entry;

use ahash::AHashMap;

#[derive(Clone, Debug)]
pub struct IdAssigner<T> {
    m: AHashMap<T, usize>,
    pub next_id: usize,
}

impl<T> IdAssigner<T>
where
    T: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            m: AHashMap::default(),
            next_id: 0,
        }
    }

    pub fn lookup_or_assign(&mut self, value: T) -> usize {
        match self.m.entry(value) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let v = e.insert(self.next_id);
                self.next_id += 1;
                *v
            }
        }
    }
}

impl<T> Default for IdAssigner<T>
where
    T: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
