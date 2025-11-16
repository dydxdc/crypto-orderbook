pub(crate) struct Price(f64);

pub enum OrderId {
    Uuid(Uuid),

    U64(u64),
}
