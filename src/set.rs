use index_vec::Idx;
use std::ops::{Deref, DerefMut};

use crate::{BitSet, IndexedDomain, IndexedValue, PointerFamily, ToIndex};

#[derive(Clone)]
pub struct Owned<T>(T);
#[derive(Clone, Copy)]
pub struct Ref<'a, T>(&'a T);
pub struct Mut<'a, T>(&'a mut T);

impl<T> Deref for Owned<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Owned<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> Deref for Ref<'_, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<T> Deref for Mut<'_, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<T> DerefMut for Mut<'_, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0
  }
}

pub struct IndexSet<T: IndexedValue, O: Deref<Target = S>, S: BitSet, P: PointerFamily> {
  set: O,
  domain: P::Pointer<IndexedDomain<T>>,
}

impl<T, S, P> IndexSet<T, Owned<S>, S, P>
where
  T: IndexedValue,
  S: BitSet,
  P: PointerFamily,
{
  pub fn new(domain: &P::Pointer<IndexedDomain<T>>) -> Self {
    IndexSet {
      set: Owned(S::empty(domain.len())),
      domain: domain.clone(),
    }
  }
}

impl<T, O, S, P> IndexSet<T, O, S, P>
where
  T: IndexedValue,
  O: Deref<Target = S>,
  S: BitSet,
  P: PointerFamily,
{
  pub fn indices(&self) -> impl Iterator<Item = T::Index> + '_ {
    self.set.iter().map(T::Index::from_usize)
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    self.indices().map(move |idx| self.domain.value(idx))
  }

  pub fn iter_enumerated(&self) -> impl Iterator<Item = (T::Index, &T)> + '_ {
    self.indices().map(move |idx| (idx, self.domain.value(idx)))
  }

  pub fn contains<M>(&self, index: impl ToIndex<T, M>) -> bool {
    let elem = index.to_index(&self.domain);
    self.set.contains(elem.index())
  }

  pub fn len(&self) -> usize {
    self.set.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn is_superset(&self, other: &IndexSet<T, impl Deref<Target = S>, S, P>) -> bool {
    self.set.superset(&*other.set)
  }
}

impl<T, O, S, P> IndexSet<T, O, S, P>
where
  T: IndexedValue,
  O: DerefMut<Target = S>,
  S: BitSet,
  P: PointerFamily,
{
  pub fn insert<M>(&mut self, elt: impl ToIndex<T, M>) {
    let elt = elt.to_index(&self.domain);
    self.set.insert(elt.index());
  }

  pub fn union(&mut self, other: &IndexSet<T, impl Deref<Target = S>, S, P>) -> bool {
    self.set.union(&*other.set)
  }

  pub fn subtract(&mut self, other: &IndexSet<T, impl Deref<Target = S>, S, P>) -> bool {
    self.set.subtract(&*other.set)
  }

  pub fn intersect(&mut self, other: &IndexSet<T, impl Deref<Target = S>, S, P>) -> bool {
    self.set.intersect(&*other.set)
  }

  pub fn insert_all(&mut self) {
    *self.set = S::empty(self.domain.len());
    self.set.invert();
  }
}

impl<T, O, S, P> PartialEq for IndexSet<T, O, S, P>
where
  T: IndexedValue,
  O: Deref<Target = S>,
  S: BitSet,
  P: PointerFamily,
{
  fn eq(&self, other: &Self) -> bool {
    *self.set == *other.set
  }
}

impl<T, O, S, P> Eq for IndexSet<T, O, S, P>
where
  T: IndexedValue,
  O: Deref<Target = S>,
  S: BitSet,
  P: PointerFamily,
{
}

impl<T, S, P> Clone for IndexSet<T, Owned<S>, S, P>
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

#[cfg(test)]
mod test {
  use crate::{test_utils::TestIndexSet, IndexedDomain};
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
  }
}
