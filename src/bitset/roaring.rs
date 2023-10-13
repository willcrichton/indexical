//! A bit-set based on [`RoaringBitmap`].
//!
//! If you want roaring's SIMD support, add `roaring-simd` to
//! your indexical feature list.

pub use roaring::{self, RoaringBitmap};

use crate::{
    bitset::BitSet,
    pointer::{ArcFamily, RcFamily, RefFamily},
};

/// Wrapper around a [`RoaringBitmap`] that includes the domain size.
#[derive(PartialEq, Clone)]
pub struct RoaringSet {
    set: RoaringBitmap,
    size: usize,
}

fn to_usize(i: u32) -> usize {
    i as usize
}

impl BitSet for RoaringSet {
    type Iter<'a> = std::iter::Map<roaring::bitmap::Iter<'a>, fn(i: u32) -> usize>;

    fn empty(size: usize) -> Self {
        RoaringSet {
            set: RoaringBitmap::new(),
            size,
        }
    }

    fn insert(&mut self, index: usize) -> bool {
        self.set.insert(index as u32)
    }

    fn contains(&self, index: usize) -> bool {
        self.set.contains(index as u32)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.set.iter().map(to_usize)
    }

    fn len(&self) -> usize {
        self.set.len() as usize
    }

    fn union(&mut self, other: &Self) {
        self.set |= &other.set;
    }

    fn intersect(&mut self, other: &Self) {
        self.set &= &other.set;
    }

    fn subtract(&mut self, other: &Self) {
        self.set -= &other.set;
    }

    fn invert(&mut self) {
        for i in 0..self.size {
            if self.set.contains(i as u32) {
                self.set.remove(i as u32);
            } else {
                self.set.insert(i as u32);
            }
        }
    }

    fn clear(&mut self) {
        self.set.clear();
    }

    fn insert_all(&mut self) {
        self.set.insert_range(0..(self.size as u32));
    }

    fn copy_from(&mut self, other: &Self) {
        self.set.clone_from(&other.set);
    }
}

/// [`IndexSet`](crate::IndexSet) specialized to the [`RoaringSet`] implementation.
pub type IndexSet<T> = crate::IndexSet<'static, T, RoaringSet, RcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`RoaringSet`] implementation with the [`ArcFamily`].
pub type ArcIndexSet<'a, T> = crate::IndexSet<'a, T, RoaringSet, ArcFamily>;

/// [`IndexSet`](crate::IndexSet) specialized to the [`RoaringSet`] implementation with the [`RefFamily`].
pub type RefIndexSet<'a, T> = crate::IndexSet<'a, T, RoaringSet, RefFamily<'a>>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`RoaringSet`] implementation.
pub type IndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, RoaringSet, RcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`RoaringSet`] implementation with the [`ArcFamily`].
pub type ArcIndexMatrix<R, C> = crate::IndexMatrix<'static, R, C, RoaringSet, ArcFamily>;

/// [`IndexMatrix`](crate::IndexMatrix) specialized to the [`RoaringSet`] implementation with the [`RefFamily`].
pub type RefIndexMatrix<'a, R, C> = crate::IndexMatrix<'a, R, C, RoaringSet, RefFamily<'a>>;

#[test]
fn test_roaring() {
    crate::test_utils::impl_test::<RoaringSet>();
}
