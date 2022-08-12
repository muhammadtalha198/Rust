use reqwest;

// tokio let's us use "async" on our main function
// #[tokio::main]
// async fn main() {
//     // chaining .await will yield our query result
//     let result = reqwest::get("http://127.0.0.1:5500/talha.html").await

//     println!("{:?}", result)
// }

fn main() {
    short_way_to_create_http_request();
}

async fn short_way_to_create_http_request() {

     let result = reqwest::get("https://api.spotify.com/v1/search").await

    println!("{:?}", result);

}

//////////////Method 2//////////////////////////////////////////////////////

// how to make http Get request

extern crate reqwest;

fn main() {
    short_way_to_create_http_request();
}

async fn short_way_to_create_http_request() {

    let response_text = reqwest::get("http://127.0.0.1:5500/talha.html")
        .await.expect("Couldent make request!")
        .text()
        .await.expect("Couldent read response text");

    println!("response text will be {}", response_text);

}


/////////////////////////////method 3//////////////////////////////////////////////////

// fn long_way_to_create_http_request() {

//     // match reqwest::get("http://127.0.0.1:5500/talha.html"){

//     //     Ok(mut response) => {

//     //         //check if 200 ok
//     //         if response.status() == reqwest :: StatusCode :: Ok {

//     //             match response.text(){

//     //                 Ok(text) => println!("Response Text: {}", text),
//     //                 Err(_) => println!("Could not read response text! ")
//     //             }
//     //         } else {
//     //             println!("Response was not 200 OK.");
//     //         }
//     //     }

//     //     Err(_) => println!("Could not make the request!");
//     // }

// }
