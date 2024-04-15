use cxx::*;

use crate::{util::AsPinMut, Application, Message, Payload};

pub struct Runtime {
    inner: SharedPtr<vsomeip_sys::runtime>
}

unsafe impl AsPinMut for Runtime {
    type Inner = vsomeip_sys::runtime;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}
impl Runtime {
    pub fn get() -> Self {
        Self {
            inner: vsomeip_sys::runtime::get()
        }
    }
    pub fn create_application_with_name(&self, name: impl AsRef<str>) -> Application {
        let name = name.as_ref();
        let_cxx_string!(name_cxx = name);
        let application = vsomeip_sys::runtime::create_application(self.pin_mut(), &name_cxx);

        let app = Application {
            inner: application,
        };

        app.init();

        app
    }
    pub fn create_payload(&self) -> Payload {
        let inner = self.inner.create_payload();

        Payload {
            inner
        }
    }
    pub fn create_message(&self, reliable: bool) -> Message {
        let inner = self.inner.create_message(reliable);

        Message {
            inner
        }
    }
}

#[test]
fn test_application() {
    let runtime = Runtime::get();
    let app = runtime.create_application_with_name("Test");
    app.start();
    std::thread::sleep(std::time::Duration::from_secs(5));
    app.stop();
}