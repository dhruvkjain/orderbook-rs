use super::Trades;

pub fn print_trades (trades: &Trades) {
    for trade in trades {
        let bid_trade = trade.get_bid_trade();
        let ask_trade = trade.get_ask_trade();
        println!("Trade: {} -> {}", ask_trade.order_id, bid_trade.order_id);
        println!("  Bid: price = {}, qunatity = {}", bid_trade.price, bid_trade.quantity);
        println!("  Ask: price = {}, qunatity = {}", ask_trade.price, ask_trade.quantity);
    }
}
