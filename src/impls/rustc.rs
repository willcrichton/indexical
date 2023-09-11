extern crate rustc_driver;
extern crate rustc_index;
extern crate rustc_mir_dataflow;

use crate::{BitSet, IndexMatrix, IndexSet, IndexedValue, PointerFamily, RcFamily};
use rustc_mir_dataflow::JoinSemiLattice;
use std::hash::Hash;

pub type RustcBitSet = rustc_index::bit_set::BitSet<usize>;

impl BitSet for RustcBitSet {
    type Iter<'a> = rustc_index::bit_set::BitIter<'a, usize>;

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

    fn intersect(&mut self, other: &Self) -> bool {
        self.intersect(other)
    }

    fn len(&self) -> usize {
        self.count()
    }

    fn union(&mut self, other: &Self) -> bool {
        self.union(other)
    }

    fn subtract(&mut self, other: &Self) -> bool {
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
}

/// [`IndexSet`] specialized to the `rustc_index::bit_set::BitSet` implementation.
pub type RustcIndexSet<T> = IndexSet<T, RustcBitSet, RcFamily>;

/// [`IndexMatrix`] specialized to the `rustc_index::bit_set::BitSet` implementation.
pub type RustcIndexMatrix<R, C> = IndexMatrix<R, C, RustcBitSet, RcFamily>;

/// [`IndexMatrix`] specialized to the `rustc_index::bit_set::BitSet` implementation with the [`ArcFamily`].
pub type RustcIndexMatrix<R, C> = IndexMatrix<R, C, RustcBitSet, ArcFamily>;

impl<T, S, P> JoinSemiLattice for IndexSet<T, S, P>
where
    T: IndexedValue,
    S: BitSet,
    P: PointerFamily,
{
    fn join(&mut self, other: &Self) -> bool {
        self.union(other)
    }
}

impl<R, C, S, P> JoinSemiLattice for IndexMatrix<R, C, S, P>
where
    R: PartialEq + Eq + Hash + Clone,
    C: IndexedValue,
    S: BitSet,
    P: PointerFamily,
{
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (row, col) in other.matrix.iter() {
            changed |= self.ensure_row(row.clone()).union(col);
        }
        changed
    }
}

#[test]
fn test_rustc_bitset() {
    crate::test_utils::impl_test::<RustcBitSet>();
}
