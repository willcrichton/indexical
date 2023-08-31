use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub trait PointerFamily {
  type Pointer<T>: Deref<Target = T> + Clone;
  fn new<T>(value: T) -> Self::Pointer<T>;
}

pub struct ArcFamily;

impl PointerFamily for ArcFamily {
  type Pointer<T> = Arc<T>;
  fn new<T>(value: T) -> Self::Pointer<T> {
    Arc::new(value)
  }
}

pub struct RcFamily;

impl PointerFamily for RcFamily {
  type Pointer<T> = Rc<T>;
  fn new<T>(value: T) -> Self::Pointer<T> {
    Rc::new(value)
  }
}
