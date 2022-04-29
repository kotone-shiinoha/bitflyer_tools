use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CommissionRate {
    commission_rate: Decimal
}