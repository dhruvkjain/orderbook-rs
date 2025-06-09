pub use std::collections::{BTreeMap, HashMap, VecDeque};
pub use ordered_float::OrderedFloat;
pub use levelinfos::{LevelInfo, OrderbookLevelInfos};
pub use order::Order;
pub use modifyorder::OrderModify;
pub use trade::{Trade, TradeInfo};
pub use orderbook::OrderBook;

pub use ordertypes::OrderType;
pub use side::Side;
// ----------------------------
// rc: When multiple ownership is needed of same heap allocated
// https://doc.rust-lang.org/rust-by-example/std/rc.html
// ----------------------------
use std::rc::Rc;
// ----------------------------
// RefCell: When multiple mutable references ownership is needed of same heap allocated
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// ----------------------------
use std::cell::RefCell;



pub type Price = OrderedFloat<f32>;
pub type Quantity = u32;
pub type OrderId = u64;
// ----------------------------
// OrderPointer is a mutable owned reference to a heap allocated variable which can be shared during runtime
// ----------------------------
pub type OrderPointer = Rc<RefCell<Order>>;
pub type OrderPointers = VecDeque<OrderPointer>;
pub type Trades = Vec<Trade>;

pub mod levelinfos;
pub mod order;
pub mod modifyorder;
pub mod trade;
pub mod ordertypes;
pub mod side;
pub mod helperfns;
pub mod orderbook;