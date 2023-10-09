use bitvec::{prelude::Lsb0, slice::IterOnes, vec::BitVec};

use crate::{ArcFamily, BitSet, IndexMatrix, IndexSet, RcFamily, RefFamily};

pub use bitvec;

impl BitSet for BitVec {
    type Iter<'a> = IterOnes<'a, usize, Lsb0>;

    fn empty(size: usize) -> Self {
        bitvec::bitvec![usize, Lsb0; 0; size]
    }

    fn contains(&self, index: usize) -> bool {
        self[index]
    }

    fn insert(&mut self, index: usize) -> bool {
        let contained = self[index];
        self.set(index, true);
        !contained
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter_ones()
    }

    fn len(&self) -> usize {
        self.count_ones()
    }

    fn union(&mut self, other: &Self) {
        *self |= other;
    }

    fn intersect(&mut self, other: &Self) {
        *self &= other;
    }

    fn invert(&mut self) {
        take_mut::take(self, |this| !this)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn subtract(&mut self, other: &Self) {
        let mut other_copy = other.clone();
        other_copy.invert();
        self.intersect(&other_copy);
    }

    fn insert_all(&mut self) {
        self.fill(true);
    }

    fn copy_from(&mut self, other: &Self) {
        self.copy_from_bitslice(other);
    }
}

/// [`IndexSet`] specialized to the [`BitVec`] implementation.
pub type BitvecIndexSet<T> = IndexSet<'static, T, BitVec, RcFamily>;

/// [`IndexSet`] specialized to the [`BitVec`] implementation with the [`ArcFamily`].
pub type BitvecArcIndexSet<'a, T> = IndexSet<'a, T, BitVec, ArcFamily>;

/// [`IndexSet`] specialized to the [`BitVec`] implementation with the [`RefFamily`].
pub type BitvecRefIndexSet<'a, T> = IndexSet<'a, T, BitVec, RefFamily<'a>>;

/// [`IndexMatrix`] specialized to the [`BitVec`] implementation.
pub type BitvecIndexMatrix<R, C> = IndexMatrix<'static, R, C, BitVec, RcFamily>;

/// [`IndexMatrix`] specialized to the [`BitVec`] implementation with the [`ArcFamily`].
pub type BitvecArcIndexMatrix<R, C> = IndexMatrix<'static, R, C, BitVec, ArcFamily>;

/// [`IndexMatrix`] specialized to the [`BitVec`] implementation with the [`RefFamily`].
pub type BitvecRefIndexMatrix<'a, R, C> = IndexMatrix<'a, R, C, BitVec, RefFamily<'a>>;

#[test]
fn test_bitvec() {
    crate::test_utils::impl_test::<BitVec>();
}
