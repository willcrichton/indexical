//! Implementations of the [`BitSet`](crate::BitSet) trait for different backends.

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
