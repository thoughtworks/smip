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
            major_version: 1,
            minor_version: 0
        });


    let client = Client::new(&vsomeip_config)?;

    loop {
        let response: i32 = client.send(0x1, 5)?;
        dbg!(response);
        
        let response: String = client.send(0x2, ())?;
        dbg!(response);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
