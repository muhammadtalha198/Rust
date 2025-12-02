// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn Display_person() {
    
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    // Debug print
    println!("{:?}", peter);
}


fn Display(){

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

}

fn main() {

    Display_person();
    Display();
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);

    // println!("{}", UnPrintable(3));    // ERROR
    // println!("{:?}", UnPrintable(3));   // ERROR

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Display`.
    #[derive(Debug)]
    struct DisplayPrintable(i32);  


    // println!("{}", DisplayPrintable(3));  // ERROR  
    println!("{:?}", DisplayPrintable(3));   

}

