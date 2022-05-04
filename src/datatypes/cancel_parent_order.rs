use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CancelParentOrder<'a> {
    product_code: &'a str,
    parent_order_id: &'a str,
}
