//! Implementations of the [`BitSet`](crate::BitSet) trait for different backends.

#[cfg(feature = "bitvec")]
mod bv;
#[cfg(feature = "bitvec")]
pub use bv::{BitvecArcIndexMatrix, BitvecIndexMatrix, BitvecIndexSet};

#[cfg(feature = "rustc")]
mod rustc;
#[cfg(feature = "rustc")]
pub use rustc::{RustcIndexMatrix, RustcIndexSet};
