use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

/// Abstraction over smart pointers with `'static` interiors.
///
/// Used so to make the indexical data structures generic with respect
/// to choice of `Rc` or `Arc` (or your own clonable smart pointer!).
pub trait PointerFamily<'a> {
    /// Pointer type for a given family.
    type Pointer<T: 'a>: Deref<Target = T> + Clone;
}

/// Family of [`Arc`] pointers.
pub struct ArcFamily;

impl<'a> PointerFamily<'a> for ArcFamily {
    type Pointer<T: 'a> = Arc<T>;
}

/// Family of [`Rc`] pointers.
pub struct RcFamily;

impl<'a> PointerFamily<'a> for RcFamily {
    type Pointer<T: 'a> = Rc<T>;
}

/// Family of `&`-references.
pub struct RefFamily<'a>(PhantomData<&'a ()>);

impl<'a> PointerFamily<'a> for RefFamily<'a> {
    type Pointer<T: 'a> = &'a T;
}
