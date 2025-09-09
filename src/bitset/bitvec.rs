//! A bit-set from the [`bitvec`] crate.

use bitvec::{prelude::Lsb0, store::BitStore};

use crate::{
    bitset::BitSet,
    pointer::{ArcFamily, RcFamily, RefFamily},
};

pub use ::bitvec::{self, vec::BitVec};

impl BitSet for BitVec {
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

    fn remove(&mut self, index: usize) -> bool {
        let contained = self[index];
        self.set(index, false);
        contained
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
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
        // Inline defn of Not::not bc it assumes ownership of the BitVec
        for elem in self.as_raw_mut_slice() {
            elem.store_value(!elem.load_value());
        }
    }

    fn clear(&mut self) {
        for elem in self.as_raw_mut_slice() {
            elem.store_value(0);
        }
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

/// [`IndexSet`](crate::IndexSet) specialized to the [`BitVec`] implementation.
pub type IndexSet<T> = crate::IndexSet<'static, T, BitVec, RcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`BitVec`] implementation with the [`ArcFamily`].
pub type ArcIndexSet<T> = crate::IndexSet<'static, T, BitVec, ArcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`BitVec`] implementation with the [`RefFamily`].
pub type RefIndexSet<'a, T> = crate::IndexSet<'a, T, BitVec, RefFamily<'a>>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`BitVec`] implementation.
pub type IndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, BitVec, RcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`BitVec`] implementation with the [`ArcFamily`].
pub type ArcIndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, BitVec, ArcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`BitVec`] implementation with the [`RefFamily`].
pub type RefIndexMatrix<'a, R, C> = crate::IndexMatrix<'a, R, C, BitVec, RefFamily<'a>>;

#[test]
fn test_bitvec() {
    crate::test_utils::impl_test::<BitVec>();
}
