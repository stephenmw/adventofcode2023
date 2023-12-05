#[derive(Debug)]
pub struct HeapElement<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Ord, V> From<(K, V)> for HeapElement<K, V> {
    fn from(x: (K, V)) -> Self {
        HeapElement {
            key: x.0,
            value: x.1,
        }
    }
}

impl<K: Ord, V> std::cmp::PartialEq for HeapElement<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for HeapElement<K, V> {}

impl<K: Ord, V> Ord for HeapElement<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for HeapElement<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
