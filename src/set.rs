use std::fmt;

use index_vec::Idx;

use crate::{BitSet, IndexedDomain, IndexedValue, PointerFamily, ToIndex};

/// An unordered collections of `T`s, implemented with a bit-set.
pub struct IndexSet<T: IndexedValue, S: BitSet, P: PointerFamily> {
  set: S,
  domain: P::Pointer<IndexedDomain<T>>,
}

impl<T, S, P> IndexSet<T, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  /// Creates an empty index set. 
  pub fn new(domain: &P::Pointer<IndexedDomain<T>>) -> Self {
    IndexSet {
      set: S::empty(domain.len()),
      domain: domain.clone(),
    }
  }
}

impl<T, S, P> IndexSet<T, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  /// Returns an iterator over all the indices contained in `self`.
  pub fn indices(&self) -> impl Iterator<Item = T::Index> + '_ {
    self.set.iter().map(T::Index::from_usize)
  }

  /// Returns an iterator over all the objects contained in `self`.
  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    self.indices().map(move |idx| self.domain.value(idx))
  }

  /// Returns an iterator over the pairs of indices and objects contained in `self`.
  pub fn iter_enumerated(&self) -> impl Iterator<Item = (T::Index, &T)> + '_ {
    self.indices().map(move |idx| (idx, self.domain.value(idx)))
  }

  /// Returns true if `index` is contained in `self`.
  pub fn contains<M>(&self, index: impl ToIndex<T, M>) -> bool {
    let elem = index.to_index(&self.domain);
    self.set.contains(elem.index())
  }

  /// Returns the number of elements in `self`.
  pub fn len(&self) -> usize {
    self.set.len()
  }

  // Return true if `self` has no elements.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns true if every element in `other` is also in `self`.
  pub fn is_superset(&self, other: &IndexSet<T, S, P>) -> bool {
    self.set.superset(&other.set)
  }

  /// Adds the element `elt` to `self`.
  pub fn insert<M>(&mut self, elt: impl ToIndex<T, M>) {
    let elt = elt.to_index(&self.domain);
    self.set.insert(elt.index());
  }

  /// Adds each element of `other` to `self`, returning true if `self` changed.
  pub fn union(&mut self, other: &IndexSet<T, S, P>) -> bool {
    self.set.union(&other.set)
  }

  /// Removes every element of `other` from `self`, returning true if `self` changed.
  pub fn subtract(&mut self, other: &IndexSet<T, S, P>) -> bool {
    self.set.subtract(&other.set)
  }

  /// Removes every element of `self` not in `other`, returning true if `self` changed.
  pub fn intersect(&mut self, other: &IndexSet<T, S, P>) -> bool {
    self.set.intersect(&other.set)
  }

  /// Adds every element of the domain to `self`.
  pub fn insert_all(&mut self) {
    self.set = S::empty(self.domain.len());
    self.set.invert();
  }

  /// Removes every element from `self`.
  pub fn clear(&mut self) {
    self.set.clear();
  }
}

impl<T, S, P> fmt::Debug for IndexSet<T, S, P>
where
  T: IndexedValue + fmt::Debug,
  S: BitSet,
  P: PointerFamily,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_set().entries(self.iter()).finish()
  }
}

impl<T, S, P> PartialEq for IndexSet<T, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  fn eq(&self, other: &Self) -> bool {
    self.set == other.set
  }
}

impl<T, S, P> Eq for IndexSet<T, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
}

impl<T, S, P> Clone for IndexSet<T, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  fn clone(&self) -> Self {
    IndexSet {
      set: self.set.clone(),
      domain: self.domain.clone(),
    }
  }

  fn clone_from(&mut self, source: &Self) {
    self.set.clone_from(&source.set);
    self.domain = source.domain.clone();
  }
}

/// Extension trait for iterators producing index sets.
pub trait IndexSetIteratorExt<T: IndexedValue, S: BitSet, P: PointerFamily, M> {
  /// Creates an [`IndexSet`] from an iterator over `T`s.
  /// 
  /// We cannot just use the normal `collect` method because this requires the domain as input.
  fn collect_indices(self, domain: &P::Pointer<IndexedDomain<T>>) -> IndexSet<T, S, P>;
}

impl<T, U, S, M, P, Iter> IndexSetIteratorExt<T, S, P, M> for Iter
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
  Iter: Iterator<Item = U>,
  U: ToIndex<T, M>,
{
  fn collect_indices(self, domain: &P::Pointer<IndexedDomain<T>>) -> IndexSet<T, S, P> {
    let mut set = IndexSet::new(domain);
    for s in self {
      set.insert(s);
    }
    set
  }
}

#[cfg(test)]
mod test {
  use crate::{set::IndexSetIteratorExt, test_utils::TestIndexSet, IndexedDomain};
  use std::rc::Rc;

  #[test]
  fn test_indexset() {
    let d = Rc::new(IndexedDomain::from_iter(["a", "b", "c"]));
    let mut s = TestIndexSet::new(&d);
    s.insert("a");
    let b = d.index(&"b");
    s.insert(b);
    assert!(s.contains("a"));
    assert!(s.contains("b"));
    assert_eq!(s.len(), 2);

    assert_eq!(["a", "b"].into_iter().collect_indices(&d), s);
    assert_eq!(format!("{s:?}"), r#"{"a", "b"}"#)
  }
}
