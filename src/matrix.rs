use fxhash::FxHashMap;
use index_vec::Idx;
use std::hash::Hash;
use std::ops::Deref;

use crate::{BitSet, IndexSet, IndexedDomain, IndexedValue, Owned, PointerFamily, ToIndex};

pub struct IndexMatrix<R, C: IndexedValue, S: BitSet, P: PointerFamily> {
  matrix: FxHashMap<R, IndexSet<C, Owned<S>, S, P>>,
  col_domain: P::Pointer<IndexedDomain<C>>,
}

impl<R, C, S, P> IndexMatrix<R, C, S, P>
where
  R: PartialEq + Eq + Hash,
  C: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  pub fn new(col_domain: &P::Pointer<IndexedDomain<C>>) -> Self {
    IndexMatrix {
      matrix: FxHashMap::default(),
      col_domain: col_domain.clone(),
    }
  }

  fn ensure_row(&mut self, row: R) -> &mut IndexSet<C, Owned<S>, S, P> {
    self
      .matrix
      .entry(row)
      .or_insert_with(|| IndexSet::new(&self.col_domain))
  }

  pub fn insert<M>(&mut self, row: R, col: impl ToIndex<C, M>) {
    let col = col.to_index(&self.col_domain);
    self.ensure_row(row).insert(col)
  }

  pub fn union_into_row(
    &mut self,
    into: R,
    from: &IndexSet<C, impl Deref<Target = S>, S, P>,
  ) -> bool {
    self.ensure_row(row).union(other)
  }
}
