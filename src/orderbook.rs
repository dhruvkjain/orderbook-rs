use std::cmp::{Reverse, min};
use super::*;

pub struct OrderEntry {
    order: OrderPointer,
    location_index: usize,
}

pub struct OrderBook {
    orders: HashMap<OrderId, OrderEntry>, // all orders access in O(1)
    bids: BTreeMap<Reverse<Price>, OrderPointers>, // Price-Time priority sorted high -> low
    asks: BTreeMap<Price, OrderPointers>, // Price-Time priority sorted low -> high
}
impl OrderBook {
    fn can_match(&self, side: Side, price: Price) -> bool {
        if side == Side::Buy {
            if self.asks.is_empty() {
                return false;
            }
            let (best_ask, _) = self.asks.iter().next().unwrap();
            return price >= *best_ask;
        } else {
            if self.bids.is_empty() {
                return false;
            }
            let (Reverse(best_bid), _) = self.bids.iter().next().unwrap();
            return price <= *best_bid;
        }
    }

    fn match_orders(&mut self) -> Trades {
        let mut trades: Trades = Vec::with_capacity(self.orders.len());

        loop {
            if self.bids.is_empty() || self.asks.is_empty() {
                println!("Empty Market !!!");
                break;
            }
            let (Reverse(best_bid_price), _) = self.bids.iter().next().unwrap();
            let (best_ask_price, _) = self.asks.iter().next().unwrap();

            if *best_bid_price < *best_ask_price {
                println!("Can't match order: current best bid price < current best ask price");
                break;
            }

            loop {
                {
                    let (_, bids) = self.bids.iter_mut().next().unwrap();
                    let (_, asks) = self.asks.iter_mut().next().unwrap();

                    if !bids.is_empty() && !asks.is_empty() {
                        break;
                    }

                    {
                        let mut bid = bids.front().unwrap().borrow_mut();
                        let mut ask = asks.front().unwrap().borrow_mut();

                        let quantity =
                            min(bid.get_remaining_quantity(), ask.get_remaining_quantity());

                        bid.fill(quantity);
                        ask.fill(quantity);

                        if bid.isfilled() {
                            self.orders.remove(&bid.get_order_id());
                        }
                        if ask.isfilled() {
                            self.orders.remove(&ask.get_order_id());
                        }
                    } // this scope is to kill the mutable reference of bids and asks

                    if bids.front().unwrap().borrow().isfilled() {
                        bids.pop_front();
                    }
                    if asks.front().unwrap().borrow().isfilled() {
                        asks.pop_front();
                    }
                } // this scope is to kill the mutable reference of self.bids and self.asks

                let best_bid_price = *self.bids.keys().next().unwrap();
                let best_ask_price = *self.asks.keys().next().unwrap();
                // cloning the keys, to no longer hold an immutable borrow so that we can
                // mutably borrow self.bids and self.asks to use remove on them

                let bids_empty = self.bids.get(&best_bid_price).unwrap().is_empty();
                let asks_empty = self.asks.get(&best_ask_price).unwrap().is_empty();

                if bids_empty {
                    self.bids.remove(&best_bid_price);
                }
                if asks_empty {
                    self.asks.remove(&best_ask_price);
                }

                let bid = self
                    .bids
                    .get(&best_bid_price)
                    .unwrap()
                    .front()
                    .unwrap()
                    .borrow();
                let ask = self
                    .asks
                    .get(&best_ask_price)
                    .unwrap()
                    .front()
                    .unwrap()
                    .borrow();
                let trade = Trade::new(
                    TradeInfo::new(
                        bid.get_order_id(),
                        bid.get_price(),
                        min(bid.get_remaining_quantity(), ask.get_remaining_quantity()),
                    ),
                    TradeInfo::new(
                        ask.get_order_id(),
                        ask.get_price(),
                        min(bid.get_remaining_quantity(), ask.get_remaining_quantity()),
                    )
                );
                trades.push(trade);
            }
        }

        // After matching we need to kill orders of type FillAndKill
        if !self.bids.is_empty() {
            let mut should_cancel = false;
            let mut order_id = 0;
            {
                let (_, bids) = self.bids.iter().next().unwrap();
                let bid = bids.front().unwrap().borrow();
                should_cancel = bid.get_order_type() == OrderType::FillAndKill;
                order_id = bid.get_order_id();
            }
            if should_cancel {
                self.cancel_order(order_id);
            }
        }

        if !self.asks.is_empty() {
            let mut should_cancel = false;
            let mut order_id = 0;
            {
                let (_, asks) = self.asks.iter().next().unwrap();
                let ask = asks.front().unwrap().borrow();
                should_cancel = ask.get_order_type() == OrderType::FillAndKill;
                order_id = ask.get_order_id();
            }
            if should_cancel {
                self.cancel_order(order_id);
            }
        }

        trades
    }

    pub fn new() -> Self {
        Self { orders: HashMap::new() , bids: BTreeMap::new(), asks: BTreeMap::new() }
    }

    pub fn add_order(&mut self, order:OrderPointer) -> Option<Trades> {
        if self.orders.contains_key(&order.borrow().get_order_id()){
            return None;
        } 
        
        if order.borrow().get_order_type() == OrderType::FillAndKill && !self.can_match(order.borrow().get_side(), order.borrow().get_price()) {
            return None;
        }
        
        let mut order_index:usize = 0;
        if order.borrow().get_side() == Side::Buy {
            let orders = self.bids.entry(Reverse(order.borrow().get_price())).or_default();
            orders.push_back(order.clone());
            order_index = orders.len() - 1;
        } else {
            let orders = self.asks.entry(order.borrow().get_price()).or_default();
            orders.push_back(order.clone());
            order_index = orders.len() - 1;
        }
        
        self.orders.insert(
            order.borrow().get_order_id(), 
            OrderEntry { 
                order: order.clone(), 
                location_index: order_index 
            }
        );

        return Some(self.match_orders());
    }

    pub fn cancel_order(&mut self, order_id:OrderId) {
        if self.orders.contains_key(&order_id){
            return ;
        } 

        let (_, order_entry) = self.orders.remove_entry(&order_id).unwrap(); 
        if order_entry.order.borrow().get_side() == Side::Sell {
            let orders = self.asks.get_mut(&order_entry.order.borrow().get_price()).unwrap();
            orders.remove(order_entry.location_index);

            // if all orders at a price level are matched, remove that level
            if orders.is_empty() {
                self.asks.remove(&order_entry.order.borrow().get_price());
            }
        } else {
            let orders = self.bids.get_mut(&Reverse(order_entry.order.borrow().get_price())).unwrap();
            orders.remove(order_entry.location_index);

            // if all orders at a price level are matched, remove that level
            if orders.is_empty() {
                self.bids.remove(&Reverse(order_entry.order.borrow().get_price()));
            }
        }
    }

    pub fn modify_order(&mut self, order: OrderModify) -> Option<Trades> {
        if !(self.orders.contains_key(&order.get_order_id())) {
            return None;
        }

        let (_, order_entry) = self.orders.get_key_value(&order.get_order_id()).unwrap();
        let order_type_clone = order_entry.order.borrow().get_order_type();
        
        self.cancel_order(order.get_order_id());
        
        return self.add_order(order.to_order_pointer(order_type_clone));
    }

    pub fn size(&self) -> usize {
        return self.orders.len();
    }

    pub fn get_orderlevelinfos(&self) -> OrderbookLevelInfos {
        let mut bid_infos = Vec::with_capacity(self.orders.len());
        let mut ask_infos = Vec::with_capacity(self.orders.len());

        fn create_level_info(price: Price, orders: &OrderPointers) -> LevelInfo {
            let mut quantity = 0;
            for order in orders.iter() {
                quantity += order.borrow().get_remaining_quantity();
            }
            LevelInfo::new(price, quantity)
        }

        for (Reverse(price), orders) in &self.bids {
            bid_infos.push(create_level_info(*price, &orders));
        }

        for (price, orders) in &self.asks {
            ask_infos.push(create_level_info(*price, &orders));
        }

        return OrderbookLevelInfos::new(bid_infos, ask_infos);
    }
}
