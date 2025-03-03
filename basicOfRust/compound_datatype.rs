// compound types
// arrays

fn arrays() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1; 5];
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];

    println!("arr: {:?}", arr);
    println!("arr2: {:?}", arr2);
    println!("fruits: {:?}", fruits);
}
// tuples
fn tuples() {
    let tuple: (i32, f64, char) = (1, 2.0, 'a');
    let tuple2: (i32, f64, char) = (11, 2.644, '4');
    let human: (&str, i32, i32) = ("John", 20, 170);
    let human0: (String, i32, i32) = ("John".to_string(), 20, 170);
    let human1 = ("John", 20, 170, true, [1, 2, 3, 4, 5]);

    println!("tuple: {:?}", tuple);
    println!("tuple2: {:?}", tuple2);
    println!("human: {:?}", human);
    println!("human0: {:?}", human0);
    println!("human1: {:?}", human1);
}
// slices
fn slices() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[0..2];
    let slice1: &[i32] = &[1, 2, 3, 4, 5];
    let slice2: &[i32] = &arr[0..3];
    let slice3: &[&str] = &["lion", "tiger", "bear"];
    let slice4: &[&String] = &[&"lion".to_string(), &"tiger".to_string(), &"bear".to_string()];

    println!("slice: {:?}", slice);
    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
    println!("slice3: {:?}", slice3);
    println!("slice4: {:?}", slice4);
}
// strings  
fn strings() {
    let str1: &str = "Hello, world!";
    let str2: String = "Hello, world!".to_string();
    let str3: &String = &str2;
    let mut str4: String  = String :: from("Hello, world!");
    str4.push_str(" why");
    str4.push('?');
    println!("str: {:?}", str1);
    println!("str2: {:?}", str2);
    println!("str3: {:?}", str3);
    println!("str4: {:?}", str4);
}   

fn stringSlice() {
    let str1: &str = "Hello, world!";
    let str2: &str = "Hello, world!";
    let str3: &str = "Hello, world!";
    let str4: &str = "Hello, world!";
    
}   

fn main() {
    arrays();
    tuples();
    slices();
    strings();
}
