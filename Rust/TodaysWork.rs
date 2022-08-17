extern crate reqwest;
// #[async_std::main]

// async fn main() {
//    let response_text = reqwest::get("http://127.0.0.1:5500/talha.html").await;
//         // .expect("couldn't make request!")
//         // .text()
//         // .expect("Couldn't read response!");

//     println!("Response text: {:?}", response_text);
// }


fn main() {
    let response_text = reqwest::get("http://127.0.0.1:5500/talha.html")
        .expect("couldn't make request!")
        .text()
        .expect("Couldn't read response!");

    println!("Response text: {:?}", response_text);
}




// fn main() {
//     http_request();
// }

// async fn http_request(){
    
//     let response_text = reqwest::get("http://127.0.0.1:5500/talha.html").await;
//         // .expect("couldn't make request!")
//         // .text()
//         // .expect("Couldn't read response!");

//     println!("Response text: {:?}", response_text);
// }
