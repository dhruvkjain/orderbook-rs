use std::f32::NAN;
use std::rc::Rc;
use std::cell::RefCell;
use orderbook_rs::helperfns::print_trades;
use orderbook_rs::*;

fn main() {
    let mut orderbook = OrderBook::new();
    let order_id_1 = 1;
    let o1:Order = Order::new(order_id_1, OrderType::GoodTillCancel, Side::Buy, OrderedFloat(100.02), 10);
    let order_id_2 = 2;
    let o2:Order = Order::new(order_id_2, OrderType::Market, Side::Sell, OrderedFloat(NAN), 12);
    let order_id_3 = 3;
    let o3:Order = Order::new(order_id_3, OrderType::GoodTillCancel, Side::Buy, OrderedFloat(100.02), 10);
    let order_id_4 = 4;
    let o4:Order = Order::new(order_id_4, OrderType::FillAndKill, Side::Sell, OrderedFloat(99.4), 27);
    let order_id_5 = 5;
    let o5:Order = Order::new(order_id_5, OrderType::GoodTillCancel, Side::Sell, OrderedFloat(99.02), 10);
    if let Some(trades) = orderbook.add_order(Rc::new(RefCell::new(o1))){
        print_trades(&trades);
    };
    if let Some(trades) = orderbook.add_order(Rc::new(RefCell::new(o2))){
        print_trades(&trades);
    };
    if let Some(trades) = orderbook.add_order(Rc::new(RefCell::new(o3))){
        print_trades(&trades);
    };
    if let Some(trades) = orderbook.add_order(Rc::new(RefCell::new(o4))){
        print_trades(&trades);
    };
    if let Some(trades) = orderbook.add_order(Rc::new(RefCell::new(o5))){
        print_trades(&trades);
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
