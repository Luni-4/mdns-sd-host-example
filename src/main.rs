use std::collections::HashMap;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

use mdns_sd::{IfKind, ServiceDaemon, ServiceEvent, ServiceInfo};

use tracing::info;
use tracing_subscriber::filter::LevelFilter;

const SERVICE_TYPE: &str = "_arco._tcp.local.";

async fn server() {
    let mdns = ServiceDaemon::new().unwrap();

    let service = ServiceInfo::new(
        // Service type
        SERVICE_TYPE,
        // Service instance name
        "device",
        // Hostname
        "arco.local.",
        // Considered IP address which allow to reach out the service.
        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        // Port on which the service listens to. It has to be same of the
        // server.
        3000,
        // Service properties
        HashMap::new(),
    )
    .unwrap()
    .enable_addr_auto();

    mdns.register(service).unwrap();
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber.
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    tokio::spawn(async { server().await });

    let mdns = ServiceDaemon::new().unwrap();

    mdns.disable_interface(IfKind::IPv6).unwrap();

    mdns.disable_interface(IpAddr::V4(Ipv4Addr::new(172, 17, 0, 1)))
        .unwrap();

    // Detects devices.
    let receiver = mdns.browse(SERVICE_TYPE).unwrap();

    while let Ok(event) = receiver.recv_timeout(Duration::from_secs(1)) {
        if let ServiceEvent::ServiceResolved(info) = event {
            info!("{:?}", info);
        }
    }

    // Stop detection.
    mdns.stop_browse(SERVICE_TYPE).unwrap();
}
