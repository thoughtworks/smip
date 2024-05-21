use smip::*;

#[service(id = 0x1234, major_version = 1, minor_version = 0)]
struct Service;

#[methods_impl]
impl Service {
    #[smip_method(id = 1)]
    fn method_with_more_than_one_arg(&self, arg1: u32, arg2: String) {}
}


fn main() {}