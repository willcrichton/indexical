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
// mod matrix;
mod pointer;
mod set;
#[cfg(test)]
mod test_utils;

pub use domain::IndexedDomain;
pub use matrix::IndexMatrix;
pub use pointer::*;
pub use set::{IndexSet, Mut, Owned, Ref};

pub trait BitSet: Clone + PartialEq {
  type Iter<'a>: Iterator<Item = usize>
  where
    Self: 'a;

  fn empty(size: usize) -> Self;

  fn insert(&mut self, index: usize);

  fn contains(&self, index: usize) -> bool;

  fn iter(&self) -> Self::Iter<'_>;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn union(&mut self, other: &Self) -> bool;

  fn intersect(&mut self, other: &Self) -> bool;

  fn invert(&mut self);

  fn subtract(&mut self, other: &Self) -> bool;

  fn superset(&self, other: &Self) -> bool {
    let orig_len = self.len();
    // TODO: can we avoid this clone?
    let mut self_copy = self.clone();
    self_copy.union(other);
    orig_len == self_copy.len()
  }
}

pub struct ValueMarker;
pub struct IndexMarker;

pub trait ToIndex<T: IndexedValue, M> {
  fn to_index(&self, domain: &IndexedDomain<T>) -> T::Index;
}

impl<T: IndexedValue> ToIndex<T, ValueMarker> for T {
  fn to_index(&self, domain: &IndexedDomain<T>) -> T::Index {
    domain.index(self)
  }
}

impl<T: IndexedValue> ToIndex<T, IndexMarker> for T::Index {
  fn to_index(&self, _domain: &IndexedDomain<T>) -> T::Index {
    *self
  }
}

pub trait IndexedValue: Clone + PartialEq + Eq + Hash {
  type Index: Idx;
}
