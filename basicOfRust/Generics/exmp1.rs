
struct Point <T> {
    x: T,
    y: T,
}

fn main() {

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.3 };

    println!("integer.x = {}", integer.x);
    println!("float.y = {}", float.y);
}
