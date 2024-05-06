
use std::{io::Write, net::{IpAddr, Ipv4Addr}};

use vsomeip_rs::{ServiceId, InstanceId};
use tempfile::NamedTempFile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VsomeIpConfig {
    pub app_id: (String, u16),
    pub services: Vec<VSomeIpServiceConfig>,
    pub addr: IpAddr,
    pub netmask: IpAddr,
    pub addr_mode: AddressingMode,
    pub service_discovery: bool,   
    pub instance_id: InstanceId 
}

impl VsomeIpConfig {
    pub fn new() -> Self {
        Self {
            app_id: ("smip_app".to_string(), 0),
            services: vec![],
            service_discovery: false,
            addr_mode: AddressingMode::Unicast,
            netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            instance_id: 0,
        }
    }
    fn build_services(&self, instance_id: InstanceId) -> String {
        let mut services = String::new();
        
        services.push_str("\"services\": [");
        for service in &self.services {
            services.push_str("{");
            services.push_str(&format!("\"service\": \"{}\", ", service.id));
            services.push_str(&format!("\"instance\": \"{}\", ", instance_id));
            
            match service.conn_type {
                ConnectionType::Tcp(port) => {
                    services.push_str("\"reliable\": {");
                    services.push_str(&format!("\"port\": {}, ", port));
                    services.push_str("\"enable-magic-cookie\": \"false\"");
                    services.push_str("}");
                    
                },
                ConnectionType::Udp(port) => {
                    services.push_str(&format!("\"unreliable\": {}", port));
                },
            }
            services.push(',');
            services.push_str(&self.build_addr_mode());
            services.push_str("}");
        }
        services.push_str("]");
        services
    }
    fn build_addr_mode(&self) -> String {
        match self.addr_mode {
            AddressingMode::Unicast => format!("\"unicast\": \"{}\"", self.addr),
            AddressingMode::Multicast => format!("\"multicast\": \"{}\"", self.addr),
        }
    }
    pub fn build(self) -> String {
        let addr_mode = self.build_addr_mode();
        let net_mask = format!("\"netmask\": \"{}\"", self.netmask);

        let logging = "\"logging\": {\"level\": \"debug\", \"console\": \"true\", \"file\": {\"enable\": \"false\", \"path\": \"/tmp/vsomeip.log\"}, \"dlt\": \"false\"}";
        let services = self.build_services(self.instance_id);
        let service_discovery = format!("\"service-discovery\": {{\"enable\": \"{}\"}}", self.service_discovery);

        let applications = format!("\"applications\": [{{\"name\": \"{}\", \"id\": \"{}\"}}, {{\"name\": \"{}\", \"id\": \"{}\"}}]", self.app_id.0, self.app_id.1, "hello_world_client", "0x1212");
        let routing = format!("\"routing\": \"{}\"", self.app_id.0);

        format!("
        {{
            {addr_mode},
            {net_mask},
            {logging},
            {applications},
            {services},
            {routing},
            {service_discovery}
        }}")
    } 
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Tcp(u16),
    Udp(u16)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Unicast,
    Multicast
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VSomeIpServiceConfig {
    pub id: ServiceId,
    pub conn_type: ConnectionType,
}

pub fn set_vsomeip_config(config: &str) {
    if std::env::var("VSOMEIP_CONFIGURATION").is_ok() {
        println!("VSOMEIP_CONFIGURATION is already set, using that...");
        return;
    }

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    temp_file.write(config.as_ref()).expect("Failed to write to temporary file");
    temp_file.flush().expect("Failed to flush temporary file");

    let config_path = temp_file.path();

    println!("Wrote vsomeip config to {}", config_path.display());

    std::env::set_var("VSOMEIP_CONFIGURATION", config_path);
}