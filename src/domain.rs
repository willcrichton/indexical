use index_vec::{Idx, IndexVec};
use rustc_hash::FxHashMap;
use std::fmt;

use crate::IndexedValue;

/// An indexed collection of objects.
///
/// Contains a reverse-mapping from `T` to `T::Index` for efficient lookups of indices.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IndexedDomain<T: IndexedValue> {
    domain: IndexVec<T::Index, T>,
    #[cfg_attr(feature = "serde", serde(skip))]
    reverse_map: FxHashMap<T, T::Index>,
}

#[cfg(feature = "serde")]
impl<'de, T: IndexedValue + serde::Deserialize<'de>> serde::Deserialize<'de> for IndexedDomain<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct IndexedDomain2<T: IndexedValue> {
            domain: IndexVec<T::Index, T>,
        }
        let domain = IndexedDomain2::<T>::deserialize(deserializer)?;
        let reverse_map = domain
            .domain
            .iter_enumerated()
            .map(|(idx, value)| (value.clone(), idx))
            .collect();
        Ok(IndexedDomain {
            domain: domain.domain,
            reverse_map,
        })
    }
}

impl<T: IndexedValue> IndexedDomain<T> {
    /// Creates an empty domain,
    pub fn new() -> Self {
        IndexedDomain {
            domain: IndexVec::new(),
            reverse_map: FxHashMap::default(),
        }
    }

    /// Gets the object corresponding to `index`.
    ///
    /// Panics if `index` is not within the domain.
    pub fn value(&self, index: T::Index) -> &T {
        &self.domain[index]
    }

    /// Gets the index corresponding to `value`.
    ///
    /// Panics if `value` is not within the domain.
    pub fn index(&self, value: &T) -> T::Index {
        self.reverse_map[value]
    }

    /// Returns true if `value` is contained in the domain.
    pub fn contains_value(&self, value: &T) -> bool {
        self.reverse_map.contains_key(value)
    }

    /// Returns true if `index` is contained in the domain.
    pub fn contains_index(&self, index: T::Index) -> bool {
        index.index() < self.domain.len()
    }

    /// Adds `value` to the domain, returning its new index.
    pub fn insert(&mut self, value: T) -> T::Index {
        self.domain.push(value)
    }

    /// Returns immutable access to the underlying indexed vector.
    pub fn as_vec(&self) -> &IndexVec<T::Index, T> {
        &self.domain
    }

    /// Returns the number of elements in the domain.
    pub fn len(&self) -> usize {
        self.domain.len()
    }

    /// Returns true if the domain is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Similar to [`IndexedDomain::index`], except it adds `value`
    /// to the domain if it does not exist yet.
    pub fn ensure(&mut self, value: &T) -> T::Index {
        if !self.contains_value(value) {
            self.insert(value.clone())
        } else {
            self.index(value)
        }
    }

    /// Returns an iterator over all elements of the domain.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator<Item = &T> {
        self.domain.iter()
    }

    /// Returns an iterator over all indices of the domain.
    pub fn indices(
        &self,
    ) -> impl DoubleEndedIterator<Item = T::Index> + ExactSizeIterator<Item = T::Index> {
        // Re-implementing `indices` because profiling suggests that
        // the IndexVec impl is not getting inlined for some reason??
        (0..self.domain.len()).map(T::Index::from_usize)
    }

    /// Returns an iterator over all pairs of indices and elements of the domain.
    pub fn iter_enumerated(
        &self,
    ) -> impl DoubleEndedIterator<Item = (T::Index, &T)> + ExactSizeIterator<Item = (T::Index, &T)>
    {
        self.domain.iter_enumerated()
    }
}

impl<T: IndexedValue> Default for IndexedDomain<T> {
    fn default() -> Self {
        IndexedDomain::new()
    }
}

impl<T: IndexedValue> From<IndexVec<T::Index, T>> for IndexedDomain<T> {
    /// Creates a new domain from an indexed vector.
    ///
    /// Consider using the [`FromIterator`] implementation if you don't want to manually construct
    /// an [`IndexVec`] object.
    fn from(domain: IndexVec<T::Index, T>) -> Self {
        let reverse_map = domain
            .iter_enumerated()
            .map(|(idx, value)| (value.clone(), idx))
            .collect();
        IndexedDomain {
            domain,
            reverse_map,
        }
    }
}

impl<T: IndexedValue> FromIterator<T> for IndexedDomain<T> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let domain = iter.into_iter().collect::<IndexVec<T::Index, T>>();
        IndexedDomain::from(domain)
    }
}

impl<T: IndexedValue + fmt::Debug> fmt::Debug for IndexedDomain<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.domain.fmt(f)
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
    assert!(d.contains_value(&mk("a")));
    assert!(!d.contains_value(&mk("c")));
    assert_eq!(d.len(), 2);

    assert_eq!(d.iter().collect::<Vec<_>>(), vec!["a", "b"]);
}
