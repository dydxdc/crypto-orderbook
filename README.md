# Crypto Orderbook

In-progress project serving as a launchpad for learning Rust and systems programming. The goal is to implement a runtime-agnostic, local-orderbook for crypto exchanges.

## Quick Start

The following example uses tokio for async runtime.
```rust
use orderbook::binance::Book;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let depth = 5000;
    let interval = Duration::from_millis(100);
    let mut book = Book::new_um("BTCUSDT", depth, interval);

    let writer = book.writer();

    // send book updates
    writer.update(...); 

    while let Some(snapshot) = book.recv().await {
        let mid: f64 = snapshot.mid().into();
        println!("Mid Price: {}", mid));
    }
}
```

**Running Example**
```bash
cargo run --example binance_ws
```

```rust
use orderbook::binance::types::DepthUpdate;
use orderbook::ws::connect;
use serde_json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = tokio_rustls::rustls::crypto::ring::default_provider().install_default();
    let url = "wss://fstream.binance.com/ws/btcusdt@depth";

    let mut ws = connect(url).await?;

    let mut book = orderbook::binance::Book::new_um("BTCUSDT", 1000, Duration::from_millis(0));

    let writer = book.writer();
    tokio::spawn(async move {
        loop {
            let Some(res) = ws.rx.recv().await else { break };
            let json = match res {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                    break;
                }
            };

            match serde_json::from_slice::<DepthUpdate>(&json) {
                Ok(depth_update) => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                    let latency = now - depth_update.seq.event_time_ms;
                    println!("Received depth update latency: {}ms", latency);

                    writer.update(depth_update.into()).await;
                }
                Err(e) => {
                    eprintln!("Error parsing message: {:?}", e);
                }
            }
        }
    });

    while let Some(snapshot) = book.recv().await {
        let mid: f64 = snapshot.mid().into();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let latency = now - snapshot.ts_ms;
        println!("Received snapshot mid price: {:.2}, latency: {}ms", mid, latency);
    }

    Ok(())
}
```

## Goals
- **Runtime Agnostic Design**: Decoupling the core orderbook logic from specific async runtimes.
- **Efficient Serialization**: Exploring efficient serialization techniques for WebSocket data streams, such as zero-copy and simd.

## Status
- [ ] **Orderbooks** 
    - [x] L2 Orderbook
    - [ ] L3 MBO Orderbook (maybe coinbase?)

- [ ] **Binance WS** 
    - [ ] fastwebsockets and simd -json


### Still researching 
- [ ] **Zero-copy Serialization**
- [ ] **SIMD JSON Parsing** (`simd-json`)

## Project Structure

```
├── examples/           # Usage examples
│   └── binance_ws.rs   # Experimental Binance WebSocket connector
├── src/
│   ├── binance/        # Exchange-specific implementations
│   │   └── types.rs    # serde-compatible type definitions
│   ├── orderbook_l2/   # orderbook 
│   │   ├── book.rs     # l2-orderbook fsm 
│   │   └── queue.rs    # Order queue (for l3) 
│   └── lib.rs
```

