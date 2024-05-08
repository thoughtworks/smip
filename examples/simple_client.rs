use std::net::{IpAddr, Ipv4Addr};
use std::error::Error;

use smip::Client;

fn main() -> Result<(), Box<dyn Error>>{
    let vsomeip_config = smip::VsomeIpConfig::new()
    .application_id(("SimpleClient".to_string(), 0xEF))
        .service_discovery(false)
        .instance_id(0x1)
        .service(smip::VSomeIpServiceConfig {
            id: 0x1234,
            conn_type: smip::ConnectionType::Udp(30509),
        })
        .netmask(IpAddr::V4(Ipv4Addr::new(255, 255, 240, 0)))
        .addr(IpAddr::V4(Ipv4Addr::new(172, 31, 43, 55)));


    let client = Client::new(&vsomeip_config)?;

    loop {
        let response: i32 = client.send(0x1, 5)?;
        dbg!(response);
        
        let response: String = client.send(0x2, ())?;
        dbg!(response);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
