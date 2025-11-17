use serde::{Deserialize, Serialize};

use crate::orderbook_l2::{L2Order, L2Price, L2PriceSize, L2Size, Sequence};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdate {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub last_update_id: u64,

    #[serde(rename = "pu")]
    pub previous_update_id: u64,

    #[serde(rename = "b")]
    pub bids: Vec<PriceSize>,

    #[serde(rename = "a")]
    pub asks: Vec<PriceSize>,
}

impl From<DepthUpdate> for L2Order<DepthUpdate> {
    fn from(val: DepthUpdate) -> Self {
        let bids = val.bids.iter().cloned().map(Into::into).collect();
        let asks = val.asks.iter().cloned().map(Into::into).collect();

        L2Order {
            id: Sequence(val.last_update_id),
            bids,
            asks,
            is_snapshot: false,
            o: val,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PriceSize(
    #[serde(with = "f64_to_u64")] pub u64, // price
    #[serde(with = "f64_to_u64")] pub u64, // size
);

impl From<PriceSize> for L2PriceSize {
    fn from(val: PriceSize) -> Self {
        L2PriceSize(L2Price(val.0), L2Size(val.1))
    }
}

pub(crate) mod f64_to_u64 {
    use serde::{Deserialize, Deserializer, Serializer, de};

    trait ToU64<E: de::Error> {
        fn to_u64(self) -> Result<u64, E>;
    }

    const FLOAT_SCALE: f64 = 10_000_000_000.0;
    impl<D: de::Error> ToU64<D> for f64 {
        fn to_u64(self) -> Result<u64, D> {
            let n = (self * FLOAT_SCALE).floor();
            if !n.is_finite() || n < 0.0 || n > u64::MAX as f64 {
                return Err(de::Error::custom("cannot convert to u64, invalid float"));
            }
            Ok(n as u64)
        }
    }

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let f = (*value as f64) / FLOAT_SCALE;
        serializer.serialize_f64(f)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Num {
            S(String),
            F(f64),
        }

        match Num::deserialize(deserializer)? {
            Num::S(s) => {
                let f: f64 = s.parse().map_err(de::Error::custom)?;
                f.to_u64()
            }
            Num::F(f) => f.to_u64(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binance::types::{DepthUpdate, PriceSize};

    #[test]
    fn deserialize_depth_update() {
        const FLOAT_SCALE: f64 = 10_000_000_000.0;

        let d = r#"{"e":"depthUpdate","E":1571889248277,"T":1571889248276,"s":"BTCUSDT","U":390497796,"u":390497878,"pu":390497794,"b":[["7403.89","0.002"],["7403.90","3.906"],["7404.00","1.428"]],"a":[["7405.96","3.340"],["7406.63","4.525"],["7407.08","2.475"]]}"#;
        let depth_update: DepthUpdate = serde_json::from_str(d).unwrap();

        assert_eq!(depth_update.event_type, "depthUpdate");
        assert_eq!(depth_update.event_time, 1571889248277);
        assert_eq!(depth_update.transaction_time, 1571889248276);
        assert_eq!(depth_update.symbol, "BTCUSDT");

        let expected_bids = vec![
            PriceSize((7403.89 * FLOAT_SCALE) as u64, (0.002 * FLOAT_SCALE) as u64),
            PriceSize((7403.90 * FLOAT_SCALE) as u64, (3.906 * FLOAT_SCALE) as u64),
            PriceSize((7404.00 * FLOAT_SCALE) as u64, (1.428 * FLOAT_SCALE) as u64),
        ];

        let expected_asks = vec![
            PriceSize((7405.96 * FLOAT_SCALE) as u64, (3.340 * FLOAT_SCALE) as u64),
            PriceSize((7406.63 * FLOAT_SCALE) as u64, (4.525 * FLOAT_SCALE) as u64),
            PriceSize((7407.08 * FLOAT_SCALE) as u64, (2.475 * FLOAT_SCALE) as u64),
        ];

        assert_eq!(depth_update.bids, expected_bids);
        assert_eq!(depth_update.asks, expected_asks);
    }
}
