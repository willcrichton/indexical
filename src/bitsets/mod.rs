//! Abstraction over bit-set implementations.

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

#[cfg(feature = "bitvec")]
mod bv;
#[cfg(feature = "bitvec")]
pub use bv::*;

#[cfg(feature = "rustc")]
mod rustc;
#[cfg(feature = "rustc")]
pub use rustc::*;

#[cfg(feature = "simd")]
mod simd;
#[cfg(feature = "simd")]
pub use simd::{
    SimdArcIndexMatrix, SimdArcIndexSet, SimdBitset, SimdIndexMatrix, SimdIndexSet,
    SimdRefIndexMatrix, SimdRefIndexSet, SimdSetElement,
};

#[cfg(feature = "roaring")]
mod roar;
#[cfg(feature = "roaring")]
pub use roar::*;
