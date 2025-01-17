use smip::Runtime;

#[smip::service(id = 0x1234, major_version = 1, minor_version = 0)]
struct MyService {
    x: u32,
}

#[smip::methods_impl]
impl MyService {
    // Method that adds whatever value it receives to x
    // Expects u32 in request
    // Returns u32 in response
    #[smip_method(id = 1)]
    fn add(&mut self, value: u32) -> u32 {
        self.x += value;

        self.x
    }

    // Method that sends a string
    // Expects nothing in request
    // Returns String in response
    #[smip_method(id = 2)]
    fn hello(&self) -> String {
        "Hello World".into()
    }
}
fn main() {
    let config = smip::RuntimeConfig::new("Simple", 0xABCD, 0x1);

    let application = Runtime::new(config).service(
        MyService {
        x: 0
    }, 30509);

    let _ = application.run();
}
