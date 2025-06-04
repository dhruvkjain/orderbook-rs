#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    GoodTillCancel,
    FillAndKill,      // if it can't match any order discard it, if it can resolve it.
    // FillOrKill,
    // GoodForDay,
    Market,           // whatever the best market price is just buy/sell.
}
