use std::net::UdpSocket;
use log::info;
use solana_sdk::transaction::VersionedTransaction;

fn main() -> Result<(), std::io::Error> {
    env_logger::builder()
        .format_timestamp_micros()
        .init();

    let socket = UdpSocket::bind("127.0.0.1:8002")?;
    info!("UDP receiver listening on 127.0.0.1:8002");

    let mut buf = [0u8; 4096];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                match bincode::deserialize::<VersionedTransaction>(&buf[..size]) {
                    Ok(transaction) => {
                        info!("RECEIVED LAUNCH TRANSACTION from {}: {}", src, transaction.signatures[0]);
                    }
                    Err(e) => {
                        info!("Failed to deserialize transaction from {}: {}", src, e);
                    }
                }
            }
            Err(e) => {
                info!("UDP receive error: {}", e);
            }
        }
    }
}
