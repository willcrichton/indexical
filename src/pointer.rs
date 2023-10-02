use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

/// Abstraction over smart pointers with `'static` interiors.
///
/// Used so to make the indexical data structures generic with respect
/// to choice of `Rc` or `Arc` (or your own clonable smart pointer!).
pub trait PointerFamily {
    /// Pointer type for a given family.
    type Pointer<T: 'static>: Deref<Target = T> + Clone;
}

/// Family of [`Arc`] pointers.
pub struct ArcFamily;

impl PointerFamily for ArcFamily {
    type Pointer<T: 'static> = Arc<T>;
}

/// Family of [`Rc`] pointers.
pub struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T: 'static> = Rc<T>;
}

/// Family of `&`-references.
pub struct RefFamily<'a>(PhantomData<&'a ()>);

impl<'a> PointerFamily for RefFamily<'a> {
    type Pointer<T: 'static> = &'a T;
}
