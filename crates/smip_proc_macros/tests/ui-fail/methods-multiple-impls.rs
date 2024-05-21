use smip::*;

#[service(id = 0x1234, major_version = 1, minor_version = 0)]
struct Service;

impl Service {
    fn normal_method(&self) {}
}
#[methods_impl]
impl Service {
    #[smip_method(id = 1)]
    fn method_1(&self) {}
}

#[methods_impl]
impl Service {
    #[smip_method(id = 2)]
    fn method_2(&mut self) {}
}

fn main() {}