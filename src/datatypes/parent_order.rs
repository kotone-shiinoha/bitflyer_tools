use serde::{Serialize};

use crate::datatypes::{OrderMethod, TimeInForce, Parameter};

#[derive(Serialize)]
pub struct ParentOrder<'a> {
    order_method: OrderMethod,
    minute_to_expire: u64,
    time_in_force: TimeInForce,
    parameters: Vec<Parameter<'a>>
}