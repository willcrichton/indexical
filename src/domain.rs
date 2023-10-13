use fxhash::FxHashMap as HashMap;
use index_vec::{Idx, IndexVec};
use std::fmt;

use crate::IndexedValue;

/// An indexed collection of objects.
///
/// Contains a reverse-mapping from `T` to `T::Index` for efficient lookups of indices.
pub struct IndexedDomain<T: IndexedValue> {
    domain: IndexVec<T::Index, T>,
    reverse_map: HashMap<T, T::Index>,
}

impl<T: IndexedValue> IndexedDomain<T> {
    /// Creates a new domain from an indexed vector.
    ///
    /// Consider using the [`FromIterator`] implementation if you don't want to manually construct
    /// an [`IndexVec`] object.
    #[inline]
    pub fn new(domain: IndexVec<T::Index, T>) -> Self {
        let reverse_map = domain
            .iter_enumerated()
            .map(|(idx, value)| (value.clone(), idx))
            .collect();
        IndexedDomain {
            domain,
            reverse_map,
        }
    }

    /// Gets the object corresponding to `index`.
    ///
    /// Panics if `index` is not within the domain.
    #[inline]
    pub fn value(&self, index: T::Index) -> &T {
        &self.domain[index]
    }

    /// Gets the index corresponding to `value`.
    ///
    /// Panics if `value` is not within the domain.
    #[inline]
    pub fn index(&self, value: &T) -> T::Index {
        self.reverse_map[value]
    }

    /// Returns true if `value` is contained in the domain.
    #[inline]
    pub fn contains(&self, value: &T) -> bool {
        self.reverse_map.contains_key(value)
    }

    /// Adds `value` to the domain, returning its new index.
    #[inline]
    pub fn insert(&mut self, value: T) -> T::Index {
        self.domain.push(value)
    }

    /// Returns immutable access to the underlying indexed vector.
    #[inline]
    pub fn as_vec(&self) -> &IndexVec<T::Index, T> {
        &self.domain
    }

    /// Returns the number of elements in the domain.
    #[inline]
    pub fn len(&self) -> usize {
        self.domain.len()
    }

    /// Returns true if the domain is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Similar to [`IndexedDomain::index`], except it adds `value`
    /// to the domain if it does not exist yet.
    #[inline]
    pub fn ensure(&mut self, value: &T) -> T::Index {
        if !self.contains(value) {
            self.insert(value.clone())
        } else {
            self.index(value)
        }
    }

    /// Returns an iterator over all elements of the domain.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.domain.iter()
    }

    /// Returns an iterator over all indices of the domain.
    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = T::Index> {
        // Re-implementing `indices` because profiling suggests that
        // the IndexVec impl is not getting inlined for some reason??
        (0..self.domain.len()).map(T::Index::from_usize)
    }

    /// Returns an iterator over all pairs of indices and elements of the domain.
    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (T::Index, &T)> + '_ {
        self.domain.iter_enumerated()
    }
}

impl<T: IndexedValue> FromIterator<T> for IndexedDomain<T> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let domain = iter.into_iter().collect();
        IndexedDomain::new(domain)
    }
}

impl<T: IndexedValue + fmt::Debug> fmt::Debug for IndexedDomain<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.domain)
    }
}

#[test]
fn test_domain() {
    fn mk(s: &str) -> String {
        s.to_string()
    }

    let d = IndexedDomain::from_iter([mk("a"), mk("b")]);
    let a = d.index(&mk("a"));
    let b = d.index(&mk("b"));
    assert_eq!(d.value(a), "a");
    assert_eq!(d.value(b), "b");
    assert!(d.contains(&mk("a")));
    assert!(!d.contains(&mk("c")));
    assert_eq!(d.len(), 2);
}
