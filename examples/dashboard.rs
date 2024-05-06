use rand::Rng;
use smip::{methods_impl, service};
use smip_core::Runtime;

#[service(id = 0x1111)]
struct Dashboard {}

impl Dashboard {
    fn new() -> Self {
        Self {}
    }
}

#[methods_impl]
impl Dashboard {
    #[smip_method(id = 0x3333)]
    fn speed(&self, request: i32) -> i32 {
        dbg!(request);
        let mut rng = rand::thread_rng();
        rng.gen_range(10..151)
    }
}

fn main() {
    let config = smip::RuntimeConfig::new("Dashboard", 0x1313, 0x2222);

    let application = Runtime::new(config).service(Dashboard::new(), 30509);

    let _ = application.run();
}
