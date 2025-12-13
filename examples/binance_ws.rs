use orderbook::ws::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "wss://fstream.binance.com/ws/btcusdt@depth";
    println!("Connecting to {}", url);

    let mut rx = connect(url).await?;

    while let Some(mut msg) = rx.recv().await {
        match simd_json::from_slice::<orderbook::binance::types::DepthUpdate<'_>>(&mut msg) {
            Ok(depth_update) => {
                if depth_update.symbol == "BTCUSDT" && depth_update.event_type == "depthUpdate" {
                    let order: orderbook::orderbook_l2::L2Order<
                        orderbook::binance::types::DepthUpdateSeq,
                    > = depth_update.into();
                    println!("Received Order: {:?}", order.id);
                }
            }
            Err(e) => {
                eprintln!("Error parsing message: {:?}", e);
                // Also print the raw message for debugging valid UTF8 errors or partial fragments
                if let Ok(s) = std::str::from_utf8(&msg) {
                    println!("Raw (valid utf8): {}", s);
                } else {
                    println!("Raw (bytes): {:?}", msg);
                }
            }
        }
    }

    Ok(())
}
