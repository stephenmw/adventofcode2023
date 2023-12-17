mod bitfield;

pub use bitfield::FixedBitField;

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

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Stein's binary GCD algorithm
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if n == 0 {
        return m;
    } else if m == 0 {
        return n;
    }

    // Extract common factor-2: gcd(2ⁱ n, 2ⁱ m) = 2ⁱ gcd(n, m)
    // and reducing until odd gcd(2ⁱ n, m) = gcd(n, m) if m is odd
    let k = {
        let k_n = n.trailing_zeros();
        let k_m = m.trailing_zeros();
        n >>= k_n;
        m >>= k_m;
        std::cmp::min(k_n, k_m)
    };

    loop {
        // Invariant: n odd
        debug_assert!(n % 2 == 1, "n = {} is even", n);

        if n > m {
            std::mem::swap(&mut n, &mut m);
        }
        m -= n;

        if m == 0 {
            return n << k;
        }

        m >>= m.trailing_zeros();
    }
}

pub fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}
