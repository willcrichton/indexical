use std::fmt;

use index_vec::Idx;

use crate::{
    FromIndexicalIterator, IndexedDomain, IndexedValue, ToIndex, bitset::BitSet,
    pointer::PointerFamily,
};

/// An unordered collections of `T`s, implemented with a bit-set.
pub struct IndexSet<'a, T: IndexedValue + 'a, S: BitSet, P: PointerFamily<'a>> {
    set: S,
    domain: P::Pointer<IndexedDomain<T>>,
}

impl<'a, T, S, P> IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    /// Creates an empty index set.
    pub fn new(domain: &P::Pointer<IndexedDomain<T>>) -> Self {
        IndexSet {
            set: S::empty(domain.len()),
            domain: domain.clone(),
        }
    }

    /// Returns an iterator over all the indices contained in `self`.
    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = T::Index> + '_ {
        self.set.iter().map(T::Index::from_usize)
    }

    /// Returns an iterator over all the objects contained in `self`.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.indices().map(move |idx| self.domain.value(idx))
    }

    /// Returns an iterator over the pairs of indices and objects contained in `self`.
    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (T::Index, &T)> + use<'a, '_, T, S, P> {
        self.indices().map(move |idx| (idx, self.domain.value(idx)))
    }

    /// Returns true if `index` is contained in `self`.
    #[inline]
    pub fn contains<M>(&self, index: impl ToIndex<T, M>) -> bool {
        let elem = index.to_index(&self.domain);
        self.set.contains(elem.index())
    }

    /// Returns the number of elements in `self`.
    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Return true if `self` has no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if every element in `other` is also in `self`.
    #[inline]
    pub fn is_superset(&self, other: &IndexSet<'a, T, S, P>) -> bool {
        self.set.superset(&other.set)
    }

    /// Adds the element `elt` to `self`, returning true if `self` changed.
    #[inline]
    pub fn insert<M>(&mut self, elt: impl ToIndex<T, M>) -> bool {
        let elt = elt.to_index(&self.domain);
        self.set.insert(elt.index())
    }

    /// Adds each element of `other` to `self`.
    #[inline]
    pub fn union(&mut self, other: &IndexSet<'a, T, S, P>) {
        self.set.union(&other.set);
    }

    /// Adds each element of `other` to `self`, returning true if `self` changed.
    #[inline]
    pub fn union_changed(&mut self, other: &IndexSet<'a, T, S, P>) -> bool {
        self.set.union_changed(&other.set)
    }

    /// Removes every element of `other` from `self`.
    #[inline]
    pub fn subtract(&mut self, other: &IndexSet<'a, T, S, P>) {
        self.set.subtract(&other.set)
    }

    /// Removes every element of `other` from `self`, returning true if `self` changed.
    #[inline]
    pub fn subtract_changed(&mut self, other: &IndexSet<'a, T, S, P>) -> bool {
        self.set.subtract_changed(&other.set)
    }

    /// Removes every element of `self` not in `other`.
    #[inline]
    pub fn intersect(&mut self, other: &IndexSet<'a, T, S, P>) {
        self.set.intersect(&other.set)
    }

    /// Removes every element of `self` not in `other`, returning true if `self` changed.
    #[inline]
    pub fn intersect_changed(&mut self, other: &IndexSet<'a, T, S, P>) -> bool {
        self.set.intersect_changed(&other.set)
    }

    /// Adds every element of the domain to `self`.
    #[inline]
    pub fn insert_all(&mut self) {
        self.set.insert_all()
    }

    /// Removes every element from `self`.
    #[inline]
    pub fn clear(&mut self) {
        self.set.clear();
    }

    /// Returns a reference to the inner set.
    #[inline]
    pub fn inner(&self) -> &S {
        &self.set
    }
}

impl<'a, T, S, P> fmt::Debug for IndexSet<'a, T, S, P>
where
    T: IndexedValue + fmt::Debug + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<'a, T, S, P> PartialEq for IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}

impl<'a, T, S, P> Eq for IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
}

impl<'a, T, S, P> Clone for IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
{
    fn clone(&self) -> Self {
        IndexSet {
            set: self.set.clone(),
            domain: self.domain.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.set.copy_from(&source.set);
        self.domain = source.domain.clone();
    }
}

impl<'a, T, U, S, M, P> FromIndexicalIterator<'a, T, P, M, U> for IndexSet<'a, T, S, P>
where
    T: IndexedValue + 'a,
    S: BitSet,
    P: PointerFamily<'a>,
    U: ToIndex<T, M>,
{
    fn from_indexical_iter(
        iter: impl Iterator<Item = U>,
        domain: &P::Pointer<IndexedDomain<T>>,
    ) -> Self {
        let mut set = IndexSet::new(domain);
        for s in iter {
            set.insert(s);
        }
        set
    }
}

#[cfg(test)]
mod test {
    use crate::{IndexedDomain, IndexicalIteratorExt, test_utils::TestIndexSet};
    use std::rc::Rc;

    fn mk(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn test_indexset() {
        let d = Rc::new(IndexedDomain::from_iter([mk("a"), mk("b"), mk("c")]));
        let mut s = TestIndexSet::new(&d);
        s.insert(mk("a"));
        let b = d.index(&mk("b"));
        s.insert(b);
        assert!(s.contains(mk("a")));
        assert!(s.contains(mk("b")));
        assert_eq!(s.len(), 2);

        assert_eq!(
            [mk("a"), mk("b")]
                .into_iter()
                .collect_indexical::<TestIndexSet<_>>(&d),
            s
        );
        assert_eq!(format!("{s:?}"), r#"{"a", "b"}"#)
    }

    #[cfg(feature = "bitvec")]
    #[test]
    fn test_indexset_reffamily() {
        let d = &IndexedDomain::from_iter([mk("a"), mk("b"), mk("c")]);
        let mut s = crate::bitset::bitvec::RefIndexSet::new(&d);
        s.insert(mk("a"));
        assert!(s.contains(mk("a")));

        let s2 = s.clone();
        assert!(std::ptr::eq(s.domain, s2.domain));
    }
}
