# Crypto Orderbook

In-progress project serving as a launchpad for learning Rust and systems programming. The goal is to implement a runtime-agnostic, local-orderbook for crypto exchanges.

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

