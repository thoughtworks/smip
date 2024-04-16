use cxx::*;
use cxx::private::SharedPtrTarget;
use std::pin::Pin;

unsafe fn shared_to_pin<T: SharedPtrTarget>(shared: &SharedPtr<T>) -> Pin<&mut T> {
    let shared_ref = shared.as_ref().unwrap();
    #[allow(clippy::cast_ref_to_mut)]
    #[allow(invalid_reference_casting)]
    let pin = unsafe { Pin::new_unchecked(&mut *(shared_ref as *const _ as *mut _)) };

    pin
}

pub(crate) unsafe trait AsPinMut {
    type Inner: SharedPtrTarget;
    fn inner(&self) -> &SharedPtr<Self::Inner>;
    fn pin_mut(&self) -> Pin<&mut Self::Inner> {
        unsafe { shared_to_pin(self.inner()) }
    }
}
