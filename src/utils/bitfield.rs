#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedBitField<const SIZE: usize> {
    pub bytes: [u8; SIZE],
}

impl<const SIZE: usize> FixedBitField<SIZE> {
    /// Capacity
    const NUM_BYTES: usize = SIZE;
    const NUM_BITS: usize = Self::NUM_BYTES * 8;

    pub fn new() -> Self {
        Self { bytes: [0; SIZE] }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < Self::NUM_BITS);
        let byte = index / 8;
        let bit = 1 << (index % 8);
        self.bytes[byte] & bit > 0
    }

    pub fn set(&mut self, index: usize) -> bool {
        assert!(index < Self::NUM_BITS);
        let byte = index / 8;
        let bit = 1 << (index % 8);
        if self.bytes[byte] & bit == 0 {
            self.bytes[byte] |= bit;
            true
        } else {
            false
        }
    }

    pub fn unset(&mut self, index: usize) -> bool {
        assert!(index < Self::NUM_BITS);
        let byte = index / 8;
        let bit = 1 << (index % 8);
        if self.bytes[byte] & bit > 0 {
            self.bytes[byte] &= !bit;
            true
        } else {
            false
        }
    }
}

impl<const SIZE: usize> Default for FixedBitField<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
