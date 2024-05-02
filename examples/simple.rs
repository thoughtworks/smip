#[smip::service(id = 0x1234, major_version = 1, minor_version = 0)]
struct MyService {
    x: u32
}

#[smip::methods_impl]
impl MyService {
    #[smip_method(id = 1)]
    fn add(&mut self, value: u32) {
        self.x += value;
        println!("foo {}", self.x);
    }
    #[smip_method(id = 2)]
    fn bar(&self, y: bool) -> String {
        format!("bar {}", y)
    }
}
pub fn main() {
}