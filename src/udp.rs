use crate::broadcaster::BroadcastMessage;
use actix::Addr;
use roxmltree::Document;
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
            Ok(xml) => {
                info!("Raw XML from {}: {}", addr, xml);

                match Document::parse(xml) {
                    Ok(doc) => {
                        if let Some(event) = doc.descendants().find(|n| n.has_tag_name("event")) {
                            let uid = event.attribute("uid").unwrap_or("");
                            let cot_type = event.attribute("type").unwrap_or("");
                            let time = event.attribute("time").unwrap_or("");
                            let stale = event.attribute("stale").unwrap_or("");
                            let how = event.attribute("how").unwrap_or("");
                            info!(
                                "Parsed CoT: uid={} type={} time={} stale={} how={}",
                                uid, cot_type, time, stale, how
                            );
                        } else {
                            error!("No <event> element found");
                        }
                        broadcaster.do_send(BroadcastMessage(xml.to_string()));
                    }
                    Err(e) => error!("Failed to parse XML with roxmltree: {}", e),
                }
            }
            Err(e) => error!("Invalid UTF-8 from {}: {}", addr, e),
        }
    }
}
