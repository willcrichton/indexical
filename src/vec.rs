//! Vector-like collection for indexed keys.

use std::{
    ops::{Index, IndexMut},
    slice::GetDisjointMutError,
};

use index_vec::Idx;

use crate::{
    IndexedDomain, IndexedValue, ToIndex,
    pointer::{ArcFamily, PointerFamily, RcFamily, RefFamily},
};

/// A fixed-sized vector with one value for each key in the domain.
pub struct IndexVec<'a, K: IndexedValue + 'a, V, P: PointerFamily<'a>> {
    vec: Vec<V>,
    pub(crate) domain: P::Pointer<IndexedDomain<K>>,
}

impl<'a, K, V, P> IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
    V: Clone,
{
    /// Constructs a new vector where each index maps to the initial element `elem`.
    pub fn from_elem(elem: V, domain: &P::Pointer<IndexedDomain<K>>) -> Self {
        let vec = vec![elem; domain.len()];
        IndexVec {
            vec,
            domain: domain.clone(),
        }
    }
}

impl<'a, K, V, P> IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    /// Constructs a new vector where each index maps to the output of `f(index)`.
    pub fn from_fn(f: impl FnMut(K::Index) -> V, domain: &P::Pointer<IndexedDomain<K>>) -> Self {
        let vec = domain.indices().map(f).collect();
        IndexVec {
            vec,
            domain: domain.clone(),
        }
    }

    /// Returns an immutable reference to a value for a given index.
    pub fn get<M>(&self, idx: impl ToIndex<K, M>) -> &V {
        let idx = idx.to_index(&self.domain);
        debug_assert!(self.domain.contains_index(idx));
        unsafe { self.vec.get_unchecked(idx.index()) }
    }

    /// Returns a mutable reference to a value for a given index.
    pub fn get_mut<M>(&mut self, idx: impl ToIndex<K, M>) -> &mut V {
        let idx = idx.to_index(&self.domain);
        debug_assert!(self.domain.contains_index(idx));
        unsafe { self.vec.get_unchecked_mut(idx.index()) }
    }

    /// Returns an iterator over immutable references to the values.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator<Item = &V> {
        self.vec.iter()
    }

    /// Returns an iterator over mutable references to the values.
    pub fn iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = &mut V> + ExactSizeIterator<Item = &mut V> {
        self.vec.iter_mut()
    }

    /// Returns the underlying vector as a slice.
    pub fn as_slice(&self) -> &[V] {
        &self.vec
    }

    /// Returns the underlying vector as a mutable slice.
    pub fn as_slice_mut(&mut self) -> &mut [V] {
        &mut self.vec
    }

    /// Returns multiple mutable references to disjoint indices,
    /// or a [`GetDisjointMutError`] if not disjoint or in-bounds.
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [K::Index; N],
    ) -> Result<[&mut V; N], GetDisjointMutError> {
        self.vec.get_disjoint_mut(indices.map(Idx::index))
    }
}

impl<'a, K, V, P> Clone for IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
    V: Clone,
{
    fn clone(&self) -> Self {
        IndexVec {
            vec: self.vec.clone(),
            domain: self.domain.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vec.clone_from(&source.vec);
    }
}

impl<'a, K, V, P> PartialEq for IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<'a, K, V, P> Eq for IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
    V: Eq,
{
}

impl<'a, K, V, P> Index<K::Index> for IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    type Output = V;

    fn index(&self, index: K::Index) -> &Self::Output {
        self.get(index)
    }
}

impl<'a, K, V, P> IndexMut<K::Index> for IndexVec<'a, K, V, P>
where
    K: IndexedValue + 'a,
    P: PointerFamily<'a>,
{
    fn index_mut(&mut self, index: K::Index) -> &mut Self::Output {
        self.get_mut(index)
    }
}

/// [`IndexVec`] specialized to the [`RcFamily`].
pub type RcIndexVec<K, V> = IndexVec<'static, K, V, RcFamily>;

/// [`IndexVec`] specialized to the [`ArcFamily`].
pub type ArcIndexVec<K, V> = IndexVec<'static, K, V, ArcFamily>;

/// [`IndexVec`] specialized to the [`RefFamily`].
pub type RefIndexVec<'a, K, V> = IndexVec<'a, K, V, RefFamily<'a>>;
