#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderType {
    GoodTillCancel,   // persist until filled or canceled.
    FillAndKill,      // fill as much as possible immediately, any remaining qunatity is cancelled.
    // FillOrKill,       // completely filled immediately or cancel entire order.
    // GoodForDay,       // active for current trading day, automatically cancelled if not filled by end of day.
    Market,           // whatever the best market price is just buy/sell.
}
