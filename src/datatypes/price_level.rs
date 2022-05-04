use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevel {
    pub price: Decimal,
    pub size: Decimal,
}
