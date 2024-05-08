use std::time::Instant;

use smip::{methods_impl, service};
use smip_core::Runtime;

use noise::*;
struct SpeedSensor {
    perlin: noise::Perlin,
    range: std::ops::Range<u32>,
    t: f64
}

impl SpeedSensor {
    fn new(range: std::ops::Range<u32>) -> Self {
        Self {
            perlin: noise::Perlin::new(0),
            range,
            t: 0.0
        }
    }
    fn value(&mut self) -> u32 {
        let x = self.perlin.get([self.t; 1]);
        let speed = self.range.start as f64 + self.range.len() as f64 * x;
        speed.round() as u32
    }
}

struct BatterySensor {
    charge: u8,
    last_update: Instant,
}
impl BatterySensor {
    fn new(charge: u8) -> Self {
        Self {
            charge,
            last_update: Instant::now(),
        }
    }
    fn value(&mut self) -> u8 {
        if Instant::now() - self.last_update > std::time::Duration::from_secs(1) {
            self.charge -= 1;
            self.last_update = Instant::now();
        }

        self.charge
    }
}

#[derive(Debug)]
pub enum Indicator {
    Off,
    Left,
    Right,
    Hazard,
}

#[service(id = 0x1111)]
struct Dashboard {
    speed_sensor: SpeedSensor,
    battery: BatterySensor,
    indicator: Indicator,
}

#[methods_impl]
impl Dashboard {
    fn new() -> Self {
        Self {
            speed_sensor: SpeedSensor::new(10..150),
            battery: BatterySensor::new(100),
            indicator: Indicator::Off
        }
    }
    #[smip_method(id = 0x3333)]
    fn speed(&mut self, request: ()) -> u32 {
        self.speed_sensor.value()
    }
    #[smip_method(id = 0x4444)]
    fn battery(&mut self, request: ()) -> u8 {
        self.battery.value()
    }
    #[smip_method(id = 0x5555)]
    fn indicator(&self, request: ()) -> String {
        format!("{:?}", self.indicator)
    }
}

fn main() {
    let config = smip::RuntimeConfig::new("Dashboard", 0x1313, 0x2222);

    let application = Runtime::new(config).service(Dashboard::new(), 30509);

    let _ = application.run();
}
