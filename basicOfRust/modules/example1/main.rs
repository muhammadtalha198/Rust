
pub mod garden;
use garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {
        name: String::from("Asparagus"),
        quantity: 10,
    };
    println!("I'm growing {plant:?}!");
}
