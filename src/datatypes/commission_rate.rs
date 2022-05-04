use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommissionRate {
    commission_rate: Decimal,
}
