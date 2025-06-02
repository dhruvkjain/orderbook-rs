use std::rc::Rc;
use std::cell::RefCell;
use super::{OrderId, Price, Side, Quantity, OrderType, OrderPointer, Order};

pub struct OrderModify {
    order_id: OrderId,
    price: Price,
    side: Side,
    quantity: Quantity,
}
impl OrderModify {
    pub fn new(order_id: OrderId, side: Side, price: Price, quantity: Quantity) -> Self {
        Self {
            order_id,
            side,
            price,
            quantity,
        }
    }

    pub fn get_order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn get_price(&self) -> Price {
        self.price
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_quantity(&self) -> Quantity {
        self.quantity
    }

    // Only orders we want to modify are `GoodTillCancel`
    // But added OrderType to support types added in future
    pub fn to_order_pointer(&self, order_type: OrderType) -> OrderPointer {
        Rc::new(RefCell::new(Order::new(
            self.get_order_id(),
            order_type,
            self.get_side(),
            self.get_price(),
            self.get_quantity(),
        )))
    }
}
