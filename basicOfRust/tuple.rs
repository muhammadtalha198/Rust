

// tuple 

fn main() {
    let tuple_var = (1, 2, 3, 4, 5, "Hello", "c", true, (1.12, 23.22));
    println!("My data is {} {} {} ", tuple_var.0, tuple_var.1, tuple_var.2);
    println!("tuple_var: {:?}", tuple_var);

    let some_value = tuple_var.8.0;
    println!("some_value: {:?}", some_value);

    let rgb_color = get_some_rgb();
    println!("rgb_color: {:?}", rgb_color);

    let (r, g, b) = rgb_color;
    println!("r: {:?}, g: {:?}, b: {:?}", r, g, b);

    let some_other_color : (u8, u8, u8, u8) = (255, 0, 0, 255);
    println!("some_other_color: {:?}", some_other_color);
    
    let empty_tuple = ();
    println!("empty_tuple: {:?}", empty_tuple);

    match some_other_color {
        0..=255 =>  println!("blah"),
        _ => println!("not blah"),
    }

}

fn get_some_rgb () -> (u8, u8, u8) {
    (255, 0, 0)
}
