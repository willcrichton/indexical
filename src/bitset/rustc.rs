//! The Rust compiler's [`BitSet`](https://doc.rust-lang.org/beta/nightly-rustc/rustc_index/bit_set/struct.BitSet.html).

extern crate rustc_driver;
pub extern crate rustc_index;
extern crate rustc_mir_dataflow;

use crate::{
    bitset::BitSet,
    pointer::{ArcFamily, PointerFamily, RcFamily, RefFamily},
    IndexedValue,
};
use rustc_mir_dataflow::JoinSemiLattice;
use std::hash::Hash;

pub use rustc_index::bit_set;

/// A bitset specialized to `usize` indices.
pub type RustcBitSet = bit_set::DenseBitSet<usize>;

impl BitSet for RustcBitSet {
    type Iter<'a> = bit_set::BitIter<'a, usize>;

    fn empty(size: usize) -> Self {
        RustcBitSet::new_empty(size)
    }

    fn contains(&self, index: usize) -> bool {
        self.contains(index)
    }

    fn insert(&mut self, index: usize) -> bool {
        self.insert(index)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn intersect(&mut self, other: &Self) {
        self.intersect(other);
    }

    fn intersect_changed(&mut self, other: &Self) -> bool {
        self.intersect(other)
    }

    fn len(&self) -> usize {
        self.count()
    }

    fn union(&mut self, other: &Self) {
        self.union(other);
    }

    fn union_changed(&mut self, other: &Self) -> bool {
        self.union(other)
    }

    fn subtract(&mut self, other: &Self) {
        self.subtract(other);
    }

    fn subtract_changed(&mut self, other: &Self) -> bool {
        self.subtract(other)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn invert(&mut self) {
        let mut inverted = RustcBitSet::new_filled(self.domain_size());
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

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation.
pub type IndexSet<T> = crate::IndexSet<'static, T, RustcBitSet, RcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation with the [`ArcFamily`].
pub type ArcIndexSet<'a, T> = crate::IndexSet<'a, T, RustcBitSet, ArcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the `bit_set::BitSet` implementation with the [`RefFamily`].
pub type RefIndexSet<'a, T> = crate::IndexSet<'a, T, RustcBitSet, RefFamily<'a>>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation.
pub type IndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, RustcBitSet, RcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation with the [`ArcFamily`].
pub type ArcIndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, RustcBitSet, ArcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the `bit_set::BitSet` implementation with the [`RefFamily`].
pub type RefIndexMatrix<'a, R, C> = crate::IndexMatrix<'a, R, C, RustcBitSet, RefFamily<'a>>;

impl<'a, T, S, P> JoinSemiLattice for crate::IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn join(&mut self, other: &Self) -> bool {
        self.union_changed(other)
    }
}

impl<'a, R, C, S, P> JoinSemiLattice for crate::IndexMatrix<'a, R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (row, col) in other.matrix.iter() {
            changed |= self.ensure_row(row.clone()).union_changed(col);
        }
        changed
    }
}

#[test]
fn test_rustc_bitset() {
    crate::test_utils::impl_test::<RustcBitSet>();
}
