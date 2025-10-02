//! The Rust compiler's [`BitSet`](https://doc.rust-lang.org/beta/nightly-rustc/rustc_index/bit_set/struct.BitSet.html).

extern crate rustc_driver;
pub extern crate rustc_index;
extern crate rustc_mir_dataflow;

use crate::{
    IndexedValue,
    bitset::BitSet,
    pointer::{ArcFamily, PointerFamily, RcFamily, RefFamily},
};
use rustc_mir_dataflow::JoinSemiLattice;
use std::hash::Hash;

pub use rustc_index::bit_set::{ChunkedBitSet, DenseBitSet, MixedBitIter, MixedBitSet};

/// A bitset specialized to `usize` indices.
pub type RustcBitSet = MixedBitSet<usize>;

impl BitSet for RustcBitSet {
    fn empty(size: usize) -> Self {
        RustcBitSet::new_empty(size)
    }

    fn contains(&self, index: usize) -> bool {
        self.contains(index)
    }

    fn insert(&mut self, index: usize) -> bool {
        self.insert(index)
    }

    fn remove(&mut self, index: usize) -> bool {
        self.remove(index)
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        self.iter()
    }

    fn intersect(&mut self, other: &Self) {
        self.intersect_changed(other);
    }

    fn intersect_changed(&mut self, other: &Self) -> bool {
        // TODO: this code should be removed once the corresponding implementation
        // has been added to rustc
        match (self, other) {
            (MixedBitSet::Large(self_large), MixedBitSet::Large(other_large)) => {
                ChunkedBitSet::intersect(self_large, other_large)
            }
            (MixedBitSet::Small(self_small), MixedBitSet::Small(other_small)) => {
                DenseBitSet::intersect(self_small, other_small)
            }
            _ => panic!("Mismatched domains"),
        }
    }

    fn len(&self) -> usize {
        self.iter().count()
    }

    fn union(&mut self, other: &Self) {
        self.union_changed(other);
    }

    fn union_changed(&mut self, other: &Self) -> bool {
        self.union(other)
    }

    fn subtract(&mut self, other: &Self) {
        self.subtract_changed(other);
    }

    fn subtract_changed(&mut self, other: &Self) -> bool {
        self.subtract(other)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn invert(&mut self) {
        let mut inverted = RustcBitSet::new_empty(self.domain_size());
        inverted.insert_all();
        inverted.subtract(self);
        *self = inverted;
    }

    fn insert_all(&mut self) {
        self.insert_all();
    }

    fn copy_from(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation with the [`RcFamily`].
pub type RcIndexSet<T> = crate::set::IndexSet<'static, T, RustcBitSet, RcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation with the [`ArcFamily`].
pub type ArcIndexSet<T> = crate::set::IndexSet<'static, T, RustcBitSet, ArcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation with the [`RefFamily`].
pub type RefIndexSet<'a, T> = crate::set::IndexSet<'a, T, RustcBitSet, RefFamily<'a>>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation with the [`RcFamily`].
pub type RcIndexMatrix<R, C> = crate::matrix::IndexMatrix<'static, R, C, RustcBitSet, RcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation with the [`ArcFamily`].
pub type ArcIndexMatrix<R, C> = crate::matrix::IndexMatrix<'static, R, C, RustcBitSet, ArcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation with the [`RefFamily`].
pub type RefIndexMatrix<'a, R, C> =
    crate::matrix::IndexMatrix<'a, R, C, RustcBitSet, RefFamily<'a>>;

impl<'a, T, S, P> JoinSemiLattice for crate::set::IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn join(&mut self, other: &Self) -> bool {
        self.union_changed(other)
    }
}

impl<'a, R, C, S, P> JoinSemiLattice for crate::matrix::IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (row, col) in &other.matrix {
            changed |= self.ensure_row(row.clone()).union_changed(col);
        }
        changed
    }
}

#[test]
fn test_rustc_bitset() {
    crate::test_utils::impl_test::<RustcBitSet>();
}
