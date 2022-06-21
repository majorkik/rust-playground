#![feature(test)]

extern crate test;

use std::collections::HashMap;

mod katas;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    
    Ok(())
}
