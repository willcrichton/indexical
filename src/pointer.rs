use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

/// Abstraction over smart pointers.
///
/// Used so to make the indexical data structures generic with respect
/// to choice of `Rc` or `Arc` (or your own clonable smart pointer!).
pub trait PointerFamily {
    /// Pointer type for a given family.
    type Pointer<T>: Deref<Target = T> + Clone;

    /// Create a new instance of the family's pointer type.
    fn new<T>(value: T) -> Self::Pointer<T>;
}

/// Family of [`Arc`] pointers.
pub struct ArcFamily;

impl PointerFamily for ArcFamily {
    type Pointer<T> = Arc<T>;

    fn new<T>(value: T) -> Self::Pointer<T> {
        Arc::new(value)
    }
}

/// Family of [`Rc`] pointers.
pub struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;

    fn new<T>(value: T) -> Self::Pointer<T> {
        Rc::new(value)
    }
}
