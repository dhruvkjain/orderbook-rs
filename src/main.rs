use std::rc::Rc;
use std::cell::RefCell;
use orderbook_rs::*;

fn main() {
    let mut orderbook = OrderBook::new();
    let order_id = 1;
    let order:Order = Order::new(order_id, OrderType::GoodTillCancel, Side::Buy, 100, 10);

    orderbook.add_order(Rc::new(RefCell::new(order)));
    println!("{}", orderbook.size());
    orderbook.cancel_order(order_id);
    println!("{}", orderbook.size());
}
