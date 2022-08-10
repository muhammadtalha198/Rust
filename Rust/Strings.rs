// Strings in rust

fn main() {
    let my_name: &str = "Muhammad Talha";
    println!("My name will be: {}", my_name);

    let mut your_name = String::new(); //when you create Strings object be like this it must be mutuable
    your_name.push_str("Quitter!");
    println!("your name will be: {}", your_name);

    let her_name = String::from("Josephine!");
    println!("her name will be: {}", her_name);
}
