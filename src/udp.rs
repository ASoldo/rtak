use crate::broadcaster::BroadcastMessage;
use crate::cot::CoT;
use actix::Addr;
use quick_xml::de::from_str;
use tokio::net::UdpSocket;
use tracing::{error, info};

pub async fn start_udp_listener(
    bind_addr: &str,
    broadcaster: Addr<crate::broadcaster::Broadcaster>,
) -> std::io::Result<()> {
    let socket = UdpSocket::bind(bind_addr).await?;
    let mut buf = vec![0u8; 65535];

    info!("UDP listener bound to {}", bind_addr);

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let data = &buf[..len];

        match std::str::from_utf8(data) {
            Ok(xml) => match from_str::<CoT>(xml) {
                Ok(cot) => {
                    info!("Received CoT from {}: {:?}", addr, cot);
                    broadcaster.do_send(BroadcastMessage(xml.to_string()));
                }
                Err(e) => error!("Failed to parse CoT XML: {}", e),
            },
            Err(e) => error!("Invalid UTF-8 from {}: {}", addr, e),
        }
    }
}
