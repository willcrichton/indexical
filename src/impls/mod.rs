//! Implementations of the [`BitSet`](crate::BitSet) trait for different backends.

#[cfg(feature = "bitvec")]
mod bv;
#[cfg(feature = "bitvec")]
pub use bitvec::{self, vec::BitVec};
#[cfg(feature = "bitvec")]
pub use bv::{BitvecArcIndexMatrix, BitvecArcIndexSet, BitvecIndexMatrix, BitvecIndexSet};

#[cfg(feature = "rustc")]
mod rustc;
#[cfg(feature = "rustc")]
pub use rustc::{RustcArcIndexMatrix, RustcArcIndexSet, RustcIndexMatrix, RustcIndexSet};
#[cfg(feature = "rustc")]
pub use rustc_index::bit_set::{self, BitSet};
