use bitvec::{bitvec, prelude::Lsb0, slice::IterOnes, vec::BitVec};

use crate::{BitSet, IndexMatrix, IndexSet, RcFamily};

impl BitSet for BitVec {
    type Iter<'a> = IterOnes<'a, usize, Lsb0>;

    fn empty(size: usize) -> Self {
        bitvec![usize, Lsb0; 0; size]
    }

    fn contains(&self, index: usize) -> bool {
        self[index]
    }

    fn insert(&mut self, index: usize) {
        self.set(index, true);
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter_ones()
    }

    fn len(&self) -> usize {
        self.count_ones()
    }

    fn union(&mut self, other: &Self) -> bool {
        let n = self.count_ones();
        *self |= other;
        self.count_ones() != n
    }

    fn intersect(&mut self, other: &Self) -> bool {
        let n = self.count_ones();
        *self &= other;
        self.count_ones() != n
    }

    fn invert(&mut self) {
        take_mut::take(self, |this| !this)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn subtract(&mut self, other: &Self) -> bool {
        let mut other_copy = other.clone();
        other_copy.invert();
        self.intersect(&other_copy)
    }
}

/// [`IndexSet`] specialized to the [`BitVec`] implementation.
pub type BitvecIndexSet<T> = IndexSet<T, BitVec, RcFamily>;

/// [`IndexMatrix`] specialized to the [`BitVec`] implementation.
pub type BitvecIndexMatrix<R, C> = IndexMatrix<R, C, BitVec, RcFamily>;

#[test]
fn test_bitvec() {
    crate::test_utils::impl_test::<BitVec>();
}
