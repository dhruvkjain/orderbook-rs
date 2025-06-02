#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    GoodTillCancel,
    FillAndKill, // if it can't match any order discard it, if it can resolve it.
}
