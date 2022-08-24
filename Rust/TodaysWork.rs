// fn main() {
//     //--------------OwnerShip Rules -----------
//     //1. Each value in Rust has a variable thats called its owner.
//     //2. There can only be one owner at a time.
//     //3. When the owner goes out of space, the value will be dropped.

//     {
//         let s: &str = "hello"; // s is valid from this point forward
//                                // create the string dynamically
//         let s1: String = String::from("hello"); //now this stored in heap
//         //do stuff with s
//     } // scope is over , s is not valid

// }

//--------------Assigning a value to another variable------------------

// fn main() {
//     let x: i32 = 5;
//     let y: i32 = x; //copy

//     let s1: String = String::from("Hello");
//     // let s2: String = s1; // Move (not shallow copy!) //error
//     let s3: String = s1.clone(); // Move (not shallow copy!) //error

//     println!("{}, world!", s1);
// }

//----------ownerShip and functions--------------------------
fn main() {
    let s: String = String::from("hello");

    takes_ownership(s);
    println!("{}", some_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}
