use super::Queue;

pub struct OrderBook {
    asks: std::collections::BTreeMap<u64, Queue>,
}
