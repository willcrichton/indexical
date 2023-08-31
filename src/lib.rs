//! Indexical is a library for efficiently working with indexed collections of objects.
//! "Indexed" means that the domain of objects is finite, and you can assign a numeric index to each object.
//! This enables the use of efficient data structures like bit-sets.
//!
//! Indexical is a layer on top of existing bit-set libraries like [bitvec] and [rustc_index].
//! Those data structures only "understand" indexes, not the objects represented by the index.
//! Indexical provides utilities for converting between the object domain and the index domain.
//!
//! # Example
//! ```
//! use indexical::{
//!     IndexedDomain, IndexedValue,
//!     define_index_type, impls::BitvecIndexSet
//! };
//! use std::rc::Rc;
//!
//! // First, define a custom type.
//! #[derive(PartialEq, Eq, Clone, Hash)]
//! pub struct MyString(String);
//!
//! // Second, define a new index for your custom type.
//! define_index_type! {
//!     pub struct StringIndex for MyString = u32;
//! }
//!
//!
//! // Third, create an indexed domain from a collection of objects.
//! let domain = Rc::new(IndexedDomain::from_iter([
//!     MyString(String::from("Hello")), MyString(String::from("world"))
//! ]));
//!
//! // Finally, you can make a set! Notice how you can pass either a `MyString`
//! // or a `StringIndex` to `set.insert(..)` and `set.contains(..)`.
//! let mut set = BitvecIndexSet::new(&domain);
//! set.insert(MyString(String::from("Hello")));
//! set.insert(StringIndex::from_usize(1));
//! assert!(set.contains(MyString(String::from("world"))));
//! ```
//!
//! # Design
//! The key idea is that the [`IndexedDomain`] is wrapped in a reference-counted pointer like [`Rc`](std::rc::Rc) and shared pervasively
//! across all Indexical types. All types can then use the [`IndexedDomain`] to convert between indexes and objects.
//!
//! [`IndexSet`] and [`IndexMatrix`] are generic with respect to two things:
//! 1. **The choice of bit-set implementation.** By default, Indexical includes the [`bitvec`] crate and provides the [`impls::BitvecIndexSet`] type.
//!    You can provide your own bit-set implementation via the [`BitSet`] trait.
//! 2. **The choice of reference-counted pointer.** By default, Indexical uses the [`Rc`](std::rc::Rc) pointer via the [`RcFamily`] type.
//!    You can choose to use the [`ArcFamily`] if you need concurrency. You can also implement your own pointer family.
//!
//! [bitvec]: https://github.com/ferrilab/bitvec
//! [rustc_index]: (https://doc.rust-lang.org/nightly/nightly-rustc/rustc_index

#![cfg_attr(feature = "rustc", feature(rustc_private))]

#[cfg(feature = "rustc")]
pub mod rustc {
    extern crate rustc_driver;
    pub extern crate rustc_index as index;
    pub extern crate rustc_mir_dataflow as mir_dataflow;
}

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
pub use set::IndexSet;

/// Interface for bit-set implementations.
///
/// Implement this trait if you want to provide a custom bit-set
/// beneath the indexical abstractions.
pub trait BitSet: Clone + PartialEq {
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

    /// Adds all ones from `other` to `self`, returning true if `self` changed.
    fn union(&mut self, other: &Self) -> bool;

    /// Removes all ones in `self` not in `other`, returning true if `self` changed.
    fn intersect(&mut self, other: &Self) -> bool;

    /// Removes all ones from `other` in `self`, returning true if `self` changed.
    fn subtract(&mut self, other: &Self) -> bool;

    /// Flips all bits in `self`.
    fn invert(&mut self);

    /// Sets all bits to 0.
    fn clear(&mut self);

    /// Returns true if all ones in `other` are a one in `self`.
    fn superset(&self, other: &Self) -> bool {
        let orig_len = self.len();
        // TODO: can we avoid this clone?
        let mut self_copy = self.clone();
        self_copy.union(other);
        orig_len == self_copy.len()
    }
}

/// Coherence hack for the `ToIndex` trait.
pub struct OwnedMarker;
/// Coherence hack for the `ToIndex` trait.
pub struct RefMarker;
/// Coherence hack for the `ToIndex` trait.
pub struct IndexMarker;

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

impl<T: IndexedValue> ToIndex<T, OwnedMarker> for T {
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index {
        domain.index(&self)
    }
}

impl<'a, T: IndexedValue> ToIndex<T, RefMarker> for &'a T {
    fn to_index(self, domain: &IndexedDomain<T>) -> T::Index {
        domain.index(self)
    }
}

impl<T: IndexedValue> ToIndex<T, IndexMarker> for T::Index {
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
