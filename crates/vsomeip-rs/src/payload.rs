use cxx::SharedPtr;

use crate::{util::AsPinMut, Runtime};

/// This class implements an array of bytes to be used as
/// payload for SOME/IP messages.
pub struct Payload {
    pub(crate) inner: SharedPtr<vsomeip_sys::payload>
}

unsafe impl Send for Payload {}
unsafe impl Sync for Payload {}

unsafe impl AsPinMut for Payload {
    type Inner = vsomeip_sys::payload;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Payload {
    /// Creates a new payload object.
    /// Shorthand for [`Runtime::get().create_payload()`](struct.Runtime.html#method.create_payload)
    pub fn new() -> Self {
        Runtime::get().create_payload()
    }
    /// Creates a new payload object with the given data.
    pub fn with_data(data: &[u8]) -> Self {
        let mut payload = Self::new();
        payload.set_data(data);

        payload
    }
    /// Copies the given data array to the payload object.
    ///
    /// The current payload content is replaced by the data provided.
    /// The given buffer remains untouched.
    ///
    /// # Parameters
    /// - `data`: slice containing the data.
    pub fn set_data(&mut self, data: &[u8]) {
        let ptr = data.as_ptr();
        let length = data.len() as u32;
        unsafe { vsomeip_sys::payload::set_data(self.pin_mut(), ptr, length) }
    }

    /// Returns the data of the payload as a slice.
    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        let data = unsafe { vsomeip_sys::payload::get_data(self.pin_mut()) };
        let len = unsafe { vsomeip_sys::payload::get_length(&self.inner) as usize };

        unsafe { std::slice::from_raw_parts::<'a, u8>(data, len) }
    }
}