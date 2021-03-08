extern crate reqwest;
use serde_json::Value;
use std::borrow::Borrow;

async fn test_punchthrough() {
    let response = reqwest::get("https://services.ler.dk/api/basicTest")
        .await
        .unwrap();
    if response.status().is_success() {
        let data = response.text().await.unwrap();
        // println!("Response: {}", data);
        let v: Value = serde_json::from_str(data.borrow()).unwrap();
        println!("Confirmation: {:?}", v["StatusCode"]);
    } else {
        println!("Bad request");
    }
}

#[tokio::main]
async fn main() {
    test_punchthrough().await;
}
