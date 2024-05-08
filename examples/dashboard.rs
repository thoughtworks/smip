use chrono::TimeZone;
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

    #[smip_method(id = 0x4444)]
    fn time(&self, request: ()) -> String {
        // Get the current time in UTC
        let utc_time = chrono::Utc::now();

    // Convert UTC time to Indian Standard Time (IST)
    let time = utc_time.with_timezone(&chrono::FixedOffset::east_opt(5*3600 + 30*60).unwrap());

    format!("{:?}", time.format("%H:%M:%S").to_string())
    }
}

fn main() {
    let config = smip::RuntimeConfig::new("Dashboard", 0x1313, 0x2222);

    let application = Runtime::new(config).service(Dashboard::new(), 30509);

    let _ = application.run();
}
