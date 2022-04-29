use rust_decimal::Decimal;
use serde::{Serialize};

use crate::datatypes::{OrderType, Side};
#[derive(Serialize)]
pub struct Parameter<'a> {
    product_code: &'a str,
    condition_type: OrderType,
    side: Side,
    price: i64,
    size: Decimal
}