fn main() {

    //declaration of a variable and assigining a value to it.
    
    let number = 12;
    let number2;
    number2 = 4;

    let name1 = "Muhammad";
    let name2;
    name2 = "Talha";
    
    //print the values to those numbers.
    
    println!("{}", number);
    println!("{}", number2);
    println!("{}", name1);
    println!("{}", name2);

    println!("Second number will be {}", number2);
    println!("Second name will be {}", name2);
  

    //Data types in Rust

    let x: i32 = 2;
    println!("The value of x will be: {}", x);

    let y: bool = true;
    println!("The value of y : {}", y);

    const FINAL_NAME: u128 = 100;
    println!("My name would be: {}", FINAL_NAME);

    let number: i128 = 12345673456789345678345;
    println!("{}", number);

    //unsigned number

    let number1: u128 = 121543546464;
    println!("{}", number1);

    let number3: f64 = 12.773;
    println!("{}", number3);

    let value: bool = true;
    println!("{}", value);

    let name_: char = '@';
    println!("{}", name_);
}
