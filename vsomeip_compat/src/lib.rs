
use std::{io::Write, net::{IpAddr, Ipv4Addr}};

use serde_json::json;
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
    pub instance_id: InstanceId,
    pub routing: Option<String>,
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
            routing: None
        }
    }

    pub fn application_id(mut self, app_id: (String, u16)) -> Self {
        self.app_id = app_id;
        self
    }

    pub fn service_discovery(mut self, enable: bool) -> Self {
        self.service_discovery = enable;
        self
    }
    
    pub fn service(mut self, service: VSomeIpServiceConfig) -> Self {
        self.services.push(service);
        self
    }

    pub fn addr(mut self, addr: IpAddr) -> Self {
        self.addr = addr;
        self
    }

    pub fn netmask(mut self, netmask: IpAddr) -> Self {
        self.netmask = netmask;
        self
    }

    pub fn instance_id(mut self, instance_id: InstanceId) -> Self {
        self.instance_id = instance_id;
        self
    }

    fn build_addr_mode(&self) -> String {
        match self.addr_mode {
            AddressingMode::Unicast => "unicast".into(),
            AddressingMode::Multicast => "multicast".into(),
        }
    }
    pub fn build(self) -> String {
        let addr_mode = self.build_addr_mode();
        let addr_mode = addr_mode.as_str();
        let mut json = json!({
            addr_mode: self.addr,
            "netmask": self.netmask,
            "logging": {
                "level": "debug",
                "console": true,
                "file": {
                    "enable": false,
                    "path": "/tmp/vsomeip.log",
                },
                "dlt": false,
            },
            "applications": [{
                "name": self.app_id.0,
                "id": self.app_id.1.to_string(),
            }],
            "services": self.services.iter().map(move |service| {
                json!({
                    "service": service.id.to_string(),
                    "instance": self.instance_id.to_string(),
                    "unreliable": match service.conn_type {
                        ConnectionType::Tcp(port) => port,
                        ConnectionType::Udp(port) => port,
                    },
                    "reliable": {
                        "port": match service.conn_type {
                            ConnectionType::Tcp(port) => port,
                            ConnectionType::Udp(port) => port,
                        },
                        "enable-magic-cookie": false,
                    },
                    addr_mode: self.addr,
                })
            }).collect::<Vec<_>>(),
            "service-discovery": {
                "enable": self.service_discovery,
            },
        });

        if let Some(routing) = self.routing {
            json["routing"] = json!(routing);
        }

        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        json.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config() {
        let config = VsomeIpConfig {
            app_id: ("smip_app".to_string(), 1),
            services: vec![
                VSomeIpServiceConfig {
                    id: 2,
                    conn_type: ConnectionType::Tcp(30509)
                }
            ],
            service_discovery: false,
            addr_mode: AddressingMode::Unicast,
            netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 23)),
            instance_id: 3,
            routing: None
        };

        let actual = config.build();

        let expected = r#"
        {
            "unicast": "192.168.0.23",
            "netmask": "255.255.255.0",
            "logging": {
                "level": "debug",
                "console": true,
                "file": {
                "enable": false,
                "path": "/tmp/vsomeip.log"
                },
                "dlt": false
            },
            "applications": [{"name": "smip_app", "id": "1"}],
            "services": [
            {
                "service": "2",
                "instance": "3", 
                "reliable": { "port": 30509, "enable-magic-cookie": false},
                "unicast": "192.168.0.23"
            }
            ],
            "service-discovery": {"enable": false}
        }
        "#;
    
        let actual_json: serde_json::Value = serde_json::from_str(&actual).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(expected).unwrap();

        assert_eq!(actual_json, expected_json);

    }

    #[test]
    fn test_config_with_service_discovery() {
        let config = VsomeIpConfig {
            app_id: ("smip_app".to_string(), 1),
            services: vec![
                VSomeIpServiceConfig {
                    id: 2,
                    conn_type: ConnectionType::Tcp(30509)
                }
            ],
            service_discovery: true,
            addr_mode: AddressingMode::Unicast,
            netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 23)),
            instance_id: 3,
            routing: None
        };

        let actual = config.build();

        let expected = r#"
        {
            "applications": [
              {
                "id": "1",
                "name": "smip_app"
              }
            ],
            "logging": {
              "console": true,
              "dlt": false,
              "file": {
                "enable": false,
                "path": "/tmp/vsomeip.log"
              },
              "level": "debug"
            },
            "netmask": "255.255.255.0",
            "service-discovery": {
              "enable": true
            },
            "services": [
              {
                "instance": "3",
                "reliable": {
                  "enable-magic-cookie": false,
                  "port": 30509
                },
                "service": "2",
                "unicast": "192.168.0.23"
              }
            ],
            "unicast": "192.168.0.23"
          }
          "#;

        let actual_json: serde_json::Value = serde_json::from_str(&actual).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(expected).unwrap();

        assert_eq!(actual_json, expected_json);

    }
}