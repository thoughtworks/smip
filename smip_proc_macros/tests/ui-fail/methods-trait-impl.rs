use smip::*;

#[service(id = 0x1234, major_version = 1, minor_version = 0)]
struct Service;

#[methods_impl]
impl ToString for Service {
    #[method(id = 1)]
    fn to_string(&self) -> String {
        "Service"
    }
}


fn main() {}