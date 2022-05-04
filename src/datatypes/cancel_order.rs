const PATH: &'static str = "/v1/me/cancelchildorder";

pub struct CancelOrder<'a> {
    product_code: &'a str,
    child_order_id: &'a str,
}

pub struct CancelOrderByAcceptanceId<'a> {
    product_code: &'a str,
    child_order_acceptance_id: &'a str,
}
