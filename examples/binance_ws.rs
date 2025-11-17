use orderbook::binance::ws::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "wss://fstream.binance.com/ws/btcusdt@aggTrade";
    println!("Connecting to {}", url);

    let mut rx = connect(url).await?;
    println!("Connected!");

    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }

    Ok(())
}
