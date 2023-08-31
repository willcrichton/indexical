use std::ops::DerefMut;

use crate::rustc::mir_dataflow::JoinSemiLattice;
use crate::{BitSet, IndexSet, IndexedValue, Owned, PointerFamily, RcFamily};

pub type RustcBitSet = crate::rustc::index::bit_set::BitSet<usize>;

impl BitSet for RustcBitSet {
  type Iter<'a> = crate::rustc::index::bit_set::BitIter<'a, usize>;

  fn empty(size: usize) -> Self {
    RustcBitSet::new_empty(size)
  }

  fn contains(&self, index: usize) -> bool {
    self.contains(index)
  }

  fn insert(&mut self, index: usize) {
    self.insert(index);
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

  fn invert(&mut self) {
    let mut inverted = RustcBitSet::new_filled(self.domain_size());
    inverted.subtract(self);
    *self = inverted;
  }
}

pub type RustcIndexSet<T> = IndexSet<T, Owned<RustcBitSet>, RustcBitSet, RcFamily>;

impl<T, O, S, P> JoinSemiLattice for IndexSet<T, O, S, P>
where
  T: IndexedValue,
  O: DerefMut<Target = S>,
  S: BitSet,
  P: PointerFamily,
{
  fn join(&mut self, other: &Self) -> bool {
    self.union(other)
  }
}

#[test]
fn test_rustc_bitset() {
  crate::test_utils::impl_test::<RustcBitSet>();
}
