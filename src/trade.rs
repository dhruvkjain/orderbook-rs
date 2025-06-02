use super::{OrderId, Price, Quantity};

pub struct TradeInfo {
    order_id: OrderId,
    price: Price,
    quantity: Quantity,
}
impl TradeInfo {
    pub fn new(order_id: OrderId, price: Price, quantity: Quantity) -> Self {
        Self { order_id, price, quantity }
    }
}

pub struct Trade {
    bid_trade: TradeInfo,
    ask_trade: TradeInfo,
}
impl Trade {
    pub fn new(bid_trade: TradeInfo, ask_trade: TradeInfo) -> Self {
        Self {
            bid_trade,
            ask_trade,
        }
    }
    pub fn get_bid_trade(&self) -> &TradeInfo {
        &self.bid_trade
    }
    pub fn get_ask_trade(&self) -> &TradeInfo {
        &self.ask_trade
    }
}
