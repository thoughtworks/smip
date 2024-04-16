use cxx::SharedPtr;

use crate::{util::AsPinMut, Runtime};

pub struct Payload {
    pub(crate) inner: SharedPtr<vsomeip_sys::payload>
}

unsafe impl AsPinMut for Payload {
    type Inner = vsomeip_sys::payload;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Payload {
    pub fn new() -> Self {
        Runtime::get().create_payload()
    }
    pub fn with_data(data: &[u8]) -> Self {
        let mut payload = Self::new();
        payload.set_data(data);

        payload
    }
    pub fn set_data(&mut self, data: &[u8]) {
        let ptr = data.as_ptr();
        let length = data.len() as u32;
        unsafe { vsomeip_sys::payload::set_data(self.pin_mut(), ptr, length) }
    }
    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        let data = unsafe { vsomeip_sys::payload::get_data(self.pin_mut()) };
        let len = unsafe { vsomeip_sys::payload::get_length(&self.inner) as usize };

        unsafe { std::slice::from_raw_parts::<'a, u8>(data, len) }
    }
}