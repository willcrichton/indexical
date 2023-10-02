//! Implementations of the [`BitSet`](crate::BitSet) trait for different backends.

#[cfg(feature = "bitvec")]
mod bv;
#[cfg(feature = "bitvec")]
pub use bv::{
    bitvec::{self, vec::BitVec},
    BitvecArcIndexMatrix, BitvecArcIndexSet, BitvecIndexMatrix, BitvecIndexSet,
    BitvecRefIndexMatrix, BitvecRefIndexSet,
};

#[cfg(feature = "rustc")]
mod rustc;
#[cfg(feature = "rustc")]
pub use rustc::{
    rustc_index::bit_set::{self, BitSet},
    RustcArcIndexMatrix, RustcArcIndexSet, RustcIndexMatrix, RustcIndexSet, RustcRefIndexMatrix,
    RustcRefIndexSet,
};
