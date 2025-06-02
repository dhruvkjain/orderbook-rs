use super::{Price, Quantity};

pub struct LevelInfo {
    price: Price,
    quantity: Quantity,
}
impl LevelInfo {
    pub fn new(price: Price, quantity: Quantity) -> Self {
        Self { price, quantity }
    }
}

pub struct OrderbookLevelInfos {
    bids: Vec<LevelInfo>,
    asks: Vec<LevelInfo>,
}
impl OrderbookLevelInfos {
    pub fn new(bids: Vec<LevelInfo>, asks: Vec<LevelInfo>) -> Self {
        Self { bids, asks }
    }
    pub fn get_bids(&self) -> &Vec<LevelInfo> {
        &self.bids
    }
    pub fn get_sks(&self) -> &Vec<LevelInfo> {
        &self.asks
    }
}
