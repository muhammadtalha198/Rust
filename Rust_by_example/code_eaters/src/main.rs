

fn main() {
    println!("Hello, world!");

    let number:u8   = 39;
    println!("The number is: {}", number);

   // number = 40; // its an error because number is immutable

   let mut number_mutable: u8 = 50;
   println!("The mutable number is: {}", number_mutable);

   number_mutable = 60;
   println!("The mutable number is now: {}", number_mutable);


   //&str type

   let string_literal: &str = "This is a string literal";
    println!("{}", string_literal);

    // now have to convert it into string type

    let string_literal_to_string: String = string_literal.to_string();
    println!("{}", string_literal_to_string);

    let string_literal_to_string2: String = String::from("This is another string");
    println!("{}", string_literal_to_string2);

    let string_literal_to_string3: String = String::from(string_literal_to_string);
    println!("{}", string_literal_to_string3);

    let mut string_itself: String = String::new();
    string_itself.push_str("This is a new string");
    println!("{}", string_itself);








}
