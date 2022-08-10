// if else in Rust

fn main() {
    // Example 1

    let a: i16 = 12;
    let b: i16 = 18;

    if a == b {
        println!("a == b");
    } else {
        println!("a != b")
    }

    // Example 2

    let price: i8 = 12;

    if price == 1 {
        println!("price is 1.");
    } else if price == 2 {
        println!("price is 2.");
    } else {
        println!("price is 12.");
    }

    // Example 3

    let formal: bool = true;

    let greeting: () = if formal {
        println!("Good Evening!");
    } else {
        println!("Good Night!");
    };
}
