
use std::collections::HashMap;
use dotenv :: dotenv;
// use std :: env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

// async fn short_way_to_create_http_request() {

//     let body = reqwest::get("https://www.rust-lang.org")
//     .await?
//     .text()
//     .await?;

//     println!("body = {:?}", body);
//     return;
// }

