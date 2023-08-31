use crate::{define_index_type, BitSet, IndexedValue};

define_index_type! {
  pub struct StrIdx for &'static str = u32;
}

#[cfg(all(feature = "bitvec", not(feature = "rustc")))]
pub type TestIndexSet<T> = crate::impls::BitvecIndexSet<T>;
#[cfg(all(feature = "bitvec", not(feature = "rustc")))]
pub type TestIndexMatrix<R, C> = crate::impls::BitvecIndexMatrix<R, C>;

#[cfg(feature = "rustc")]
pub type TestIndexSet<T> = crate::impls::RustcIndexSet<T>;
#[cfg(feature = "rustc")]
pub type TestIndexMatrix<R, C> = crate::impls::RustcIndexMatrix<R, C>;

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

    assert!(bv.intersect(&bv2));
    assert_eq!(bv.iter().collect::<Vec<_>>(), vec![5]);
}
