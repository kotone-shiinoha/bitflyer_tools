use rust_decimal::Decimal;

use super::Side;

pub struct Execution {
    id: i64,
    side: Side,
    price: Decimal,
    size: Decimal,
    exec_date: chrono::NaiveDateTime,
    buy_child_order_acceptance_id: String,
    sell_child_order_acceptance_id: String,
}
