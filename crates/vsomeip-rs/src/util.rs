use cxx::*;
use cxx::private::SharedPtrTarget;
use std::pin::Pin;

/// FIXME: The conversion done here is highly unsafe and UB. 
/// This is the only simple method I could think of to get a pinned mutable reference from a `SharedPtr`.
/// See for more info: https://github.com/dtolnay/cxx/issues/537
#[allow(clippy::needless_lifetimes)]
pub(crate) unsafe fn shared_to_pin<'a, T: SharedPtrTarget>(shared: &'a SharedPtr<T>) -> Pin<&'a mut T> {
    let shared_ref = shared.as_ref().unwrap();
    
    #[allow(invalid_reference_casting)]
    let pin = unsafe { Pin::new_unchecked(&mut *(shared_ref as *const T as *mut T)) };

    pin
}

/// Trait to convert a shared pointer to a pinned mutable reference.
/// # Safety
/// 
/// It is upto the implementor to ensure that the conversion is safe for the type and the aliasing rules are not violated.
pub(crate) unsafe trait AsPinMut {
    type Inner: SharedPtrTarget;
    fn inner(&self) -> &SharedPtr<Self::Inner>;
    fn pin_mut(&self) -> Pin<&mut Self::Inner> {
        unsafe { shared_to_pin(self.inner()) }
    }
}
