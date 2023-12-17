use crate::utils::HeapElement;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FusedIterator;

#[derive(Debug)]
pub struct MergeIterator<I: Iterator<Item = V>, V> {
    heap: BinaryHeap<Reverse<HeapElement<V, I>>>,
}

impl<I: Iterator<Item = V>, V: Ord> MergeIterator<I, V> {
    pub fn new<A, B>(iterators: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: IntoIterator<IntoIter = I>,
    {
        let heap: BinaryHeap<_> = iterators
            .into_iter()
            .filter_map(|i| {
                let mut iter = i.into_iter();
                Some(Reverse(HeapElement::from((iter.next()?, iter))))
            })
            .collect();
        Self { heap }
    }
}

impl<I, V> Clone for MergeIterator<I, V>
where
    I: Iterator<Item = V> + Clone,
    V: Ord + Clone,
{
    fn clone(&self) -> Self {
        Self {
            heap: self.heap.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.heap.clone_from(&source.heap);
    }
}

impl<I: Iterator<Item = V>, V: Ord> Iterator for MergeIterator<I, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        {
            let cur = &mut self.heap.peek_mut()?.0;
            if let Some(next) = cur.value.next() {
                let ret = std::mem::replace(&mut cur.key, next);
                return Some(ret);
            }
        }

        let elem = self.heap.pop()?.0;
        Some(elem.key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size_hints = self.heap.iter().map(|e| e.0.value.size_hint());

        size_hints.fold((0, Some(0)), |acc, x| {
            let min = acc.0.saturating_add(x.0).saturating_add(1);
            let max = acc
                .1
                .zip(x.1)
                .map(|(a, b)| a.checked_add(b)?.checked_add(1))
                .flatten();
            (min, max)
        })
    }
}

impl<I: Iterator<Item = V>, V: Ord> FusedIterator for MergeIterator<I, V> {}

#[cfg(test)]
mod tests {
    use crate::utils::merge::MergeIterator;

    #[test]
    fn merge_test() {
        let m = MergeIterator::new(vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 5],
            vec![2, 3],
            vec![5],
            vec![],
        ]);
        let expected = vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 5];

        let merged: Vec<_> = m.collect();

        assert_eq!(merged, expected);
    }

    #[test]
    fn merge_size_hint() {
        let m = MergeIterator::new(vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 5],
            vec![2, 3],
            vec![5],
            vec![],
        ]);

        assert_eq!(m.size_hint(), (10, Some(10)));
    }
}
