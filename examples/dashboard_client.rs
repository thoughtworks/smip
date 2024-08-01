use std::error::Error;

use smip::Client;

fn main() -> Result<(), Box<dyn Error>>{

    let service_id = 0x1111;
    let instance_id = 0x2222;


    let vsomeip_config = smip::VsomeIpConfig::new()
    .application_id(("DashboardClient".to_string(), 0x1212))
        .service_discovery(false)
        .instance_id(instance_id)
        .service(smip::VSomeIpServiceConfig {
            id: service_id,
            conn_type: smip::ConnectionType::Udp(30509),
            ..Default::default()
        });

    let client = Client::new(&vsomeip_config)?;

    loop {
        let speed: i32 = client.send(0x3333, ())?;
        println!("Speed: {}", speed);
        
        let battery: u8 = client.send(0x4444, ())?;
        println!("Battery: {}", battery);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
