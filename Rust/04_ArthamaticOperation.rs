//Arthamatic operations in rust

fn main() {
    let a: u8 = 5;
    let b: u8 = 1;

    let x: f64 = 2.0;
    let y: f32 = 3.0;

    let sum: u8 = a + b;
    println!("The sum of a and b is: {}", sum);

    let difference: f64 = x - 1.0;
    println!("The difference of x and value is: {}", difference);

    let product: u8 = 4 * a;
    println!("The product of a and value is: {}", product);

    let quotient: f32 = 9.0 / y;
    println!("The division of value and y is: {}", quotient);

    let remainder: u8 = a % b;
    println!("The remainder of a and b is: {}", remainder);

    let t: bool = true;
    let f: bool = false;
}
