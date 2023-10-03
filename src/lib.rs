#![doc = include_str!("../README.md")]
//! ## Design
//! The key idea is that the [`IndexedDomain`] is shared pervasively
//! across all Indexical types. All types can then use the [`IndexedDomain`] to convert between indexes and objects, usually via the [`ToIndex`] trait.
//!
//! [`IndexSet`] and [`IndexMatrix`] are generic with respect to two things:
//! 1. **The choice of bit-set implementation.** By default, Indexical includes the [`bitvec`] crate and provides the [`impls::BitvecIndexSet`] type.
//!    You can provide your own bit-set implementation via the [`BitSet`] trait.
//! 2. **The choice of domain pointer.** By default, Indexical uses the [`Rc`](std::rc::Rc) pointer via the [`RcFamily`] type.
//!    You can choose to use the [`ArcFamily`] if you need concurrency, or the [`RefFamily`] if you want to avoid reference-counting.

#![cfg_attr(feature = "rustc", feature(rustc_private))]
#![cfg_attr(feature = "simd", feature(portable_simd, unchecked_math))]
#![warn(missing_docs)]

use index_vec::Idx;
use std::hash::Hash;

mod domain;
pub mod impls;
mod matrix;
mod pointer;
mod set;
#[cfg(test)]
mod test_utils;

pub use index_vec;

pub use domain::IndexedDomain;
pub use matrix::IndexMatrix;
pub use pointer::*;
pub use set::{IndexSet, IndexSetIteratorExt};

/// Interface for bit-set implementations.
///
/// Implement this trait if you want to provide a custom bit-set
/// beneath the indexical abstractions.
pub trait BitSet: Clone + PartialEq {
    /// Type of iterator returned by `iter`.
    type Iter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Constructs a new bit-set with a domain of size `size`.
    fn empty(size: usize) -> Self;

    /// Sets `index` to 1, returning true if `self` changed.
    fn insert(&mut self, index: usize) -> bool;

    /// Returns true if `index` is 1.
    fn contains(&self, index: usize) -> bool;

    /// Returns an iterator over all the indices of ones in the bit-set.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns the number of ones in the bit-set.
    fn len(&self) -> usize;

    /// Returns true if there are no ones in the bit-set.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Note: we have the `_changed` methods separated out because
    // if you don't care about the return value, then it's just extra
    // computation w/ some APIs like bitvec.

    /// Adds all ones from `other` to `self`.
    fn union(&mut self, other: &Self);

    /// Adds all ones from `other` to `self`, returning true if `self` changed.
    fn union_changed(&mut self, other: &Self) -> bool {
        let n = self.len();
        self.union(other);
        n != self.len()
    }

    /// Removes all ones in `self` not in `other`.
    fn intersect(&mut self, other: &Self);

    /// Removes all ones in `self` not in `other`, returning true if `self` changed.
    fn intersect_changed(&mut self, other: &Self) -> bool {
        let n = self.len();
        self.intersect(other);
        n != self.len()
    }

    /// Removes all ones from `other` in `self`.
    fn subtract(&mut self, other: &Self);

    /// Removes all ones from `other` in `self`, returning true if `self` changed.
    fn subtract_changed(&mut self, other: &Self) -> bool {
        let n = self.len();
        self.intersect(other);
        n != self.len()
    }

    /// Flips all bits in `self`.
    fn invert(&mut self);

    /// Sets all bits to 0.
    fn clear(&mut self);

    /// Adds every element of the domain to `self`.
    fn insert_all(&mut self);

    /// Returns true if all ones in `other` are a one in `self`.
    fn superset(&self, other: &Self) -> bool {
        let orig_len = self.len();
        // TODO: can we avoid this clone?
        let mut self_copy = self.clone();
        self_copy.union(other);
        orig_len == self_copy.len()
    }

    /// Copies `other` into `self`. Must have the same lengths.
    fn copy_from(&mut self, other: &Self);
}

/// Coherence hack for the `ToIndex` trait.
pub struct MarkerOwned;
/// Coherence hack for the `ToIndex` trait.
pub struct MarkerRef;
/// Coherence hack for the `ToIndex` trait.
pub struct MarkerIndex;

/// Implicit conversions from elements to indexes.
/// Commonly used in the [`IndexSet`] and [`IndexMatrix`] interfaces.
///
/// Note that we cannot use the [`Into`] trait because this conversion requires
/// the [`IndexedDomain`] as input.
///
/// The `M` type parameter is a coherence hack to ensure the two blanket implementations
/// do not conflict.
pub trait ToIndex<T: IndexedValue, M> {
    /// Converts `self` to an index over `T`.
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index;
}

impl<T: IndexedValue> ToIndex<T, MarkerOwned> for T {
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index {
        domain.index(&self)
    }
}

impl<'a, T: IndexedValue> ToIndex<T, MarkerRef> for &'a T {
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index {
        domain.index(self)
    }
}

impl<T: IndexedValue> ToIndex<T, MarkerIndex> for T::Index {
    fn to_index(self, _domain: &IndexedDomain<T>) -> T::Index {
        self
    }
}

/// Links a type to its index.
///
/// Should be automatically implemented by the [`define_index_type`] macro.
pub trait IndexedValue: Clone + PartialEq + Eq + Hash + 'static {
    /// The index for `Self`.
    type Index: Idx;
}

/// Creates a new index type and associates it with an object type.
///
/// This is a thin wrapper around [`index_vec::define_index_type`]. The only
/// modification is the `for $TYPE` syntax that generates the [`IndexedValue`]
/// implementation.
#[macro_export]
macro_rules! define_index_type {
  (
    $(#[$attrs:meta])*
    $v:vis struct $type:ident for $target:ty = $raw:ident;
    $($CONFIG_NAME:ident = $value:expr;)* $(;)?
  ) => {
    $crate::index_vec::define_index_type! {
      $(#[$attrs])*
      $v struct $type = $raw;
      $($CONFIG_NAME = $value;)*
    }

    impl $crate::IndexedValue for $target {
      type Index = $type;
    }
  }
}
