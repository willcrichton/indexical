#![doc = include_str!("../README.md")]
//! ## Design
//! The key idea is that the [`IndexedDomain`] is shared pervasively
//! across all Indexical types. All types can then use the [`IndexedDomain`] to convert between indexes and objects, usually via the [`ToIndex`] trait.
//!
//! [`IndexSet`] and [`IndexMatrix`] are generic with respect to two things:
//! 1. **The choice of bit-set implementation.** By default, Indexical includes the [`bitvec`] crate and provides the [`bitset::bitvec::IndexSet`] type.
//!    You can provide your own bit-set implementation via the [`bitset::BitSet`] trait.
//! 2. **The choice of domain pointer.** By default, Indexical uses the [`Rc`](std::rc::Rc) pointer via the [`RcFamily`](pointer::RcFamily) type.
//!    You can choose to use the [`ArcFamily`](pointer::ArcFamily) if you need concurrency, or the [`RefFamily`](pointer::RefFamily) if you want to avoid reference-counting.

#![cfg_attr(feature = "rustc", feature(rustc_private))]
#![cfg_attr(feature = "simd", feature(portable_simd, unchecked_shifts))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

use self::pointer::PointerFamily;
use index_vec::Idx;
use std::hash::Hash;

pub mod bitset;
mod domain;
pub mod map;
mod matrix;
pub mod pointer;
mod set;
pub mod vec;
#[cfg(test)]
mod test_utils;

#[doc(hidden)]
pub use index_vec as _index_vec;

pub use domain::IndexedDomain;
pub use matrix::IndexMatrix;
pub use set::IndexSet;

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

impl<T: IndexedValue> ToIndex<T, MarkerRef> for &T {
    #[inline]
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index {
        domain.index(self)
    }
}

impl<T: IndexedValue> ToIndex<T, MarkerIndex> for T::Index {
    #[inline]
    fn to_index(self, _domain: &IndexedDomain<T>) -> T::Index {
        self
    }
}

/// Links a type to its index.
///
/// Should be automatically implemented by the [`define_index_type`] macro.
pub trait IndexedValue: Clone + PartialEq + Eq + Hash {
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
    $v:vis struct $type:ident for $target:ident $(<$($l:lifetime),*>)? = $raw:ident;
    $($CONFIG_NAME:ident = $value:expr;)* $(;)?
  ) => {
    $crate::_index_vec::define_index_type! {
      $(#[$attrs])*
      $v struct $type = $raw;
      $($CONFIG_NAME = $value;)*
    }

    impl $(<$($l),*>)? $crate::IndexedValue for $target $(<$($l),*>)? {
      type Index = $type;
    }
  }
}

/// Generic interface for converting iterators into indexical collections.
pub trait FromIndexicalIterator<'a, T: IndexedValue + 'a, P: PointerFamily<'a>, M, A>:
    Sized
{
    /// Converts an iterator into a collection within the given domain.
    fn from_indexical_iter(
        iter: impl Iterator<Item = A>,
        domain: &P::Pointer<IndexedDomain<T>>,
    ) -> Self;
}

/// Extension trait that adds `collect_indexical` to all iterators.
pub trait IndexicalIteratorExt<'a, T: IndexedValue + 'a, P: PointerFamily<'a>, M>:
    Iterator + Sized
{
    /// Like [`Iterator::collect`], except also takes as input a `domain`.
    fn collect_indexical<B>(self, domain: &P::Pointer<IndexedDomain<T>>) -> B
    where
        B: FromIndexicalIterator<'a, T, P, M, Self::Item>,
    {
        FromIndexicalIterator::from_indexical_iter(self, domain)
    }
}

impl<'a, I: Iterator, T: IndexedValue + 'a, P: PointerFamily<'a>, M>
    IndexicalIteratorExt<'a, T, P, M> for I
{
}
