use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use log::info;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::builder()
        .format_timestamp_micros()
        .init();
    let mut client = ShredstreamProxyClient::connect("http://127.0.0.1:9999")
        .await
        .unwrap();
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(slot_entry) = stream.message().await.unwrap() {
        let transaction = match bincode::deserialize::<solana_sdk::transaction::VersionedTransaction>(&slot_entry.entries) {
            Ok(transaction) => transaction,
            Err(e) => {
                println!("Deserialization failed with err: {e}");
                continue;
            }
        };
        
        info!(
            "RECEIVED LAUNCH TRANSACTION - Slot {}: {}",
            slot_entry.slot,
            transaction.signatures[0]
        );
    }
    Ok(())
}
