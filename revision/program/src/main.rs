fn main() {
    // Single string prin
    println!("Hello, world!");

    ////number print in a place holder
    println!("Number will be: {}", 24);

    // Multiple place holder
    println!("{} is a company {}. ", "Educative", "Software");

    // positional Argument
    println!(
        "Enhance your coding skills {0} courses. {0} course are very {1}",
        "Educative", "interavctive"
    );

    // place holder for a debug trait
    let array: [i32; 4] = [1, 2, 3, 4];
}
