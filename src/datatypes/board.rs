use super::PriceLevel;
use rust_decimal::Decimal;

pub struct Board {
    mid_price: Decimal,
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
}
