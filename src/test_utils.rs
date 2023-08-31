use crate::{BitSet, IndexedValue};

index_vec::define_index_type! {
  pub struct StrIdx = u32;
}

impl IndexedValue for &'static str {
  type Index = StrIdx;
}

#[cfg(feature = "bitvec")]
pub type TestIndexSet<T> = crate::impls::BitvecIndexSet<T>;

#[cfg(feature = "rustc")]
pub type TestIndexSet<T> = crate::impls::RustcIndexSet<T>;

pub fn impl_test<T: BitSet>() {
  let mut bv = T::empty(10);
  assert!(!bv.contains(0));

  bv.insert(0);
  bv.insert(5);
  assert!(bv.contains(0));
  assert!(bv.contains(5));
  assert!(!bv.contains(1));
  assert_eq!(bv.iter().collect::<Vec<_>>(), vec![0, 5]);
  assert_eq!(bv.len(), 2);

  let mut bv2 = T::empty(10);
  bv2.insert(5);
  assert!(bv.superset(&bv2));
  bv2.insert(1);
  assert!(!bv.superset(&bv2));

  bv.intersect(&bv2);
  assert_eq!(bv.iter().collect::<Vec<_>>(), vec![5]);
}
