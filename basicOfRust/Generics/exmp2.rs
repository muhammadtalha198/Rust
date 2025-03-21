
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };;

    println!("integer.x = {}", both_integer.x);
    println!("float.y = {}", both_float.y);
    println!("integer_and_float.x = {}", integer_and_float.x);
}
