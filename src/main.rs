use std::f32::NAN;
use std::rc::Rc;
use std::cell::RefCell;
use orderbook_rs::helperfns::print_trades;
use orderbook_rs::*;

fn main() {
    let mut orderbook = OrderBook::new();
    let order_id = 1;
    let o1:Order = Order::new(order_id, OrderType::GoodTillCancel, Side::Buy, OrderedFloat(100.02), 10);
    let order_id = 2;
    let o2:Order = Order::new(order_id, OrderType::Market, Side::Sell, OrderedFloat(NAN), 12);
    if let Some(t2) = orderbook.add_order(Rc::new(RefCell::new(o2))){
        print_trades(t2);
    };
    if let Some(t1) = orderbook.add_order(Rc::new(RefCell::new(o1))){
        print_trades(t1);
    };


    println!("OrderBook size: {}", orderbook.size());
    // orderbook.cancel_order(order_id);
    let orderlevelinfos= orderbook.get_orderlevelinfos();
    let bidslevel = orderlevelinfos.get_bids();
    let askslevel = orderlevelinfos.get_asks();
    
    println!("Bids:");
    let lvl = 0;
    for bidlvl in bidslevel {
        println!("Level: {} -> Price: {}, Quantity: {}", lvl, bidlvl.price, bidlvl.quantity);
    };

    println!();
    println!("Asks:");
    let lvl = 0;
    for asklvl in askslevel {
        println!("Level: {} -> Price: {}, Quantity: {}", lvl, asklvl.price, asklvl.quantity);
    };
}
