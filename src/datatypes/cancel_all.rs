use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrders<'a> {
    product_code: &'a str
}