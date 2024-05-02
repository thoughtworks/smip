#[smip::service(id = 0x1234, major_version = 1, minor_version = 0)]
struct MyService {
    x: u32
}

#[smip::methods_impl]
impl MyService {
    #[smip_method(id = 1)]
    fn foo(&self, x: u32) {
        println!("foo {}", x);
    }
    #[smip_method(id = 2)]
    fn bar(&self, y: bool) -> String {
        format!("bar {}", y)
    }
}
pub fn main() {
}