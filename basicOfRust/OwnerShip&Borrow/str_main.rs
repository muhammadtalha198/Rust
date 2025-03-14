
fn main(){
    let some_string : String = String::from("Hello, world!");
    let some_str: &str = "Hello, Muhammad!";

    some_procedure(&some_string, some_str);
    println!("some_string: {}", some_string);
    println!("some_str: {}", some_str);
}

fn some_procedure(param_a: &String, param_b: &str){
    println!("param_a: {}", param_a);
    println!("param_b: {}", param_b);
}
