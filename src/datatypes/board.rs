use rust_decimal::Decimal;
use super::PriceLevel;

pub struct Board {
    mid_price: Decimal,
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>
}