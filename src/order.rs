use super::{OrderId, OrderType, Side, Price, Quantity};

pub struct Order {
    order_id: OrderId,
    order_type: OrderType,
    side: Side,
    price: Price,
    initial_quantity: Quantity,
    remaining_quantity: Quantity,
}
impl Order {
    pub fn new(
        order_id: OrderId,
        order_type: OrderType,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Self {
            order_id,
            order_type,
            side,
            price,
            initial_quantity: quantity,
            remaining_quantity: quantity,
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
        self.get_initial_quantity() - self.get_remaining_quantity()
    }

    pub fn isfilled(&self) -> bool {
        self.get_remaining_quantity() == 0
    }
    pub fn fill(&mut self, quantity: Quantity) {
        if quantity > self.get_remaining_quantity() {
            println!(
                "Order {} can't be filled for more than it's remaining quantity",
                self.order_id
            );
        }
        self.remaining_quantity -= quantity;
    }
}
