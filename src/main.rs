use bitflyer_tools::orderbook::OrderBook;

#[tokio::main]
fn main() {
    let bok = OrderBook::connect(&["XLM_JPY"]).await.unwrap();
    for i in bok.rx_result.try_iter() {
        let json: serde_json::Value = serde_json::from_slice(&i[..]).unwrap();
        println!("{}", serde_json::to_value(json));
    }
}