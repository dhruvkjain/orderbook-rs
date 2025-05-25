// ----------------------------
// rc: When multiple ownership is needed of same heap allocated 
// https://doc.rust-lang.org/rust-by-example/std/rc.html
// ----------------------------
use std::rc::Rc;
use std::cell::RefCell;

type Price = i32;
type Quantity = u32;
type OrderId = u64;
type OrderPointer = Rc<RefCell<Order>>;

#[derive(Copy, Clone, Debug)]
enum OrderType {
    GoodTillCancel,
    FillAndKill
}

#[derive(Copy, Clone, Debug)]
enum Side {
    Buy,
    Sell
}

struct LevelInfo {
    price: Price,
    quantity: Quantity
}

struct OrderbookLevelInfos {
    bids:Vec<LevelInfo>,
    asks:Vec<LevelInfo>
}
impl OrderbookLevelInfos {
    pub fn new(bids: Vec<LevelInfo>, asks: Vec<LevelInfo>) -> Self{
        Self { bids, asks }
    }
    pub fn get_bids(&self) -> &Vec<LevelInfo>{
        &self.bids
    }
    pub fn get_sks(&self) -> &Vec<LevelInfo>{
        &self.asks
    }
}

struct Order {
    order_id: OrderId,
    order_type:OrderType,
    side: Side,
    price: Price,
    initial_quantity: Quantity,
    remaining_quantity: Quantity
}
impl Order {
    pub fn new(
        order_id: OrderId, 
        order_type: OrderType, 
        side: Side,
        price: Price,
        quantity: Quantity
    ) -> Self {
        Self { 
            order_id, 
            order_type, 
            side, 
            price, 
            initial_quantity: quantity, 
            remaining_quantity: quantity
        }
    }

    pub fn get_order_id(&self) -> OrderId {
        self.order_id
    }
    pub fn get_order_type(&self) -> OrderType {
        self.order_type
    }
    pub fn get_side(&self) -> Side {
        self.side
    }
    pub fn get_price(&self) -> Price {
        self.price
    }
    pub fn get_initial_quantity(&self) -> Quantity {
        self.initial_quantity
    }
    pub fn get_remaining_quantity(&self) -> Quantity {
        self.remaining_quantity
    }
    pub fn get_filled_quantity(&self) -> Quantity {
        self.get_initial_quantity()-self.get_remaining_quantity()
    }

    pub fn fill(&mut self, quantity: Quantity){
        if quantity > self.get_remaining_quantity(){
            println!("Order {} can't be filled for more than it's remaining quantity", self.order_id);
        }
        self.remaining_quantity -= quantity;
    }
}