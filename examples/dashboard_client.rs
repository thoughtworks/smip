use std::net::{IpAddr, Ipv4Addr};
use std::error::Error;

use smip::Client;

fn main() -> Result<(), Box<dyn Error>>{

    let service_id = 0x1111;
    let instance_id = 0x2222;
    let method_id = 0x3333;


    let vsomeip_config = smip::VsomeIpConfig::new()
    .application_id(("DashboardClient".to_string(), 0x1212))
        .service_discovery(false)
        .instance_id(instance_id)
        .service(smip::VSomeIpServiceConfig {
            id: service_id,
            conn_type: smip::ConnectionType::Udp(30509),
        })
        // .service(
        //     smip::VSomeIpServiceConfig {
        //         id: service_id,
        //         conn_type: smip::ConnectionType::Udp(30509),
                
        //     }
        // )
        .netmask(IpAddr::V4(Ipv4Addr::new(255, 255, 240, 0)))
        .addr(IpAddr::V4(Ipv4Addr::new(172, 31, 43, 55)));


    let client = Client::new(&vsomeip_config)?;

    loop {
        let response: i32 = client.send(method_id, 233)?;
        dbg!(response);
        
        let response: String = client.send(0x4444, ())?;
        println!("{}", response);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
