//! Map-like collections for indexed keys.

use std::{
    collections::hash_map,
    ops::{Index, IndexMut},
};

use fxhash::FxHashMap;
use index_vec::{Idx, IndexVec};

use crate::{
    pointer::{ArcFamily, PointerFamily, RcFamily, RefFamily},
    IndexedDomain, IndexedValue, ToIndex,
};

/// A mapping from indexed keys to values, implemented sparsely with a hash map.
///
/// This is more memory-efficient than the [`DenseIndexMap`] with a small
/// number of keys.
pub struct SparseIndexMap<'a, K: IndexedValue + 'a, V, P: PointerFamily<'a>> {
    map: FxHashMap<K::Index, V>,
    domain: P::Pointer<IndexedDomain<K>>,
}

/// [`SparseIndexMap`] specialized to the [`RcFamily`].
pub type SparseRcIndexMap<'a, K, V> = SparseIndexMap<'a, K, V, RcFamily>;

/// [`SparseIndexMap`] specialized to the [`ArcFamily`].
pub type SparseArcIndexMap<'a, K, V> = SparseIndexMap<'a, K, V, ArcFamily>;

/// [`SparseIndexMap`] specialized to the [`RefFamily`].
pub type SparseRefIndexMap<'a, K, V> = SparseIndexMap<'a, K, V, RefFamily<'a>>;

impl<'a, K, V, P> SparseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    /// Constructs an empty map within the given domain.
    pub fn new(domain: &P::Pointer<IndexedDomain<K>>) -> Self {
        SparseIndexMap {
            map: FxHashMap::default(),
            domain: domain.clone(),
        }
    }

    /// Returns an immutable reference to a value for a given key if it exists.
    #[inline]
    pub fn get<M>(&self, key: impl ToIndex<K, M>) -> Option<&V> {
        let idx = key.to_index(&self.domain);
        self.map.get(&idx)
    }

    /// Returns a mutable reference to a value for a given key if it exists.
    #[inline]
    pub fn get_mut<M>(&mut self, key: impl ToIndex<K, M>) -> Option<&mut V> {
        let idx = key.to_index(&self.domain);
        self.map.get_mut(&idx)
    }

    /// Returns a reference to a value for a given key.
    ///
    /// # Safety
    /// This function has undefined behavior if `key` is not in `self`.
    #[inline]
    pub unsafe fn get_unchecked<M>(&self, key: impl ToIndex<K, M>) -> &V {
        let idx = key.to_index(&self.domain);
        self.map.get(&idx).unwrap_unchecked()
    }

    /// Returns a mutable reference to a value for a given key.
    ///
    /// # Safety
    /// This function has undefined behavior if `key` is not in `self`.
    #[inline]
    pub unsafe fn get_unchecked_mut<M>(&mut self, key: impl ToIndex<K, M>) -> &mut V {
        let idx = key.to_index(&self.domain);
        self.map.get_mut(&idx).unwrap_unchecked()
    }

    /// Inserts the key/value pair into `self`.
    #[inline]
    pub fn insert<M>(&mut self, key: impl ToIndex<K, M>, value: V) {
        let idx = key.to_index(&self.domain);
        self.map.insert(idx, value);
    }

    /// Returns an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.map.values()
    }

    /// Returns a mutable entry into the map for the given key.
    #[inline]
    pub fn entry<M>(&mut self, key: impl ToIndex<K, M>) -> hash_map::Entry<'_, K::Index, V> {
        let idx = key.to_index(&self.domain);
        self.map.entry(idx)
    }
}

impl<'a, K, V, P> Index<K::Index> for SparseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    type Output = V;

    fn index(&self, index: K::Index) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, K, V, P> IndexMut<K::Index> for SparseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    fn index_mut(&mut self, index: K::Index) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

/// A mapping from indexed keys to values, implemented densely with a vector.
///
/// This is more time-efficient than the [`SparseIndexMap`] for lookup,
/// but it consumes more memory for missing elements.
pub struct DenseIndexMap<'a, K: IndexedValue + 'a, V, P: PointerFamily<'a>> {
    map: IndexVec<K::Index, V>,
    domain: P::Pointer<IndexedDomain<K>>,
}

/// [`DenseIndexMap`] specialized to the [`RcFamily`].
pub type DenseRcIndexMap<'a, K, V> = DenseIndexMap<'a, K, V, RcFamily>;

/// [`DenseIndexMap`] specialized to the [`ArcFamily`].
pub type DenseArcIndexMap<'a, K, V> = DenseIndexMap<'a, K, V, ArcFamily>;

/// [`DenseIndexMap`] specialized to the [`RefFamily`].
pub type DenseRefIndexMap<'a, K, V> = DenseIndexMap<'a, K, V, RefFamily<'a>>;

impl<'a, K, V, P> DenseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    /// Constructs a new map with an initial element of `mk_elem(i)` for each `i` in `domain`.
    #[inline]
    pub fn new(domain: &P::Pointer<IndexedDomain<K>>, mk_elem: impl FnMut(K::Index) -> V) -> Self {
        DenseIndexMap {
            map: IndexVec::from_iter(domain.indices().map(mk_elem)),
            domain: domain.clone(),
        }
    }

    /// Returns an immutable reference to a value for a given key if it exists.
    #[inline]
    pub fn get<M>(&self, idx: impl ToIndex<K, M>) -> Option<&V> {
        let idx = idx.to_index(&self.domain);
        self.map.get(idx)
    }

    /// Returns a mutable reference to a value for a given key if it exists.
    #[inline]
    pub fn get_mut<M>(&mut self, idx: impl ToIndex<K, M>) -> Option<&mut V> {
        let idx = idx.to_index(&self.domain);
        self.map.get_mut(idx)
    }

    /// Returns a reference to a value for a given key.
    ///
    /// # Safety
    /// This function has undefined behavior if `key` is not in `self`.
    #[inline]
    pub unsafe fn get_unchecked<M>(&self, idx: impl ToIndex<K, M>) -> &V {
        let idx = idx.to_index(&self.domain);
        self.map.raw.get_unchecked(idx.index())
    }

    /// Returns a mutable reference to a value for a given key.
    ///
    /// # Safety
    /// This function has undefined behavior if `key` is not in `self`.
    #[inline]
    pub unsafe fn get_unchecked_mut<M>(&mut self, idx: impl ToIndex<K, M>) -> &mut V {
        let idx = idx.to_index(&self.domain);
        self.map.raw.get_unchecked_mut(idx.index())
    }

    /// Inserts the key/value pair into `self`.
    #[inline]
    pub fn insert<M>(&mut self, idx: impl ToIndex<K, M>, value: V) {
        let idx = idx.to_index(&self.domain);
        self.map[idx] = value;
    }

    /// Returns an iterator over the values of the map.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.map.iter()
    }
}

impl<'a, K, V, P> Index<K::Index> for DenseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    type Output = V;

    #[inline]
    fn index(&self, index: K::Index) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, K, V, P> IndexMut<K::Index> for DenseIndexMap<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    #[inline]
    fn index_mut(&mut self, index: K::Index) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
