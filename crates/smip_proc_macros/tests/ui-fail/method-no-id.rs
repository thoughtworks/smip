use smip::*;

#[service(id = 0x1234, major_version = 1, minor_version = 0)]
struct Service;

#[methods_impl]
impl Service {
    #[smip_method]
    fn foo(&self) {}
}


fn main() {}